use crate::event::LogEvent;
use serde::de::{self, DeserializeOwned, Deserializer, IntoDeserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug},
    marker::PhantomData,
};
use string_cache::DefaultAtom as Atom;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct EncodingConfig<F> {
    pub(crate) format: F,

    // TODO: Consider a HashSet... But Vec is faster at small sizes!
    // TODO: Serde does not offer mutual exclusivity?
    #[serde(default)]
    pub(crate) only_fields: Option<Vec<Atom>>,
    // TODO: Serde does not offer mutual exclusivity?
    #[serde(default)]
    pub(crate) except_fields: Option<Vec<Atom>>,
}

impl<F> EncodingConfig<F>
where
    F: DeserializeOwned + Serialize + Debug + Eq + PartialEq + Clone,
{
    pub(crate) fn new(
        format: F,
        only_fields: Option<Vec<Atom>>,
        except_fields: Option<Vec<Atom>>,
    ) -> Self {
        EncodingConfig {
            format,
            only_fields,
            except_fields,
        }
    }
    pub(crate) fn rework(&self, mut event: LogEvent) -> LogEvent {
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
pub(crate) fn from_encoding_config<'de, F, D>(
    deserializer: D,
) -> Result<EncodingConfig<F>, D::Error>
where
    F: DeserializeOwned + Serialize + Debug + Eq + PartialEq + Clone,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<J: DeserializeOwned + Serialize + Debug + Eq + PartialEq + Clone>(
        PhantomData<fn() -> EncodingConfig<J>>,
    );

    impl<'de, J> Visitor<'de> for StringOrStruct<J>
    where
        J: DeserializeOwned + Serialize + Debug + Eq + PartialEq + Clone,
    {
        type Value = EncodingConfig<J>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(EncodingConfig {
                format: J::deserialize(value.into_deserializer())?,
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
