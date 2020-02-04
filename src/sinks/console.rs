use super::util::SinkExt;
use crate::{
    event::{self, Event, LogEvent},
    topology::config::{DataType, SinkConfig, SinkContext, SinkDescription},
};
use futures::{future, Sink};
use serde::{Deserialize, Serialize};
use tokio::{
    codec::{FramedWrite, LinesCodec},
    io,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Stdout,
    Stderr,
}

impl Default for Target {
    fn default() -> Self {
        Target::Stdout
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConsoleSinkConfig {
    #[serde(default)]
    pub target: Target,
    #[serde(deserialize_with = "from_encoding_config")]
    pub encoding: EncodingConfig,
}

use serde::de::{self, Deserializer, IntoDeserializer, MapAccess, Visitor};
use std::{
    fmt::{self, Debug},
    marker::PhantomData,
};
use string_cache::DefaultAtom as Atom;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct EncodingConfig {
    format: Encoding,

    // TODO: Consider a HashSet... But Vec is faster at small sizes!
    // TODO: Serde does not offer mutual exclusivity?
    #[serde(default)]
    only_fields: Option<Vec<Atom>>,
    // TODO: Serde does not offer mutual exclusivity?
    #[serde(default)]
    except_fields: Option<Vec<Atom>>,
}

impl EncodingConfig {
    pub const fn new(
        format: Encoding,
        only_fields: Option<Vec<Atom>>,
        except_fields: Option<Vec<Atom>>,
    ) -> Self {
        EncodingConfig {
            format,
            only_fields,
            except_fields,
        }
    }
    fn rework(&self, mut event: LogEvent) -> LogEvent {
        if let Some(ref fields) = self.only_fields {
            event.drain().filter(|(k, _v)| fields.contains(k)).collect()
        } else if let Some(ref fields) = self.except_fields {
            fields.iter().for_each(|f| {
                event.remove(f);
            });
            event
        } else {
            event
        }
    }
}

// Derived from https://serde.rs/string-or-struct.html
fn from_encoding_config<'de, D>(deserializer: D) -> Result<EncodingConfig, D::Error>
where
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct(PhantomData<fn() -> EncodingConfig>);

    impl<'de> Visitor<'de> for StringOrStruct {
        type Value = EncodingConfig;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(EncodingConfig {
                format: Encoding::deserialize(value.into_deserializer())?,
                only_fields: Default::default(),
                except_fields: Default::default(),
            })
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Encoding {
    Text,
    Json,
}

inventory::submit! {
    SinkDescription::new_without_default::<ConsoleSinkConfig>("console")
}

#[typetag::serde(name = "console")]
impl SinkConfig for ConsoleSinkConfig {
    fn build(&self, cx: SinkContext) -> crate::Result<(super::RouterSink, super::Healthcheck)> {
        let encoding = self.encoding.clone();

        let output: Box<dyn io::AsyncWrite + Send> = match self.target {
            Target::Stdout => Box::new(io::stdout()),
            Target::Stderr => Box::new(io::stderr()),
        };

        let sink = FramedWrite::new(output, LinesCodec::new())
            .stream_ack(cx.acker())
            .sink_map_err(|_| ())
            .with(move |event| encode_event(event, &encoding));

        Ok((Box::new(sink), Box::new(future::ok(()))))
    }

    fn input_type(&self) -> DataType {
        DataType::Any
    }

    fn sink_type(&self) -> &'static str {
        "console"
    }
}

fn encode_event(event: Event, encoding: &EncodingConfig) -> Result<String, ()> {
    match event {
        Event::Log(log) => {
            println!("Before: {:?}", log);
            let log = encoding.rework(log);
            println!("After: {:?}", log);
            match encoding.format {
                Encoding::Json => serde_json::to_string(&log.unflatten())
                    .map_err(|e| panic!("Error encoding: {}", e)),
                Encoding::Text => {
                    let s = log
                        .get(&event::MESSAGE)
                        .map(|v| v.to_string_lossy())
                        .unwrap_or_else(|| "".into());
                    Ok(s)
                }
            }
        }
        Event::Metric(metric) => serde_json::to_string(&metric).map_err(|_| ()),
    }
}

#[cfg(test)]
mod test {
    use super::{encode_event, Encoding, EncodingConfig};
    use crate::event::metric::{Metric, MetricKind, MetricValue};
    use crate::event::Event;
    use chrono::{offset::TimeZone, Utc};
    const DEFAULT_ENCODING_CONFIG: EncodingConfig = EncodingConfig::new(Encoding::Text, None, None);

    #[test]
    fn encodes_raw_logs() {
        let event = Event::from("foo");
        assert_eq!(
            Ok("foo".to_string()),
            encode_event(event, &DEFAULT_ENCODING_CONFIG)
        );
    }

    #[test]
    fn encodes_counter() {
        let event = Event::Metric(Metric {
            name: "foos".into(),
            timestamp: Some(Utc.ymd(2018, 11, 14).and_hms_nano(8, 9, 10, 11)),
            tags: Some(
                vec![("key".to_owned(), "value".to_owned())]
                    .into_iter()
                    .collect(),
            ),
            kind: MetricKind::Incremental,
            value: MetricValue::Counter { value: 100.0 },
        });
        assert_eq!(
            Ok(r#"{"name":"foos","timestamp":"2018-11-14T08:09:10.000000011Z","tags":{"key":"value"},"kind":"incremental","value":{"type":"counter","value":100.0}}"#.to_string()),
            encode_event(event, &DEFAULT_ENCODING_CONFIG)
        );
    }

    #[test]
    fn encodes_set() {
        let event = Event::Metric(Metric {
            name: "users".into(),
            timestamp: None,
            tags: None,
            kind: MetricKind::Incremental,
            value: MetricValue::Set {
                values: vec!["bob".into()].into_iter().collect(),
            },
        });
        assert_eq!(
            Ok(r#"{"name":"users","timestamp":null,"tags":null,"kind":"incremental","value":{"type":"set","values":["bob"]}}"#.to_string()),
            encode_event(event, &DEFAULT_ENCODING_CONFIG)
        );
    }

    #[test]
    fn encodes_histogram_without_timestamp() {
        let event = Event::Metric(Metric {
            name: "glork".into(),
            timestamp: None,
            tags: None,
            kind: MetricKind::Incremental,
            value: MetricValue::Distribution {
                values: vec![10.0],
                sample_rates: vec![1],
            },
        });
        assert_eq!(
            Ok(r#"{"name":"glork","timestamp":null,"tags":null,"kind":"incremental","value":{"type":"distribution","values":[10.0],"sample_rates":[1]}}"#.to_string()),
            encode_event(event, &DEFAULT_ENCODING_CONFIG)
        );
    }
}
