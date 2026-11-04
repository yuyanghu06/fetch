use super::*;
use crate::runner::startup_diagnostics::StartupDiagnostics;
use std::collections::BTreeMap;

fn transport()->TransportConfig {TransportConfig::Http{endpoint:"https://fixture.example/mcp".into(),headers:BTreeMap::new()}}

#[tokio::test]
async fn startup_capture_exists_before_configuration_and_old_handles_cannot_contaminate_new_attempts() {
    let runner=Runner::new();let first=runner.begin_startup_diagnostics("not-configured",&transport()).await;
    first.line("first attempt detail");let initial=runner.startup_diagnostics("not-configured").await.unwrap();
    assert!(initial.to_string().contains("first attempt detail"));
    let second=runner.begin_startup_diagnostics("not-configured",&transport()).await;
    first.line("old late callback must be absent");first.freeze();second.line("second attempt still writable");
    let current=runner.startup_diagnostics("not-configured").await.unwrap();
    assert!(current["generation"].as_u64().unwrap()>initial["generation"].as_u64().unwrap());
    assert!(current.to_string().contains("second attempt still writable"));
    assert!(current.to_string().contains("first attempt detail"),"startup history must survive an unpolled attempt transition");assert!(!current.to_string().contains("old late callback"));
    runner.shutdown().await;
}

#[test]
fn latest_history_and_explicit_limits_replace_the_old_prefix_cap() {
    let capture=StartupDiagnostics::new(1,&transport()).unwrap();
    for index in 0..4200 {capture.line(&format!("startup prefix {index:04}"));}
    let value=capture.snapshot();let lines=value["lines"].as_array().unwrap();
    assert!(lines[0].as_str().unwrap().contains("Older console output was removed"));assert!(lines.len()<=4096);
    assert_eq!(lines.last().unwrap(),"startup prefix 4199");
    assert!(!value.to_string().contains("startup prefix 0000"));
    let long=StartupDiagnostics::new(2,&transport()).unwrap();long.line(&"x".repeat(70000));
    assert!(long.snapshot().to_string().contains("line truncated"));
    let bytes=StartupDiagnostics::new(3,&transport()).unwrap();
    for _ in 0..30 {bytes.line(&"y".repeat(60000));}
    let value=bytes.snapshot();let lines=value["lines"].as_array().unwrap();
    assert!(lines.iter().map(|line|line.as_str().unwrap().len()).sum::<usize>()<=1024*1024);
    assert!(lines[0].as_str().unwrap().contains("Older console output was removed"));
}

#[test]
fn structured_service_errors_keep_numeric_code_and_reason_but_remove_nested_secrets() {
    let capture=StartupDiagnostics::new(1,&transport()).unwrap();
    capture.add_redactions(["assigned-private-value".into()]);
    capture.value("mcp error",&json!({"code":-32001,"message":"helpful service reason assigned-private-value",
        "data":{"access_token":"private-access","cookie":"private-cookie","code_verifier":"private-verifier",
            "code":"private-code","state":"private-state","reason":"separate explanatory detail",
            "inner":{"refresh_token":"private-refresh"}}}));
    capture.text("stderr","Authorization: Bearer private-bearer");
    let value=capture.snapshot().to_string();
    assert!(value.contains("-32001"));assert!(value.contains("separate explanatory detail"));
    for secret in ["assigned-private-value","private-access","private-cookie","private-verifier","private-code","private-state","private-refresh","private-bearer"]{assert!(!value.contains(secret),"leaked {secret}");}
}

#[test]
fn secret_values_in_json_keys_and_numeric_credential_fields_are_redacted() {
    let capture=StartupDiagnostics::new(1,&transport()).unwrap();
    capture.add_redactions(["secret-json-key".into()]);
    capture.value("service error",&json!({"code":-32001,"secret-json-key":"harmless text",
        "access_token":987654321,"password":765432109,"message":"original useful denial"}));
    let value=capture.snapshot().to_string();assert!(value.contains("-32001"));assert!(value.contains("original useful denial"));
    for secret in ["secret-json-key","987654321","765432109"]{assert!(!value.contains(secret),"leaked {secret}");}
}

