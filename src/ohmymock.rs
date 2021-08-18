use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub url: String,

    pub id: String,

    pub method: String,

    #[serde(rename = "type")]
    pub data_type: String,

    pub mocks: HashMap<String, Mock>,

    #[serde(rename = "activeMock")]
    pub active_mock: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mock {
    pub delay: Option<i32>,

    #[serde(rename = "headers")]
    pub headers: Option<HashMap<String, String>>,

    pub id: Option<String>,

    #[serde(rename = "createdOn")]
    pub created_on: Option<String>,

    #[serde(rename = "statusCode")]
    pub status_code: Option<i32>,

    pub response: Option<String>,

    #[serde(rename = "responseMock")]
    pub response_mock: Option<String>,

    #[serde(rename = "headersMock")]
    pub headers_mock: Option<HashMap<String, String>>,

    #[serde(rename = "type")]
    pub mock_type: Option<String>,

    #[serde(rename = "subType")]
    pub sub_type: Option<String>,
}

pub struct Processor;

impl Processor {
    pub fn new() -> Self {
        Processor {}
    }

    pub fn get_mocks(&self, filename: &str) -> Result<Vec<Data>, String> {
        let mut file = File::open(filename).map_err(|e| e.to_string())?;

        let mut data = String::new();
        file.read_to_string(&mut data).map_err(|e| e.to_string())?;

        let config_data: Configuration = serde_json::from_str(&data).map_err(|e| e.to_string())?;

        Ok(config_data.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_expected_object() -> Vec<Data> {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let mut mocks = HashMap::new();
        mocks.insert(
            "x1dQmNzZUv".to_string(),
            Mock {
                delay: Some(0),
                headers: Some(headers.clone()),
                id: Some("x1dQmNzZUv".into()),
                created_on: Some("2021-07-14T13:00:26.387Z".into()),
                status_code: Some(200),
                response: Some("{\"result\":{\"is_ok\":true,\"description\":\"ok\"}}".into()),
                response_mock: Some("{\"result\":{\"is_ok\":true,\"description\":\"ok\"}}".into()),
                headers_mock: Some(headers),
                mock_type: Some("application".into()),
                sub_type: Some("json".into()),
            },
        );

        let data = Data {
            url: "https://domain\\.cl/api/sample".into(),
            id: "i1zc8pkfyQ".into(),
            method: "POST".into(),
            data_type: "XHR".into(),
            mocks: mocks,
            active_mock: "x1dQmNzZUv".into(),
        };

        vec![data]
    }

    #[test]
    fn test_get_mocks_ok() {
        let filename = "mock.json";
        let expected = get_expected_object();
        let subject = Processor::new();

        let retorned = match subject.get_mocks(filename) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };
        
        assert_eq!(expected.len(), retorned.len());
        
        let retorned_data = retorned.first().unwrap();
        let expected_data = expected.first().unwrap();

        assert_eq!(expected_data.url, retorned_data.url);
        assert_eq!(expected_data.id, retorned_data.id);
        assert_eq!(expected_data.active_mock, retorned_data.active_mock);
        assert_eq!(expected_data.method, retorned_data.method);

        assert_eq!(expected_data.mocks.len(), retorned_data.mocks.len());
    }
}
