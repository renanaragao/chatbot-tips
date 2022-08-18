use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub ok: bool,
    pub result: Vec<Update>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    #[serde(rename = "update_id")]
    pub update_id: i64,
    pub message: Message,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "message_id")]
    pub message_id: i64,
    pub from: From,
    pub chat: Chat,
    pub date: i64,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From {
    pub id: i64,
    #[serde(rename = "is_bot")]
    pub is_bot: bool,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "language_code")]
    pub language_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

pub struct UriTelegram {
    pub host: String,
    pub token: String,
    pub scheme: String,
    pub port: u16,
}

impl UriTelegram {
    pub fn new<'a>(host: String, token: String, scheme: String, port: u16) -> Self {
        Self {
            host,
            token,
            scheme,
            port: port,
        }
    }
}
