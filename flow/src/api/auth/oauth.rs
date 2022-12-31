/*
    Appellation: oauth <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use fluidity::JsResult;
use gloo::net::http::{Method, Request};

use wasm_bindgen::prelude::*;

use serde_json::Value;
use std::collections::HashMap;

pub struct OAuth {
    pub client_id: String,
    secret_key: String,
    endpoint: String,
    pub scope: String,
}

impl OAuth {
    pub fn request(&self) -> Request {
        Request::new(self.endpoint.as_str()).method(Method::POST)
    }
}

#[wasm_bindgen]
pub async fn get_access_token(
    client_id: &str,
    client_secret: &str,
    auth_url: &str,
    scope: &str,
) -> JsResult<String> {
    // Set up the request body
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("scope", scope);

    let resp = Request::new(auth_url).method(Method::POST).send().await;
    // Check the response status
    if resp.is_ok() {
        // Parse the response body
        let value: Value = resp?.json().await?;
        let access_token = value["access_token"].as_str().unwrap();
        Ok(access_token.to_string())
    } else {
        Ok("Error".to_string())
    }
}
