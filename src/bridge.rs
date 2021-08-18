use std::collections::HashMap;

use futures::future::try_join_all;
use url::Url;

use crate::{
    mockserver::{
        model::{Expectation, SearchFilter},
        Client,
    },
    ohmymock::{Data, Processor},
};

pub struct Bridge {
    mockserver_cli: Client,
    ohmymock_proc: Processor,
}

impl Bridge {
    pub fn new(cli: Client, proc: Processor) -> Self {
        Bridge {
            mockserver_cli: cli,
            ohmymock_proc: proc,
        }
    }

    pub async fn create_expectation(&self, filename: &str) -> Result<(), String> {
        let mocks = self.ohmymock_proc.get_mocks(filename)?;
        if mocks.is_empty() {
            return Err("expectations not found".to_string());
        }

        let mut create_futures = vec![];
        mocks
            .iter()
            .map(|m| self.mock_to_expectation(m))
            .for_each(|e| {
                let f = self.mockserver_cli.create_expectations(e.unwrap());
                create_futures.push(f);
            });

        if let Err(e) = try_join_all(create_futures).await {
            return Err(e.to_string());
        }

        Ok(())
    }

    pub async fn search_expectations(
        &self,
        filter: Option<&SearchFilter>,
    ) -> Result<Vec<Expectation>, String> {
        self.mockserver_cli.retrieve_expectations(filter).await
    }

    fn mock_to_expectation(&self, d: &Data) -> Result<Expectation, String> {
        let active_mock_id = d.active_mock.as_str();
        let active_mock = d.mocks.get(active_mock_id).unwrap();

        let mut expectation = Expectation::default();
        expectation.id = d.id.clone();
        expectation.priority = 0;
        expectation.http_request.method = d.method.clone();
        expectation.http_request.path = {
            let url_str = d.url.replace("\\.", ".");
            let url = Url::parse(&url_str).map_err(|_| "url invalida".to_string())?;
            url.path().into()
        };
        expectation.http_response.body = active_mock.response.clone();
        expectation.http_response.headers = {
            let mut headers: HashMap<String, Vec<String>> = HashMap::new();
            for (k, v) in active_mock.headers.as_ref().unwrap().iter() {
                headers.insert(k.to_string(), vec![v.to_string()]);
            }
            Some(headers)
        };

        Ok(expectation)
    }
}