#[tokio::test]
async fn borrowed_remote_verification_probes_do_not_freeze_the_parent_oauth_capture() {
    use axum::response::IntoResponse;
    let listener=tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();let endpoint=format!("http://{}/mcp",listener.local_addr().unwrap());
    let app=axum::Router::new().route("/mcp",axum::routing::post(|headers:axum::http::HeaderMap,axum::Json(request):axum::Json<Value>|async move {
        if !headers.contains_key("authorization"){return (axum::http::StatusCode::UNAUTHORIZED,axum::Json(json!({"error":"login-required"}))).into_response();}
        if request["method"]=="notifications/initialized" {return axum::http::StatusCode::ACCEPTED.into_response();}
        axum::Json(json!({"jsonrpc":"2.0","id":request["id"],"result":{"protocolVersion":"2025-11-25","capabilities":{},"serverInfo":{"name":"protected-fixture","version":"1"}}})).into_response()
    }));
    let upstream=tokio::spawn(async move{axum::serve(listener,app).await.unwrap()});
    let runner=Runner::new();let transport=TransportConfig::Http{endpoint,headers:BTreeMap::from([("Authorization".into(),"Bearer borrowed-private-token".into())])};
    let capture=runner.begin_startup_diagnostics("oauth",&transport).await;
    let result=runner.verify_remote_transport_with_diagnostics(&transport,capture.clone()).await;
    capture.text("oauth callback","parent remains writable after both probes");
    let value=capture.snapshot().to_string();
    runner.shutdown().await;upstream.abort();let _=upstream.await;
    assert!(result.is_ok());assert!(value.contains("parent remains writable after both probes"));assert!(!value.contains("borrowed-private-token"));
}

#[tokio::test]
async fn authentication_admission_preserves_history_until_the_next_capture() {
    let runner=Runner::new();let config=transport();
    runner.configure(ServerRoute{server_id:"continuous".into(),transport:config.clone(),execution:ExecutionPolicy::default()}).await.unwrap();
    let capture=runner.begin_startup_diagnostics("continuous",&config).await;
    capture.line("initial startup output before a panel was opened");
    runner.begin_authentication("continuous").await.unwrap();
    assert!(runner.startup_diagnostics("continuous").await.unwrap().to_string().contains("initial startup output"));
    capture.line("late startup output must not append after admission");
    let next=runner.begin_startup_diagnostics("continuous",&config).await;next.line("authentication output");
    let output=runner.startup_diagnostics("continuous").await.unwrap().to_string();
    assert!(output.contains("initial startup output"));assert!(output.contains("authentication output"));
    assert!(!output.contains("late startup output"));
    runner.reopen_admission("continuous").await;runner.shutdown().await;
}

#[tokio::test]
async fn cumulative_history_retains_exit_once_redaction_and_the_current_run_limit() {
    use std::os::unix::process::ExitStatusExt;
    let runner=Runner::new();let config=TransportConfig::Http{endpoint:"https://fixture.example/mcp".into(),headers:BTreeMap::from([("Authorization".into(),"Bearer continuous-private".into())])};
    let first=runner.begin_startup_diagnostics("bounded",&config).await;
    first.line("old useful reason continuous-private");first.natural_exit(std::process::ExitStatus::from_raw(9<<8));
    for index in 0..2100 {first.line(&format!("first startup prefix {index:04}"));}
    let second=runner.begin_startup_diagnostics("bounded",&config).await;
    let inherited=second.snapshot();assert!(inherited["exit_code"].is_null());assert!(inherited["signal"].is_null());
    let inherited_text=inherited.to_string();assert_eq!(inherited_text.matches("Process exited with status 9.").count(),1);
    assert!(inherited_text.contains("old useful reason"));assert!(!inherited_text.contains("continuous-private"));
    for index in 0..2100 {second.line(&format!("auth repeated output {index:04}"));}
    let third=runner.begin_startup_diagnostics("bounded",&config).await;third.line("after the visible limit");
    let history=third.snapshot();let lines=history["lines"].as_array().unwrap();let output=history.to_string();
    assert!(!output.contains("first startup prefix 0000"));assert!(output.contains("Older console output was removed"));
    assert!(output.contains("after the visible limit"));assert_eq!(output.matches("Process exited with status 9.").count(),1);
    assert!(lines.len()<=4096);assert!(lines.iter().map(|line|line.as_str().unwrap().len()).sum::<usize>()<=1024*1024);
    runner.shutdown().await;
}
