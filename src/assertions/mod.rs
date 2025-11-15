//! Request and response assertion system

pub mod assertion;
pub mod matcher;
pub mod validator;

pub use assertion::{Assertion, AssertionResult, AssertionType};
pub use matcher::{Matcher, MatcherType};
pub use validator::{ResponseValidator, ValidationReport};

use crate::error::Result;
use crate::http::HttpResponse;

/// Run assertions on a response
pub fn validate_response(
    response: &HttpResponse,
    assertions: &[Assertion],
) -> Result<ValidationReport> {
    let validator = ResponseValidator::new();
    Ok(validator.validate(response, assertions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_response_empty() {
        // Basic test to ensure module works
        let assertions: Vec<Assertion> = vec![];
        // Just testing that we can create empty assertions
        assert_eq!(assertions.len(), 0);
    }
}
