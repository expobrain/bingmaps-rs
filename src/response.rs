use serde;

#[derive(Deserialize)]
pub struct Response<T: serde::Deserialize> {
    // #[serde(rename = "authenticationResultCode")]
    // pub authentication_result: String,

    // #[serde(rename = "brandLogoUri")]
    // pub brand_logo_uri: String,

    // #[serde(rename = "copyright")]
    // pub copyright: String,

    // #[serde(rename = "errorDetails")]
    // pub error_details: Vec<String>,

    // #[serde(rename = "traceId")]
    // pub trace_id: String,

    #[serde(rename = "resourceSets")]
    pub resource_sets: Vec<ResourceSet<T>>,
}

#[derive(Deserialize)]
pub struct ResourceSet<T: serde::Deserialize> {
    // #[serde(rename = "estimatedTotal")]
    // pub estimated_total: i64,

    #[serde(rename = "resources")]
    pub resources: Vec<T>,
}
