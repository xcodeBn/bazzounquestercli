// Example test program showcasing all features
use bazzounquester::{
    assertions::{Assertion, Matcher},
    auth::{AuthScheme, BearerAuth},
    http::{HttpClient, HttpMethod, RequestBuilder},
    scripts::{Script, ScriptContext, ScriptType},
    workflow::{execute_chain, RequestChain, WorkflowStep},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Bazzounquester Feature Showcase\n");

    // Test 1: Simple request
    println!("1️⃣  Simple GET request:");
    simple_request()?;

    // Test 2: Authentication
    println!("\n2️⃣  Bearer authentication:");
    auth_request()?;

    // Test 3: Scripting
    println!("\n3️⃣  Pre/Post-request scripting:");
    script_example()?;

    // Test 4: Assertions
    println!("\n4️⃣  Response validation:");
    assertion_example()?;

    // Test 5: Workflow
    println!("\n5️⃣  Request workflow:");
    workflow_example()?;

    println!("\n✅ All features tested successfully!");
    Ok(())
}

fn simple_request() -> Result<(), Box<dyn std::error::Error>> {
    let request = RequestBuilder::new(HttpMethod::Get, "https://httpbin.org/uuid".to_string());

    let client = HttpClient::new();
    let response = client.execute(&request)?;

    println!("   Status: {}", response.status);
    println!("   Body: {}", response.body);
    Ok(())
}

fn auth_request() -> Result<(), Box<dyn std::error::Error>> {
    let auth = AuthScheme::Bearer(BearerAuth::new("test-token-123".to_string()));

    let request =
        RequestBuilder::new(HttpMethod::Get, "https://httpbin.org/bearer".to_string()).auth(auth);

    let client = HttpClient::new();
    let response = client.execute(&request)?;

    println!("   Authenticated request sent");
    println!("   Status: {}", response.status);
    Ok(())
}

fn script_example() -> Result<(), Box<dyn std::error::Error>> {
    let script = Script::new(
        ScriptType::PreRequest,
        r#"
        let timestamp = "1234567890";
        log("Setting timestamp: " + timestamp);
        "#
        .to_string(),
    );

    let mut context = ScriptContext::new();
    bazzounquester::scripts::execute_pre_request(&script, &mut context)?;

    println!("   Script executed");
    println!("   Console output: {:?}", context.console_output());
    Ok(())
}

fn assertion_example() -> Result<(), Box<dyn std::error::Error>> {
    let request = RequestBuilder::new(
        HttpMethod::Get,
        "https://httpbin.org/status/200".to_string(),
    );

    let client = HttpClient::new();
    let response = client.execute(&request)?;

    let assertions = vec![
        Assertion::status_code(Matcher::equals(200)),
        Assertion::response_time(Matcher::less_than(5000)),
    ];

    let report = bazzounquester::assertions::validate_response(&response, &assertions)?;
    println!("   {}", report.summary());
    Ok(())
}

fn workflow_example() -> Result<(), Box<dyn std::error::Error>> {
    let chain = RequestChain::new("Test Workflow".to_string())
        .add_step(WorkflowStep::new(
            "Get UUID".to_string(),
            HttpMethod::Get,
            "https://httpbin.org/uuid".to_string(),
        ))
        .add_step(WorkflowStep::new(
            "Get JSON".to_string(),
            HttpMethod::Get,
            "https://httpbin.org/json".to_string(),
        ));

    let result = execute_chain(&chain)?;
    println!("   {}", result.summary());
    Ok(())
}
