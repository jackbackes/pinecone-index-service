/*
 * defaultTitle
 *
 * defaultDescription
 *
 * The version of the OpenAPI document: 0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    /// The name of the collection to be created.
    #[serde(rename = "name")]
    pub name: String,
    /// The name of the source index to be used as the source for the collection.
    #[serde(rename = "source")]
    pub source: String,
}

impl CreateCollectionRequest {
    pub fn new(name: String, source: String) -> CreateCollectionRequest {
        CreateCollectionRequest {
            name,
            source,
        }
    }
}


