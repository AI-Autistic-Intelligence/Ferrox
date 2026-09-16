use ferrox_sentinel::{SentinelEngine, SentinelConfig, ThreatLevel};
use ferrox_sentinel::features::entropy::{calculate_shannon_entropy, evaluate_payload_entropy};
use ferrox_sentinel::features::hashing_trick::FeatureHasher;
use ferrox_sentinel::algorithms::minhash_lsh::LshClusterIndex;

#[test]
fn test_shannon_entropy_calculation() {
    let benign = "http://localhost:8080/api/v1/users";
    let malicious = "http://localhost:8080/api/v1/search?q=%27%20UNION%20SELECT%20CHAR(45,45,45),CHAR(45,45,45)%20--%20";

    let entropy_benign = calculate_shannon_entropy(benign.as_bytes());
    let entropy_malicious = calculate_shannon_entropy(malicious.as_bytes());

    assert!(entropy_benign > 0.0);
    assert!(entropy_malicious > entropy_benign);

    let eval_benign = evaluate_payload_entropy(benign);
    let eval_malicious = evaluate_payload_entropy(malicious);

    assert_eq!(eval_benign.is_suspicious, 0.0);
    assert!(eval_malicious.is_suspicious > 0.5);
}

#[test]
fn test_feature_hashing_trick() {
    let hasher = FeatureHasher::new(1024);
    let features = vec!["GET", "/api/v1/auth/login", "User-Agent: Mozilla/5.0"];

    let vec1 = hasher.transform(&features);
    let vec2 = hasher.transform(&features);

    assert_eq!(vec1.num_buckets, 1024);
    assert_eq!(vec1.buckets.len(), 1024);
    assert_eq!(vec1.buckets, vec2.buckets);
}

#[test]
fn test_minhash_lsh_campaign_attribution() {
    let mut lsh = LshClusterIndex::new(5, 4);

    let features_ip1 = vec!["/api/v1/login".to_string(), "UNION".to_string(), "SELECT".to_string()];
    let features_ip2 = vec!["/api/v1/login".to_string(), "UNION".to_string(), "SELECT".to_string()];

    lsh.insert("192.168.1.100", &features_ip1);
    lsh.insert("192.168.1.101", &features_ip2);

    let matches = lsh.find_similar(&features_ip1);
    assert!(matches.contains("192.168.1.100"));
    assert!(matches.contains("192.168.1.101"));
}

#[test]
fn test_sentinel_engine_threat_assessment() {
    let engine = SentinelEngine::new(SentinelConfig::default());

    let benign_assessment = engine.evaluate_request(
        "127.0.0.1",
        "/api/v1/health",
        "Host: localhost",
        "",
    );
    assert_eq!(benign_assessment.level, ThreatLevel::Benign);

    let sqli_payload = "/api/v1/users?id=1%20UNION%20SELECT%20username,%20password%20FROM%20users--";
    let malicious_assessment = engine.evaluate_request(
        "10.0.0.50",
        sqli_payload,
        "User-Agent: sqlmap/1.5",
        "UNION SELECT ALL",
    );

    assert!(malicious_assessment.threat_score >= 0.65);
    assert!(malicious_assessment.level == ThreatLevel::HighRisk || malicious_assessment.level == ThreatLevel::CriticalAnomaly);
}
