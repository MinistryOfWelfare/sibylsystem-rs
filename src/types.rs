// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct BanResult {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "result")]
    result: Result,

    #[serde(rename = "error")]
    error: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "previous_ban")]
    previous_ban: Ban,

    #[serde(rename = "current_ban")]
    current_ban: Ban,
}

#[derive(Serialize, Deserialize)]
pub struct Ban {
    #[serde(rename = "user_id")]
    user_id: i64,

    #[serde(rename = "banned")]
    banned: bool,

    #[serde(rename = "reason")]
    reason: String,

    #[serde(rename = "message")]
    message: String,

    #[serde(rename = "ban_source_url")]
    ban_source_url: String,

    #[serde(rename = "banned_by")]
    banned_by: i64,

    #[serde(rename = "crime_coefficient")]
    crime_coefficient: i64,

    #[serde(rename = "date")]
    date: String,

    #[serde(rename = "ban_flags")]
    ban_flags: Vec<String>,
}
