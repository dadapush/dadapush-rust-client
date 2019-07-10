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
pub struct MessagePushRequest {
    /// action size range is 0,3
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<::models::Action>>,
    /// message content
    #[serde(rename = "content")]
    pub content: String,
    /// when value is false, will not send client push
    #[serde(rename = "needPush")]
    pub need_push: bool,
    /// message title
    #[serde(rename = "title")]
    pub title: String,
}

impl MessagePushRequest {
    pub fn new(content: String, need_push: bool, title: String) -> MessagePushRequest {
        MessagePushRequest {
            actions: None,
            content: content,
            need_push: need_push,
            title: title,
        }
    }
}

