# Bazzounquester

<div align="center">

**Your Postman in the Terminal - A Professional HTTP Client & API Testing Tool**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-1.8.0-blue.svg)](https://github.com/yourusername/bazzounquester)
[![Tests](https://img.shields.io/badge/tests-280%20passing-brightgreen.svg)](https://github.com/yourusername/bazzounquester)
[![Coverage](https://img.shields.io/badge/coverage-~85%25-green.svg)](https://github.com/yourusername/bazzounquester)

Lightning-fast, feature-rich HTTP client for developers who live in the terminal

[Features](#features) • [Installation](#installation) • [Quick Start](#quick-start) • [Documentation](#core-capabilities) • [Examples](#examples)

</div>

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Capabilities](#core-capabilities)
  - [HTTP Requests](#http-requests)
  - [Authentication](#authentication)
  - [File Uploads](#file-uploads)
  - [Scripting](#pre--post-request-scripting)
  - [Assertions](#request-validation--assertions)
  - [Workflows](#request-chaining--workflows)
  - [Collections](#collections--workspaces)
  - [Environment Variables](#environment-variables)
  - [Sessions & Cookies](#sessions--cookies)
  - [History](#request-history)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

---

## Features

### Core HTTP Client
- **All HTTP Methods** - GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- **Interactive REPL Mode** - Natural command-line interface with history
- **Beautiful Output** - Color-coded responses optimized for light & dark terminals
- **Fast & Lightweight** - Built with Rust for maximum performance
- **Custom Headers** - Full control over request headers
- **Query Parameters** - Easy query string management
- **Response Timing** - Automatic performance measurement

### Advanced Features (Postman-Level)
- **Authentication Suite** - Basic, Bearer, API Key, OAuth 2.0 (5 grant types)
- **File Uploads** - Multipart form data with automatic MIME detection
- **Request Chaining** - Build complex workflows with variable extraction
- **Scripting Engine** - Pre/post-request scripts with JavaScript-like syntax
- **Assertions** - Comprehensive validation with 14+ matcher types
- **Collections** - Organize and save request collections
- **Environments** - Manage variables across dev/staging/prod
- **Sessions & Cookies** - Automatic cookie handling and persistence
- **Request History** - Track and replay past requests

### Developer Experience
- **Workspaces** - Organize projects efficiently
- **Syntax Highlighting** - Pretty-printed JSON responses
- **Comprehensive Testing** - 280+ unit tests for reliability
- **Modular Architecture** - Clean, maintainable codebase
- **Rich Documentation** - Examples and guides for every feature

---

## Installation

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)

### Option 1: Install from Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/bazzounquester.git
cd bazzounquester

# Install using cargo
cargo install --path .

# Verify installation
bazzounquester --version
```

### Option 2: Quick Build & Run

```bash
# Clone and build
git clone https://github.com/yourusername/bazzounquester.git
cd bazzounquester
cargo build --release

# Run directly
./target/release/bazzounquester
```

### Add to PATH

If `bazzounquester` command is not found, add Cargo's bin directory to your PATH:

```bash
# For Bash/Zsh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For Fish
echo 'set -gx PATH $HOME/.cargo/bin $PATH' >> ~/.config/fish/config.fish
```

### Optional: Create an Alias

```bash
alias bzq='bazzounquester'
alias req='bazzounquester'
```

---

## Quick Start

### Launch Interactive Mode

```bash
bazzounquester
```

You'll see:

```
╔═══════════════════════════════════════════════════════════════╗
║              Bazzounquester - Interactive Mode                ║
║          Your Postman in the Terminal - HTTP CLI Tool        ║
╚═══════════════════════════════════════════════════════════════╝

  Version: v1.8.0
  Author: Hassan Bazzoun <hassan.bazzoundev@gmail.com>

  Type 'help' for commands | 'version' for info | 'exit' to quit

bazzounquester>
```

### Your First Request

```bash
# Simple GET request
bazzounquester> get https://httpbin.org/json

# POST with JSON body
bazzounquester> post https://httpbin.org/post \
  -H "Content-Type:application/json" \
  -b '{"name":"John","email":"john@example.com"}'
```

### Command-Line Mode

```bash
# Direct execution without interactive mode
bazzounquester get https://api.github.com/users/octocat

# With authentication
bazzounquester get https://api.github.com/user \
  -H "Authorization:Bearer YOUR_TOKEN"
```

---

## Core Capabilities

### HTTP Requests

#### All HTTP Methods

```bash
# GET
get https://api.example.com/users

# POST with JSON
post https://api.example.com/users \
  -H "Content-Type:application/json" \
  -b '{"name":"Alice","role":"admin"}'

# PUT
put https://api.example.com/users/123 \
  -b '{"name":"Alice Updated"}'

# PATCH
patch https://api.example.com/users/123 \
  -b '{"email":"newemail@example.com"}'

# DELETE
delete https://api.example.com/users/123

# HEAD (headers only)
head https://api.example.com/users

# OPTIONS (check allowed methods)
options https://api.example.com/users
```

#### Query Parameters

```bash
get https://api.example.com/search \
  -q "query=rust" \
  -q "page=1" \
  -q "limit=20"
```

#### Custom Headers

```bash
get https://api.example.com/data \
  -H "Authorization:Bearer token123" \
  -H "X-API-Key:secret" \
  -H "Accept:application/json"
```

---

### Authentication

Bazzounquester supports all major authentication methods:

#### Basic Authentication

```rust
use bazzounquester::auth::BasicAuth;

let auth = BasicAuth::new("username", "password");
let encoded = auth.encode(); // Base64 encoded credentials
```

#### Bearer Token

```rust
use bazzounquester::auth::BearerAuth;

let auth = BearerAuth::new("your-jwt-token");
```

#### API Key

```rust
use bazzounquester::auth::ApiKeyAuth;

// Header-based
let auth = ApiKeyAuth::header("X-API-Key", "secret-key");

// Query-based
let auth = ApiKeyAuth::query("api_key", "secret-key");
```

#### OAuth 2.0

Supports all 5 OAuth 2.0 grant types:

```rust
use bazzounquester::auth::{OAuth2Auth, GrantType};

// Authorization Code
let auth = OAuth2Auth::new(
    "client_id",
    "client_secret",
    GrantType::AuthorizationCode {
        auth_url: "https://auth.example.com/authorize".to_string(),
        token_url: "https://auth.example.com/token".to_string(),
        redirect_uri: "http://localhost:8080/callback".to_string(),
    }
);

// Client Credentials
let auth = OAuth2Auth::new(
    "client_id",
    "client_secret",
    GrantType::ClientCredentials {
        token_url: "https://auth.example.com/token".to_string(),
    }
);

// Password Grant
let auth = OAuth2Auth::new(
    "client_id",
    "client_secret",
    GrantType::Password {
        token_url: "https://auth.example.com/token".to_string(),
        username: "user@example.com".to_string(),
        password: "password".to_string(),
    }
);
```

---

### File Uploads

Upload files with automatic MIME type detection and multipart form data support:

```rust
use bazzounquester::upload::{FileUpload, FormData};

// Single file upload
let file = FileUpload::new("./document.pdf", "file".to_string())?;

// Form data with multiple fields
let mut form = FormData::new();
form.add_field("name", "John Doe");
form.add_field("email", "john@example.com");
form.add_file(file)?;

// Build multipart request
let multipart = form.build()?;
```

**Features:**
- Automatic MIME type detection
- Custom filenames
- Multiple file uploads
- Mixed form fields and files
- File size validation

---

### Pre & Post-Request Scripting

Execute JavaScript-like scripts before and after requests using the Rhai scripting engine:

```rust
use bazzounquester::scripts::{Script, ScriptEngine, ScriptContext};

// Pre-request script
let pre_script = Script::pre(r#"
    // Set dynamic timestamp
    let timestamp = new Date().getTime();
    variables.set("timestamp", timestamp);

    // Generate request ID
    let request_id = "req_" + Math.random();
    variables.set("request_id", request_id);

    console.log("Request prepared: " + request_id);
"#);

// Post-request script
let post_script = Script::post(r#"
    // Extract auth token from response
    let data = JSON.parse(response.body);
    variables.set("auth_token", data.token);

    // Log response time
    console.log("Request took: " + response.time + "ms");

    // Conditional logic
    if (response.status == 200) {
        console.log("Success!");
    }
"#);

// Execute scripts
let mut context = ScriptContext::new();
let mut engine = ScriptEngine::new();

engine.execute(&pre_script, &mut context)?;
```

**Available in Scripts:**
- `variables` - Get/set variables
- `response` - Access response data (post-request only)
- `console.log()` - Debug output
- `JSON.parse()` / `JSON.stringify()` - JSON handling
- Math operations and string manipulation

---

### Request Validation & Assertions

Validate responses with powerful assertion system:

```rust
use bazzounquester::assertions::{Assertion, Matcher, MatcherType, ResponseValidator};

// Status code assertion
let assertion = Assertion::new(
    "$.status",
    Matcher::new(MatcherType::Equals, "200")
);

// Response body assertions
let assertions = vec![
    Assertion::new("$.data.name", Matcher::new(MatcherType::Equals, "John Doe")),
    Assertion::new("$.data.email", Matcher::new(MatcherType::Contains, "@example.com")),
    Assertion::new("$.data.age", Matcher::new(MatcherType::GreaterThan, "18")),
    Assertion::new("$.data.roles", Matcher::new(MatcherType::Contains, "admin")),
];

// Validate response
let validator = ResponseValidator::new(assertions);
let results = validator.validate(&response)?;

for result in results {
    if result.passed {
        println!("✓ {}", result.assertion.description);
    } else {
        println!("✗ {} - {}", result.assertion.description, result.error.unwrap());
    }
}
```

**Supported Matchers (14 types):**
- `Equals`, `NotEquals` - Exact matching
- `Contains`, `NotContains` - Substring matching
- `StartsWith`, `EndsWith` - Prefix/suffix matching
- `Regex` - Regular expression matching
- `LessThan`, `LessThanOrEqual`, `GreaterThan`, `GreaterThanOrEqual` - Numeric comparison
- `IsEmpty`, `IsNotEmpty` - Empty checks
- `HasLength` - Length validation
- `IsNull`, `IsNotNull` - Null checks

---

### Request Chaining & Workflows

Build complex multi-step workflows with variable extraction and substitution:

```rust
use bazzounquester::workflow::{WorkflowStep, RequestChain, WorkflowExecutor};

// Step 1: Login and extract token
let login_step = WorkflowStep::new("login")
    .request(
        RequestBuilder::new(HttpMethod::Post, "https://api.example.com/login")
            .body(r#"{"username":"admin","password":"secret"}"#)
    )
    .extract_variable("auth_token", "$.data.token")
    .extract_variable("user_id", "$.data.user_id");

// Step 2: Fetch user data using extracted token
let fetch_step = WorkflowStep::new("fetch_user")
    .request(
        RequestBuilder::new(HttpMethod::Get, "https://api.example.com/users/{{user_id}}")
            .header("Authorization", "Bearer {{auth_token}}")
    )
    .assertion(Assertion::new("$.status", Matcher::equals("success")))
    .extract_variable("user_email", "$.data.email");

// Step 3: Update user profile
let update_step = WorkflowStep::new("update_user")
    .request(
        RequestBuilder::new(HttpMethod::Put, "https://api.example.com/users/{{user_id}}")
            .header("Authorization", "Bearer {{auth_token}}")
            .body(r#"{"email":"{{user_email}}","verified":true}"#)
    )
    .assertion(Assertion::new("$.status", Matcher::equals("updated")));

// Build and execute workflow
let chain = RequestChain::new("user_update_flow")
    .step(login_step)
    .step(fetch_step)
    .step(update_step);

let executor = WorkflowExecutor::new(client);
let result = executor.execute(&chain)?;

println!("Workflow completed in {:.2}s", result.total_duration.as_secs_f64());
println!("Steps: {}/{} successful", result.successful_steps, result.total_steps);
```

**Workflow Features:**
- Variable extraction from responses (JSON path)
- Variable substitution in URLs, headers, and bodies
- Conditional execution
- Iteration over data sets
- Pre/post-request scripts per step
- Assertions per step
- Detailed execution results

---

### Collections & Workspaces

Organize and save your requests:

```rust
use bazzounquester::collections::{Collection, Workspace};

// Create a collection
let mut collection = Collection::new("API Tests");
collection.add_request("Get Users", request_builder);
collection.add_request("Create User", create_request);

// Organize into workspace
let mut workspace = Workspace::new("Development");
workspace.add_collection(collection);

// Save to disk
workspace.save()?;

// Load later
let workspace = Workspace::load("Development")?;
```

---

### Environment Variables

Manage variables across different environments:

```rust
use bazzounquester::env::{Environment, EnvironmentManager};

// Create environments
let mut dev_env = Environment::new("development");
dev_env.set("api_url", "http://localhost:3000");
dev_env.set("api_key", "dev-key-123");

let mut prod_env = Environment::new("production");
prod_env.set("api_url", "https://api.example.com");
prod_env.set("api_key", "prod-key-secret");

// Manage environments
let mut manager = EnvironmentManager::new();
manager.add_environment(dev_env);
manager.add_environment(prod_env);
manager.set_active("development");

// Use variables in requests
let url = manager.substitute("{{api_url}}/users");
```

---

### Sessions & Cookies

Automatic cookie handling and persistence:

```rust
use bazzounquester::session::{Session, CookieJar};

// Create session
let mut session = Session::new("my-session");

// Cookies are automatically extracted and sent with subsequent requests
let response1 = client.get("https://example.com/login")?;
session.update_from_response(&response1);

let response2 = client.get("https://example.com/profile")?;
// Cookies from response1 are automatically included

// Persist session
session.save()?;

// Restore later
let session = Session::load("my-session")?;
```

---

### Request History

Track and replay past requests:

```rust
use bazzounquester::history::{HistoryManager, HistoryEntry};

let mut history = HistoryManager::new();

// Automatically logged
history.log(request, response);

// Search history
let entries = history.search("github.com")?;

// Replay request
let entry = history.get(entry_id)?;
let response = entry.replay()?;

// Export history
history.export_json("history.json")?;
```

---

## Examples

### Example 1: Simple API Testing

```bash
# Test JSONPlaceholder API
bazzounquester get https://jsonplaceholder.typicode.com/users/1

# Create a post
bazzounquester post https://jsonplaceholder.typicode.com/posts \
  -H "Content-Type:application/json" \
  -b '{"title":"My Post","body":"Content","userId":1}'
```

### Example 2: GitHub API with Authentication

```bash
# Fetch authenticated user
bazzounquester get https://api.github.com/user \
  -H "Authorization:Bearer YOUR_GITHUB_TOKEN"

# List repositories
bazzounquester get https://api.github.com/user/repos \
  -H "Authorization:Bearer YOUR_GITHUB_TOKEN" \
  -q "sort=updated" \
  -q "per_page=10"
```

### Example 3: Complete Workflow

Run the comprehensive showcase example:

```bash
cargo run --example showcase
```

This demonstrates:
- Simple HTTP requests
- Authentication (Basic, Bearer, OAuth)
- File uploads
- Pre/post-request scripting
- Response assertions
- Multi-step workflows

---

## Architecture

Bazzounquester features a clean, modular architecture:

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library exports
├── http/                # HTTP client & request handling
│   ├── client.rs        # HTTP client implementation
│   ├── request.rs       # Request builder
│   ├── response.rs      # Response handling
│   └── method.rs        # HTTP methods
├── auth/                # Authentication
│   ├── basic.rs         # Basic auth
│   ├── bearer.rs        # Bearer tokens
│   ├── apikey.rs        # API key auth
│   └── oauth2.rs        # OAuth 2.0 (5 grant types)
├── upload/              # File uploads
│   ├── file.rs          # File upload handling
│   ├── form.rs          # Form data
│   └── multipart.rs     # Multipart builder
├── scripts/             # Scripting engine
│   ├── script.rs        # Script definitions
│   ├── engine.rs        # Rhai execution engine
│   └── context.rs       # Script context
├── assertions/          # Validation system
│   ├── assertion.rs     # Assertion definitions
│   ├── matcher.rs       # 14 matcher types
│   └── validator.rs     # Response validation
├── workflow/            # Request chaining
│   ├── step.rs          # Workflow steps
│   ├── chain.rs         # Request chains
│   ├── executor.rs      # Workflow execution
│   └── variables.rs     # Variable substitution
├── collections/         # Collections & workspaces
├── env/                 # Environment management
├── session/             # Session & cookies
└── history/             # Request history
```

**280+ Unit Tests** covering all modules

---

## Development

### Run Tests

```bash
# All tests
cargo test

# Library tests only
cargo test --lib

# With output
cargo test -- --nocapture
```

### Run Benchmarks

```bash
cargo bench
```

### Format & Lint

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

---

## Roadmap

### Completed (v1.0 - v1.8)
- Core HTTP client (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- Interactive REPL mode
- Beautiful terminal output (light & dark mode optimized)
- Modular architecture with 13+ modules
- Comprehensive test suite (280+ tests)
- Collections and workspaces
- Environment variables
- Request/response history
- Sessions and cookie management
- File upload and multipart forms
- Authentication suite (Basic, Bearer, API Key, OAuth 2.0)
- Pre/post-request scripting (Rhai engine)
- Response assertions (14 matcher types)
- Request chaining and workflows

### Planned (v2.0+)
- Full async/await with Tokio
- HTTP/2 and HTTP/3 support
- WebSocket client
- GraphQL support
- Performance testing & load generation
- Mock server capabilities
- CI/CD test runner (Newman-like)
- OpenAPI/Swagger import
- Enhanced TUI with syntax highlighting
- Autocomplete for URLs and headers
- Security testing (OWASP checks)

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Start for Contributors

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Commit: `git commit -m 'Add amazing feature'`
7. Push: `git push origin feature/amazing-feature`
8. Open a Pull Request

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Author

**Hassan Bazzoun**
- Email: hassan.bazzoundev@gmail.com
- GitHub: [@yourusername](https://github.com/yourusername)

---

## Acknowledgments

Built with these open-source libraries:

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [clap](https://github.com/clap-rs/clap) - Command-line parser
- [rustyline](https://github.com/kkawakam/rustyline) - Readline implementation
- [colored](https://github.com/mackwic/colored) - Terminal colors
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [rhai](https://github.com/rhaiscript/rhai) - Embedded scripting language
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime

---

## Project Stats

![Version](https://img.shields.io/badge/version-1.8.0-blue.svg)
![Tests](https://img.shields.io/badge/tests-280%20passing-brightgreen.svg)
![Modules](https://img.shields.io/badge/modules-13+-purple.svg)
![Lines of Code](https://img.shields.io/badge/lines%20of%20code-8000+-yellow.svg)
![Coverage](https://img.shields.io/badge/coverage-~85%25-green.svg)
