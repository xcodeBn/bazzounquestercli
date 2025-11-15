//! HTTP client for executing requests

use crate::error::Result;
use crate::http::request::RequestBuilder;
use crate::http::response::HttpResponse;
use crate::upload::MultipartBuilder;
use reqwest::blocking::Client;
use std::time::Instant;

/// HTTP client for making requests
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Execute a request and return the response
    pub fn execute(&self, request: &RequestBuilder) -> Result<HttpResponse> {
        let start = Instant::now();

        // Apply authentication first (modifies headers/query params)
        let mut headers = request.headers.clone();
        let mut query_params = request.query_params.clone();
        request.apply_auth(&mut headers, &mut query_params);

        // Create a temporary request with auth applied
        let mut auth_request = request.clone();
        auth_request.headers = headers;
        auth_request.query_params = query_params;

        // Parse headers and query params
        let header_map = auth_request.parse_headers()?;
        let query_map = auth_request.parse_query_params()?;

        // Build request
        let mut req = match request.method {
            crate::http::HttpMethod::Get => self.client.get(&request.url),
            crate::http::HttpMethod::Post => self.client.post(&request.url),
            crate::http::HttpMethod::Put => self.client.put(&request.url),
            crate::http::HttpMethod::Delete => self.client.delete(&request.url),
            crate::http::HttpMethod::Patch => self.client.patch(&request.url),
            crate::http::HttpMethod::Head => self.client.head(&request.url),
            crate::http::HttpMethod::Options => {
                self.client.request(reqwest::Method::OPTIONS, &request.url)
            }
        };

        // Add headers
        req = req.headers(header_map);

        // Add query parameters
        if !query_map.is_empty() {
            req = req.query(&query_map);
        }

        // Add form data if present (takes precedence over body)
        if let Some(form_data) = request.get_form_data() {
            if form_data.has_files() {
                // Use multipart/form-data for files
                let multipart_builder = MultipartBuilder::from_form_data(form_data)?;
                let multipart_body = multipart_builder.build()?;
                let content_type = multipart_builder.content_type();

                req = req.header(reqwest::header::CONTENT_TYPE, content_type)
                    .body(multipart_body);
            } else {
                // Use application/x-www-form-urlencoded for text-only forms
                let encoded = form_data.to_urlencoded();
                req = req.header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(encoded);
            }
        } else if let Some(body_str) = request.get_raw_body() {
            // Add body if present and no form data
            // Try to parse as JSON first
            match request.parse_body() {
                Ok(Some(json_value)) => {
                    req = req.json(&json_value);
                }
                _ => {
                    // If not valid JSON, send as plain text
                    req = req.body(body_str.to_string());
                }
            }
        }

        // Send request and measure time
        let response = req.send()?;
        let duration = start.elapsed();

        // Convert to our response type
        HttpResponse::from_reqwest(response, duration)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = HttpClient::new();
        assert!(std::mem::size_of_val(&client) > 0);
    }

    #[test]
    fn test_default() {
        let _client = HttpClient::default();
    }

    // Integration tests would go here with a mock server
    // For now, we'll add them in the integration test suite
}
