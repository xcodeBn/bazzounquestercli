# Contributing to Bazzounquester

Thank you for considering contributing to Bazzounquester! This document provides guidelines and instructions for contributing to the project.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Questions](#questions)

---

## Code of Conduct

This project and everyone participating in it is governed by respect and professionalism. By participating, you are expected to:

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

---

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

**Bug Report Template:**

```markdown
**Description:**
A clear and concise description of the bug.

**Steps to Reproduce:**
1. Step one
2. Step two
3. ...

**Expected Behavior:**
What you expected to happen.

**Actual Behavior:**
What actually happened.

**Environment:**
- OS: [e.g., Ubuntu 22.04, macOS 13.0, Windows 11]
- Rust Version: [e.g., 1.70.0]
- Bazzounquester Version: [e.g., 1.8.0]

**Additional Context:**
Any other relevant information (logs, screenshots, etc.)
```

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

**Enhancement Template:**

```markdown
**Feature Description:**
A clear and concise description of the feature.

**Use Case:**
Describe the problem this feature would solve.

**Proposed Solution:**
How you envision this feature working.

**Alternatives Considered:**
Any alternative solutions you've thought about.

**Additional Context:**
Mockups, examples, or references to similar features.
```

### Contributing Code

We welcome code contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch** from `main`
3. **Make your changes** with tests
4. **Submit a pull request**

---

## Development Setup

### Prerequisites

- **Rust 1.70 or higher** - [Install Rust](https://rustup.rs/)
- **Git** - For version control
- **A code editor** - We recommend VS Code with rust-analyzer

### Setup Steps

1. **Fork and clone the repository:**

```bash
git clone https://github.com/yourusername/bazzounquester.git
cd bazzounquester
```

2. **Create a feature branch:**

```bash
git checkout -b feature/your-feature-name
```

3. **Install dependencies and build:**

```bash
cargo build
```

4. **Run tests to verify setup:**

```bash
cargo test
```

5. **Run the application:**

```bash
cargo run
```

---

## Project Structure

Understanding the project structure will help you navigate the codebase:

```
bazzounquester/
├── src/
│   ├── main.rs              # CLI entry point & REPL
│   ├── lib.rs               # Library exports
│   │
│   ├── http/                # HTTP client & requests
│   │   ├── mod.rs
│   │   ├── client.rs        # HTTP client
│   │   ├── request.rs       # Request builder
│   │   ├── response.rs      # Response handling
│   │   └── method.rs        # HTTP methods
│   │
│   ├── auth/                # Authentication
│   │   ├── mod.rs
│   │   ├── basic.rs         # Basic auth
│   │   ├── bearer.rs        # Bearer tokens
│   │   ├── apikey.rs        # API keys
│   │   └── oauth2.rs        # OAuth 2.0
│   │
│   ├── upload/              # File uploads
│   │   ├── mod.rs
│   │   ├── file.rs
│   │   ├── form.rs
│   │   └── multipart.rs
│   │
│   ├── scripts/             # Scripting engine
│   │   ├── mod.rs
│   │   ├── script.rs
│   │   ├── engine.rs
│   │   └── context.rs
│   │
│   ├── assertions/          # Validation
│   │   ├── mod.rs
│   │   ├── assertion.rs
│   │   ├── matcher.rs
│   │   └── validator.rs
│   │
│   ├── workflow/            # Request chaining
│   │   ├── mod.rs
│   │   ├── step.rs
│   │   ├── chain.rs
│   │   ├── executor.rs
│   │   └── variables.rs
│   │
│   ├── collections/         # Collections & workspaces
│   ├── env/                 # Environment variables
│   ├── session/             # Sessions & cookies
│   └── history/             # Request history
│
├── examples/                # Example code
│   └── showcase.rs          # Feature showcase
│
├── benches/                 # Performance benchmarks
│   └── request_benchmarks.rs
│
├── tests/                   # Integration tests
│
├── Cargo.toml               # Dependencies & metadata
├── README.md                # Project documentation
├── ARCHITECTURE.md          # Architecture details
├── CONTRIBUTING.md          # This file
├── LICENSE                  # MIT License
└── .gitignore
```

### Key Modules

- **`http/`** - Core HTTP functionality (client, requests, responses)
- **`auth/`** - Authentication methods (Basic, Bearer, API Key, OAuth 2.0)
- **`upload/`** - File uploads with multipart form data
- **`scripts/`** - Pre/post-request scripting with Rhai
- **`assertions/`** - Response validation with 14 matcher types
- **`workflow/`** - Multi-step request chaining
- **`collections/`** - Save and organize requests
- **`env/`** - Environment variable management
- **`session/`** - Cookie handling and persistence
- **`history/`** - Request/response logging

---

## Coding Standards

### Rust Conventions

Follow standard Rust conventions and idioms:

- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Use descriptive names
- Prefer `&str` over `String` for function parameters when possible
- Use `Result<T>` for fallible operations
- Avoid `unwrap()` in library code (use `?` or `expect()`)

### Formatting

**Always format your code before committing:**

```bash
cargo fmt
```

### Linting

**Run Clippy to catch common mistakes:**

```bash
cargo clippy -- -D warnings
```

Fix all Clippy warnings before submitting a PR.

### Documentation

**Document all public APIs:**

```rust
/// Sends an HTTP GET request to the specified URL.
///
/// # Arguments
///
/// * `url` - The URL to send the request to
///
/// # Returns
///
/// Returns a `Result` containing the `HttpResponse` on success,
/// or an error if the request fails.
///
/// # Examples
///
/// ```
/// let client = HttpClient::new();
/// let response = client.get("https://api.example.com/users")?;
/// ```
pub fn get(&self, url: &str) -> Result<HttpResponse> {
    // ...
}
```

### Error Handling

- Use `Result<T>` for all fallible operations
- Create descriptive error types
- Provide context in error messages
- Don't use `panic!()` in library code

**Example:**

```rust
use std::error::Error;

pub fn parse_url(url: &str) -> Result<Url, Box<dyn Error>> {
    Url::parse(url)
        .map_err(|e| format!("Failed to parse URL '{}': {}", url, e).into())
}
```

---

## Testing

### Writing Tests

**Every module should have tests.** We aim for >85% code coverage.

#### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_encoding() {
        let auth = BasicAuth::new("user", "pass");
        let encoded = auth.encode();
        assert_eq!(encoded, "dXNlcjpwYXNz");
    }

    #[test]
    fn test_request_builder() {
        let request = RequestBuilder::new(HttpMethod::Get, "https://example.com")
            .header("User-Agent", "test")
            .build()
            .unwrap();

        assert_eq!(request.method, HttpMethod::Get);
        assert_eq!(request.headers.get("user-agent").unwrap(), "test");
    }
}
```

#### Integration Tests

Place integration tests in the `tests/` directory:

```rust
// tests/integration/workflow_test.rs
use bazzounquester::workflow::*;

#[test]
fn test_workflow_execution() {
    // Test complete workflow
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run library tests only
cargo test --lib

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_auth_encoding

# Run tests in a specific module
cargo test http::
```

### Test Coverage

Aim for high test coverage:

- **Critical paths:** 100% coverage
- **Public APIs:** 95%+ coverage
- **Overall:** 85%+ coverage

---

## Pull Request Process

### Before Submitting

1. **Run all tests:**
   ```bash
   cargo test
   ```

2. **Format code:**
   ```bash
   cargo fmt
   ```

3. **Run Clippy:**
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Build successfully:**
   ```bash
   cargo build --release
   ```

5. **Update documentation** if you changed public APIs

6. **Add tests** for new functionality

### PR Guidelines

1. **Create a descriptive PR title:**
   - ✅ `Add OAuth 2.0 refresh token support`
   - ✅ `Fix: Response timing calculation for chunked responses`
   - ❌ `Update code`
   - ❌ `Fixes`

2. **Fill out the PR template:**

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested your changes

## Checklist
- [ ] My code follows the project's coding standards
- [ ] I have added tests for my changes
- [ ] All tests pass locally
- [ ] I have updated the documentation
- [ ] My changes generate no new warnings
```

3. **Link related issues:**
   - Use keywords: `Fixes #123`, `Closes #456`, `Relates to #789`

4. **Keep PRs focused:**
   - One feature/fix per PR
   - Avoid mixing refactoring with feature work

5. **Be responsive to feedback:**
   - Address review comments promptly
   - Ask questions if something is unclear

### Commit Messages

Write clear, descriptive commit messages:

**Format:**
```
<type>: <short summary>

<detailed description (optional)>

<issue references (optional)>
```

**Types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Maintenance tasks

**Examples:**

```bash
feat: Add OAuth 2.0 refresh token grant type

Implements the refresh token grant type for OAuth 2.0 authentication.
Includes automatic token renewal and expiration handling.

Fixes #123
```

```bash
fix: Correct response timing for chunked transfers

Response duration was incorrectly calculated for chunked transfer
encoding. Now uses start time from connection establishment.
```

---

## Development Workflow

### Typical Development Cycle

1. **Pick an issue or feature to work on**
   - Check the issue tracker
   - Ask in discussions if you're not sure where to start

2. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Write tests first (TDD):**
   ```bash
   # Write failing tests
   cargo test your_feature
   ```

4. **Implement the feature:**
   - Write clean, documented code
   - Follow the coding standards

5. **Run tests frequently:**
   ```bash
   cargo test
   ```

6. **Format and lint:**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

7. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat: Add amazing feature"
   ```

8. **Push and create PR:**
   ```bash
   git push origin feature/your-feature-name
   ```

9. **Address review feedback**

10. **Celebrate when merged! 🎉**

---

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Update version in README badges
4. Run full test suite
5. Create git tag: `git tag v1.x.0`
6. Push tag: `git push origin v1.x.0`
7. Publish to crates.io: `cargo publish`

---

## Questions?

- **GitHub Issues:** For bugs and feature requests
- **GitHub Discussions:** For questions and general discussion
- **Email:** hassan.bazzoundev@gmail.com

---

## Recognition

Contributors will be recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Project README

Thank you for contributing to Bazzounquester! 🚀
