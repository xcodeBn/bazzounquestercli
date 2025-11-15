//! Integration tests for bazzounquester

use bazzounquester::{
    http::{HttpClient, HttpMethod, RequestBuilder},
    Result,
};

#[test]
fn test_http_get_request() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Get, "https://httpbin.org/get".to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());
    assert_eq!(response.status.as_u16(), 200);

    Ok(())
}

#[test]
fn test_http_get_with_query_params() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Get, "https://httpbin.org/get".to_string())
        .query("foo=bar".to_string())
        .query("baz=qux".to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());
    assert!(response.body.contains("\"foo\": \"bar\""));
    assert!(response.body.contains("\"baz\": \"qux\""));

    Ok(())
}

#[test]
fn test_http_post_with_json() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Post, "https://httpbin.org/post".to_string())
        .header("Content-Type:application/json".to_string())
        .body(r#"{"test":"data"}"#.to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());
    assert!(response.body.contains("\"test\": \"data\""));

    Ok(())
}

#[test]
fn test_http_put_request() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Put, "https://httpbin.org/put".to_string())
        .body(r#"{"updated":true}"#.to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());

    Ok(())
}

#[test]
fn test_http_delete_request() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Delete, "https://httpbin.org/delete".to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());

    Ok(())
}

#[test]
fn test_http_patch_request() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Patch, "https://httpbin.org/patch".to_string())
        .body(r#"{"patched":true}"#.to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());

    Ok(())
}

#[test]
fn test_custom_headers() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Get, "https://httpbin.org/headers".to_string())
        .header("X-Custom-Header:test-value".to_string())
        .header("User-Agent:bazzounquester-test".to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());
    assert!(response.body.contains("X-Custom-Header"));
    assert!(response.body.contains("test-value"));

    Ok(())
}

#[test]
fn test_response_time_tracking() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Get, "https://httpbin.org/delay/1".to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());
    // Should take at least 1 second
    assert!(response.duration.as_secs() >= 1);

    Ok(())
}

#[test]
fn test_json_response_parsing() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(HttpMethod::Get, "https://httpbin.org/json".to_string());

    let response = client.execute(&request)?;

    assert!(response.is_success());
    assert!(response.is_json());

    // Test pretty printing
    let pretty = response.pretty_body();
    assert!(!pretty.is_empty());

    Ok(())
}

#[test]
fn test_404_error() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(
        HttpMethod::Get,
        "https://httpbin.org/status/404".to_string(),
    );

    let response = client.execute(&request)?;

    assert!(response.is_client_error());
    assert_eq!(response.status.as_u16(), 404);
    assert_eq!(response.status_color(), "red");

    Ok(())
}

#[test]
fn test_500_error() -> Result<()> {
    let client = HttpClient::new();
    let request = RequestBuilder::new(
        HttpMethod::Get,
        "https://httpbin.org/status/500".to_string(),
    );

    let response = client.execute(&request)?;

    assert!(response.is_server_error());
    assert_eq!(response.status.as_u16(), 500);
    assert_eq!(response.status_color(), "red");

    Ok(())
}
