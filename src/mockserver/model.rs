use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Expectation {
    pub id: String,

    pub priority: i32,

    #[serde(rename = "httpRequest")]
    pub http_request: HttpRequest,

    #[serde(rename = "httpResponse")]
    pub http_response: HttpResponse,
}

impl Default for Expectation {
    fn default() -> Self {
        Self {
            id: "".into(),
            priority: 0,
            http_request: HttpRequest {
                method: "GET".into(),
                path: "".into(),
                path_parameters: None,
                query_string_parameters: None,
                body: None,
            },
            http_response: HttpResponse {
                delay: None,
                body: None,
                cookies: None,
                headers: None,
            },
        }
    }
}

impl Display for Expectation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json_txt = serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", json_txt)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpRequest {
    pub method: String,

    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pathParameters")]
    pub path_parameters: Option<HashMap<String, Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: Option<HashMap<String, Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<Delay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "base64Bytes")]
    pub base64_bytes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Delay {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeUnit")]
    pub time_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchFilter {
    pub method: String,
    pub path: String,
}
