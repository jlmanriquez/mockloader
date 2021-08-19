use reqwest::StatusCode;

use self::model::{Expectation, SearchFilter};

pub mod model;

pub struct Client {
    host: String,
}

impl Client {
    pub fn new(host: &str) -> Self {
        Client { host: host.into() }
    }

    pub async fn create_expectations(&self, exp: Expectation) -> Result<(), String> {
        let mockserver_url = format!("{}/mockserver/expectation", self.host);
        let client = reqwest::Client::new();
        let resp = client
            .put(&mockserver_url)
            .json(&exp)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        match resp.status() {
            StatusCode::CREATED => {
                println!("✅ expectation id {} created successfully", exp.id);
                Ok(())
            }
            StatusCode::BAD_REQUEST => Err("incorrect request format".into()),
            _ => Err("invalid expectation".into()),
        }
    }

    pub async fn retrieve_expectations(
        &self,
        filter: Option<&SearchFilter>,
    ) -> Result<Vec<Expectation>, String> {
        let mockserver_url = format!("{}/mockserver/retrieve?format=json&type=active_expectations", self.host);
        let client = reqwest::Client::new().put(mockserver_url);

        let resp = match filter {
            Some(f) => {
                client
                    .json(f)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?
            },
            None => {
                client
                .send()
                .await
                .map_err(|e| e.to_string())?
            },
        };

        match resp.status() {
            StatusCode::OK => {
                let expectations = resp.json::<Vec<Expectation>>().await.map_err(|e| e.to_string())?;
                Ok(expectations)
            },
            StatusCode::NOT_ACCEPTABLE => Err("invalid expectation".into()),
            _ => Err("internal server error".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{*, model::{HttpRequest, HttpResponse}};

    #[tokio::test]
    async fn test_retrieve_expectations_empty() {
        let filter: Option<&SearchFilter> = None;
        let target = Client::new("http://localhost:1080");
        
        let resp = target
            .retrieve_expectations(filter)
            .await;

        assert_eq!(resp.is_ok(), true);
        assert_eq!(resp.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_create_expectation_ok() {
        let exp = Expectation {
            id: "i1zc8pkfyQ".into(),
            priority: 0,
            http_request: HttpRequest {
                method: "POST".into(),
                path: "/api/sample".into(),
                path_parameters: None,
                query_string_parameters: None,
                body: None,                
            },
            http_response: HttpResponse {
                delay: None,
                body: Some("{\"result\":{\"is_ok\":true,\"description\":\"ok\"}}".into()),
                cookies: None,
                headers: {
                    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
                    headers.insert("content-type".into(), vec!["application/json".to_string()]);
                    Some(headers)
                },
            },
        };

        let client = Client::new("http://localhost:1080");
        let resp = client
            .create_expectations(exp)
            .await;

        assert_eq!(resp.is_ok(), true);
    }

    #[tokio::test]
    async fn test_retrieve_expectations() {
        let filter: Option<&SearchFilter> = None;
        let target = Client::new("http://localhost:1080");
        
        let resp = target
            .retrieve_expectations(filter)
            .await;

        assert_eq!(resp.is_ok(), true);
        assert_eq!(resp.unwrap().len() > 0, true);
    }
}
