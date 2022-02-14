extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BanResult {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "result")]
    result: Option<Result>,

    #[serde(rename = "error")]
    error: Option<Error>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReportResult {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "result")]
    result: Option<String>,

    #[serde(rename = "error")]
    error: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
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

    #[serde(rename = "target_type")]
    target_type: i64,

    #[serde(rename = "crime_coefficient")]
    crime_coefficient: i64,

    #[serde(rename = "date")]
    date: String,

    #[serde(rename = "source_group")]
    source_group: String,

    #[serde(rename = "hue_color")]
    hue_color: String,

    #[serde(rename = "ban_flags")]
    ban_flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    #[serde(rename = "code")]
    code: i64,

    #[serde(rename = "message")]
    message: String,

    #[serde(rename = "origin")]
    origin: String,

    #[serde(rename = "date")]
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatsResult {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "result")]
    result: HashMap<String, i64>,

    #[serde(rename = "error")]
    error: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllBansResult {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "result")]
    result: Option<BannedUserResult>,

    #[serde(rename = "error")]
    error: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BannedUserResult {
    #[serde(rename = "users")]
    users: Vec<Option<Result>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGeneralInfoResult {
    #[serde(rename = "success")]
    success: bool,

    #[serde(rename = "result")]
    result: GenResult,

    #[serde(rename = "error")]
    error: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenResult {
    #[serde(rename = "user_id")]
    user_id: i64,

    #[serde(rename = "division")]
    division: i64,

    #[serde(rename = "assigned_by")]
    assigned_by: i64,

    #[serde(rename = "assigned_reason")]
    assigned_reason: String,

    #[serde(rename = "assigned_at")]
    assigned_at: String,

    #[serde(rename = "permission")]
    permission: i64,
}
