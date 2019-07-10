/*
 * DaDaPush Public API
 *
 * DaDaPush: Real-time Notifications App Send real-time notifications through our API without coding and maintaining your own app for iOS or Android devices.
 *
 * The version of the OpenAPI document: v1
 * Contact: contacts@dadapush.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(rename = "errmsg")]
    pub errmsg: String,
}

impl Result {
    pub fn new(code: i32, errmsg: String) -> Result {
        Result {
            code: code,
            data: None,
            errmsg: errmsg,
        }
    }
}

