use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WebhookResponse {
    pub data: WebhookEventResource,
}

#[derive(Serialize, Deserialize)]
pub struct WebhookEventResource {
    #[serde(rename = "type")]
    pub data_type: String,
    pub id: String,
    pub attributes: Attributes,
    pub relationships: Relationships,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Relationships {
    pub webhook: RelationshipResource,
    pub transaction: Option<RelationshipResource>,
}

#[derive(Serialize, Deserialize)]
pub struct RelationshipResource {
    pub data: ResourceData,
    pub links: Links,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceData {
    #[serde(rename = "type")]
    pub data_type: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    pub related: String,
}
