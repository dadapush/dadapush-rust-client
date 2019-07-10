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
pub struct MessageObject {
    /// action size range is 0,3
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<::models::Action>>,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "title")]
    pub title: String,
}

impl MessageObject {
    pub fn new(channel_name: String, content: String, created_at: String, id: i64, title: String) -> MessageObject {
        MessageObject {
            actions: None,
            channel_name: channel_name,
            content: content,
            created_at: created_at,
            id: id,
            title: title,
        }
    }
}

