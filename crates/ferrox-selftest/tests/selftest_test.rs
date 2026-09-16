use ferrox_selftest::{AuditConfig, WstgAuditor};
use axum::{routing::get, Router};

#[tokio::test]
async fn test_wstg_auditor_against_mock_server() {
    // 1. Setup mock Axum server with standard security headers
    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::STRICT_TRANSPORT_SECURITY,
            axum::http::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 2. Run WstgAuditor
    let config = AuditConfig {
        target_url: format!("http://{}", addr),
        timeout_secs: 2,
        verbose: true,
    };

    let auditor = WstgAuditor::new(config);
    let report = auditor.run_all().await;

    assert_eq!(report.target_url, format!("http://{}", addr));
    assert!(report.total_tests > 0);
}
