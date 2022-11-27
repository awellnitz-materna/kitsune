use super::BaseObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub name: Option<String>,
    pub preferred_username: String,
    pub subject: Option<String>,
    #[serde(flatten)]
    pub rest: BaseObject,
    pub public_key: PublicKey,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub subject: Option<String>,
    pub content: String,
    #[serde(flatten)]
    pub rest: BaseObject,
}
