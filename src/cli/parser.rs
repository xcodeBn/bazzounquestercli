//! Command parsing utilities

use crate::error::{Error, Result};
use crate::http::{HttpMethod, RequestBuilder};

/// Parser for HTTP commands in interactive mode
pub struct CommandParser;

impl CommandParser {
    /// Parse a command line into arguments using shell-like parsing
    pub fn parse_line(input: &str) -> Result<Vec<String>> {
        shlex::split(input).ok_or_else(|| {
            Error::InvalidCommand("Invalid command syntax. Check your quotes.".to_string())
        })
    }

    /// Parse HTTP command arguments into a RequestBuilder
    pub fn parse_http_command(method: &str, args: &[String]) -> Result<RequestBuilder> {
        if args.is_empty() {
            return Err(Error::MissingArgument(format!(
                "Missing URL. Usage: {} <url> [options]",
                method
            )));
        }

        let url = args[0].clone();
        let http_method = HttpMethod::from_str(method)?;
        let mut builder = RequestBuilder::new(http_method, url);

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-H" | "--header" => {
                    if i + 1 < args.len() {
                        builder = builder.header(args[i + 1].clone());
                        i += 2;
                    } else {
                        return Err(Error::MissingArgument("Missing value for -H flag".to_string()));
                    }
                }
                "-q" | "--query" => {
                    if i + 1 < args.len() {
                        builder = builder.query(args[i + 1].clone());
                        i += 2;
                    } else {
                        return Err(Error::MissingArgument("Missing value for -q flag".to_string()));
                    }
                }
                "-b" | "--body" => {
                    if i + 1 < args.len() {
                        builder = builder.body(args[i + 1].clone());
                        i += 2;
                    } else {
                        return Err(Error::MissingArgument("Missing value for -b flag".to_string()));
                    }
                }
                _ => {
                    return Err(Error::InvalidCommand(format!(
                        "Unknown option: {}",
                        args[i]
                    )));
                }
            }
        }

        Ok(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_simple() {
        let result = CommandParser::parse_line("get https://example.com").unwrap();
        assert_eq!(result, vec!["get", "https://example.com"]);
    }

    #[test]
    fn test_parse_line_with_quotes() {
        let result = CommandParser::parse_line(r#"post https://example.com -H "Content-Type:application/json""#).unwrap();
        assert_eq!(result.len(), 4);
        assert_eq!(result[3], "Content-Type:application/json");
    }

    #[test]
    fn test_parse_line_invalid_quotes() {
        let result = CommandParser::parse_line(r#"get "unclosed quote"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_http_command_get() {
        let args = vec!["https://example.com".to_string()];
        let builder = CommandParser::parse_http_command("GET", &args).unwrap();
        assert_eq!(builder.method, HttpMethod::Get);
        assert_eq!(builder.url, "https://example.com");
    }

    #[test]
    fn test_parse_http_command_with_headers() {
        let args = vec![
            "https://example.com".to_string(),
            "-H".to_string(),
            "Content-Type:application/json".to_string(),
        ];
        let builder = CommandParser::parse_http_command("POST", &args).unwrap();
        assert_eq!(builder.headers.len(), 1);
    }

    #[test]
    fn test_parse_http_command_with_query() {
        let args = vec![
            "https://example.com".to_string(),
            "-q".to_string(),
            "foo=bar".to_string(),
        ];
        let builder = CommandParser::parse_http_command("GET", &args).unwrap();
        assert_eq!(builder.query_params.len(), 1);
    }

    #[test]
    fn test_parse_http_command_with_body() {
        let args = vec![
            "https://example.com".to_string(),
            "-b".to_string(),
            r#"{"key":"value"}"#.to_string(),
        ];
        let builder = CommandParser::parse_http_command("POST", &args).unwrap();
        assert!(builder.body.is_some());
    }

    #[test]
    fn test_parse_http_command_no_url() {
        let args: Vec<String> = vec![];
        let result = CommandParser::parse_http_command("GET", &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_http_command_unknown_option() {
        let args = vec![
            "https://example.com".to_string(),
            "-x".to_string(),
            "value".to_string(),
        ];
        let result = CommandParser::parse_http_command("GET", &args);
        assert!(result.is_err());
    }
}
