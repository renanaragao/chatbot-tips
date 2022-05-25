use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: String,
    pub language_code: String,
}