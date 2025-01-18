use reqwest::{Client as ReqwestClient, Method, RequestBuilder};
use serde::Serialize;
use serde_json::Result as SerdeJsonResult;
use std::{collections::HashMap, error::Error};

use crate::Config;

pub struct GqlClient<'a> {
    core_client: ReqwestClient,
    endpoint: &'a str,
}

#[derive(Serialize)]
struct GqlRequestBody<'a> {
    query: &'a str,
    variables: HashMap<String, String>,
}

impl<'a> GqlClient<'a> {
    pub fn from_config(config: &'a Config) -> Result<Self, Box<dyn Error>> {
        let builder = ReqwestClient::builder();
        let rwc = builder.default_headers(config.to_header_map()?).build()?;
        Ok(Self {
            core_client: rwc,
            endpoint: &config.endpoint,
        })
    }

    pub fn build_request(&self, query: &'a str) -> SerdeJsonResult<RequestBuilder> {
        let request_body = GqlRequestBody {
            query,
            variables: HashMap::new(),
        };
        let builder = self.core_client.request(Method::POST, self.endpoint);
        Ok(builder.body(serde_json::to_string(&request_body)?))
    }

    // fn execute_query(&self) {
    //     self.core_client.post
    // }
}
