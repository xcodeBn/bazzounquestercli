//! Performance benchmarks for bazzounquester

use bazzounquester::http::{HttpMethod, RequestBuilder};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_request_builder(c: &mut Criterion) {
    c.bench_function("request_builder_creation", |b| {
        b.iter(|| {
            let request = RequestBuilder::new(
                black_box(HttpMethod::Get),
                black_box("https://example.com".to_string()),
            )
            .header(black_box("Content-Type:application/json".to_string()))
            .query(black_box("foo=bar".to_string()))
            .body(black_box(r#"{"key":"value"}"#.to_string()));

            black_box(request)
        });
    });
}

fn benchmark_header_parsing(c: &mut Criterion) {
    c.bench_function("parse_headers", |b| {
        b.iter(|| {
            let request = RequestBuilder::new(HttpMethod::Get, "https://example.com".to_string())
                .header("Content-Type:application/json".to_string())
                .header("Authorization:Bearer token123".to_string())
                .header("X-Custom-Header:value".to_string());

            black_box(request.parse_headers())
        });
    });
}

fn benchmark_query_param_parsing(c: &mut Criterion) {
    c.bench_function("parse_query_params", |b| {
        b.iter(|| {
            let request = RequestBuilder::new(HttpMethod::Get, "https://example.com".to_string())
                .query("foo=bar".to_string())
                .query("baz=qux".to_string())
                .query("test=value".to_string());

            black_box(request.parse_query_params())
        });
    });
}

fn benchmark_json_parsing(c: &mut Criterion) {
    c.bench_function("parse_json_body", |b| {
        b.iter(|| {
            let request = RequestBuilder::new(HttpMethod::Post, "https://example.com".to_string())
                .body(black_box(
                    r#"{"key":"value","nested":{"data":true}}"#.to_string(),
                ));

            black_box(request.parse_body())
        });
    });
}

fn benchmark_http_method_from_str(c: &mut Criterion) {
    c.bench_function("http_method_from_str", |b| {
        b.iter(|| {
            black_box(HttpMethod::parse(black_box("GET")));
            black_box(HttpMethod::parse(black_box("POST")));
            black_box(HttpMethod::parse(black_box("PUT")));
            black_box(HttpMethod::parse(black_box("DELETE")));
        });
    });
}

criterion_group!(
    benches,
    benchmark_request_builder,
    benchmark_header_parsing,
    benchmark_query_param_parsing,
    benchmark_json_parsing,
    benchmark_http_method_from_str
);

criterion_main!(benches);
