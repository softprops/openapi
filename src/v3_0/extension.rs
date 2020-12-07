use std::collections::HashMap;
use std::fmt;

use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Contains openapi specification extensions
/// see https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Extensions(HashMap<String, serde_json::Value>);

impl Extensions {
    fn add(&mut self, ext_id: String, value: serde_json::Value) {
        self.0.insert(ext_id, value);
    }

    /// Fetch extension by name
    pub fn get(&self, ext_id: &str) -> Option<&serde_json::Value> {
        self.0.get(ext_id)
    }

    /// A reference to all the captured extensions
    pub fn all(&self) -> &HashMap<String, serde_json::Value> {
        &self.0
    }
}

impl Default for Extensions {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<'de> Deserialize<'de> for Extensions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExtensionsVisitor;
        impl<'de> Visitor<'de> for ExtensionsVisitor {
            type Value = Extensions;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Extensions")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Extensions, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut extensions = Extensions::default();
                while let Some(key) = map.next_key::<String>()? {
                    if key.starts_with("x-") {
                        extensions.add(key, map.next_value()?);
                    }
                }
                Ok(extensions)
            }
        }
        deserializer.deserialize_map(ExtensionsVisitor)
    }
}

impl Serialize for Extensions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.0.clone() {
            map.serialize_entry(&k, &v)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_test::{assert_tokens, Token};

    use crate::v3_0::extension::Extensions;

    #[test]
    fn test_serde_extensions() {
        let mut extensions = Extensions::default();
        extensions.add(String::from("x-test"), Value::from("val"));
        assert_tokens(
            &extensions,
            &[
                Token::Map { len: Some(1) },
                Token::String("x-test"),
                Token::String("val"),
                Token::MapEnd,
            ],
        )
    }

    #[test]
    fn test_get_extension() {
        let value = Value::from("val");

        let mut extensions = Extensions::default();
        extensions.add(String::from("x-test"), value.clone());

        assert_eq!(extensions.get("x-test"), Some(&value));
    }

    #[test]
    fn test_all_extensions() {
        let value = Value::from("val");

        let mut extensions = Extensions::default();
        extensions.add(String::from("x-test"), value.clone());

        assert_eq!(
            extensions.all().get_key_value("x-test"),
            Some((&"x-test".to_string(), &value))
        );
    }
}
