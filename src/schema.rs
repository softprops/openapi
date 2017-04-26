use std::collections::BTreeMap;

/// top level document
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Spec {
    /// version string
    pub swagger: String,
    pub info: Info,
    pub paths: BTreeMap<String, Operations>, // / -> get -> op
    pub definitions: BTreeMap<String, Schema>,
    pub schemes: Option<Vec<String>>,
    pub host: Option<String>,
    #[serde(rename="basePath")]
    pub base_path: Option<String>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub parameters: Option<BTreeMap<String, Parameter>>,
    pub responses: Option<BTreeMap<String, Response>>,
    #[serde(rename="securityDefinitions")]
    pub security_definitions: Option<BTreeMap<String, Security>>,
    pub tags: Option<Vec<Tag>>,
}


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename="externalDocs")]
    pub external_docs: Option<Vec<ExternalDoc>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalDoc {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Operations {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub patch: Option<Operation>,
    pub delete: Option<Operation>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Operation {
    pub description: Option<String>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub schemes: Vec<String>,
    pub tags: Vec<String>,
    #[serde(rename="operationId")]
    pub operation_id: Option<String>,
    pub responses: BTreeMap<String, Response>,
    pub parameters: Option<Vec<Parameter>>,
}


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Parameter {
    pub name: String,
    #[serde(rename="in")]
    pub location: String,
    pub required: Option<bool>,
    pub schema: Option<Schema>,
    #[serde(rename="uniqueItems")]
    pub unique_items: Option<bool>,
    #[serde(rename="type")]
    pub param_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Response {
    pub description: String,
    pub schema: Option<Schema>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Security {
    #[serde(rename="type")]
    pub security_type: String, // todo:
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Schema {
    #[serde(rename="$ref")]
    pub ref_path: Option<String>,
    pub description: Option<String>,
    #[serde(rename="type")]
    pub schema_type: Option<String>,
    pub items: Option<Box<Schema>>, // if scheme_type array (box is for recursion)
    pub properties: Option<BTreeMap<String, Schema>>, // implies object
}
