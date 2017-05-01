use std::collections::BTreeMap;

/// top level document
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Spec {
    /// version string
    pub swagger: String,
    pub info: Info,
    pub paths: BTreeMap<String, Operations>, // / -> get -> op
    pub definitions: BTreeMap<String, Schema>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemes: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="basePath")]
    pub base_path: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub consumes: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub produces: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<BTreeMap<String, Parameter>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub responses: Option<BTreeMap<String, Response>>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="securityDefinitions")]
    pub security_definitions: Option<BTreeMap<String, Security>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="externalDocs")]
    pub external_docs: Option<Vec<ExternalDoc>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalDoc {
    pub url: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Info {
    pub title: String,
    pub version: String,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="termsOfService")]
    pub terms_of_service: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Operations {
    #[serde(skip_serializing_if="Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Operation {
    #[serde(skip_serializing_if="Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub consumes: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub produces: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemes: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    pub responses: BTreeMap<String, Response>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    #[serde(rename="in")]
    pub location: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="uniqueItems")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="type")]
    pub param_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Response {
    pub description: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema: Option<Schema>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Security {
    #[serde(rename="apiKey")]
    ApiKey {
        name: String,
        #[serde(rename="in")]
        location: String,
    },
    #[serde(rename="oauth2")]
    Oauth2 {
        flow: String,
        #[serde(rename="authorizationUrl")]
        authorization_url: String,
        #[serde(rename="tokenUrl")]
        #[serde(skip_serializing_if="Option::is_none")]
        token_url: Option<String>,
        scopes: BTreeMap<String, String>,
    },
    #[serde(rename="basic")]
    Basic,
}

/// A [JSON schema](http://json-schema.org/) definition describing
/// the shape and properties of an object.
///
/// This may also contain a `$ref` to another definition
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Schema {
    #[serde(skip_serializing_if="Option::is_none")]
    /// a [JSON reference](https://tools.ietf.org/html/draft-pbryan-zyp-json-ref-03)
    /// path to another defintion
    #[serde(rename="$ref")]
    pub ref_path: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="type")]
    pub schema_type: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename="enum")]
    pub enum_values: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<Box<Schema>>, // if scheme_type array (box is for recursion)
    #[serde(skip_serializing_if="Option::is_none")]
    pub properties: Option<BTreeMap<String, Schema>>, // implies object
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::BTreeMap;

    #[test]
    fn security_api_deserializes() {
        let json = r#"{"type":"apiKey", "name":"foo", "in": "query"}"#;
        assert_eq!(
            serde_json::from_str::<Security>(&json).unwrap(),
            Security::ApiKey {
                name: "foo".into(),
                location: "query".into(),
            }
        );
    }

    #[test]
    fn security_api_serializes() {
        let json = r#"{"type":"apiKey","name":"foo","in":"query"}"#;
        assert_eq!(
            serde_json::to_string(
                &Security::ApiKey {
                     name: "foo".into(),
                     location: "query".into(),
                 },
            )
                    .unwrap(),
            json
        );
    }

    #[test]
    fn security_basic_deserializes() {
        let json = r#"{"type":"basic"}"#;
        assert_eq!(
            serde_json::from_str::<Security>(&json).unwrap(),
            Security::Basic
        );
    }

    #[test]
    fn security_basic_serializes() {
        let json = r#"{"type":"basic"}"#;
        assert_eq!(json, serde_json::to_string(&Security::Basic).unwrap());
    }

    #[test]
    fn security_oauth_deserializes() {
        let json = r#"{"type":"oauth2","flow":"implicit","authorizationUrl":"foo/bar","scopes":{"foo":"bar"}}"#;
        let mut scopes = BTreeMap::new();
        scopes.insert("foo".into(), "bar".into());
        assert_eq!(
            serde_json::from_str::<Security>(&json).unwrap(),
            Security::Oauth2 {
                flow: "implicit".into(),
                authorization_url: "foo/bar".into(),
                token_url: None,
                scopes: scopes,
            }
        );
    }

    #[test]
    fn security_oauth_serializes() {
        let json = r#"{"type":"oauth2","flow":"implicit","authorizationUrl":"foo/bar","scopes":{"foo":"bar"}}"#;
        let mut scopes = BTreeMap::new();
        scopes.insert("foo".into(), "bar".into());
        assert_eq!(
            json,
            serde_json::to_string(
                &Security::Oauth2 {
                     flow: "implicit".into(),
                     authorization_url: "foo/bar".into(),
                     token_url: None,
                     scopes: scopes,
                 },
            )
                    .unwrap()
        );
    }
}
