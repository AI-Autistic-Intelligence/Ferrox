import json
import random

def generate_synthetic_telemetry(num_samples=500):
    """Generates synthetic HTTP request telemetry dataset for training."""
    benign_uris = [
        "/api/v1/health", "/api/v1/users", "/api/v1/burraco/tables",
        "/api/version", "/assets/cloth1.png", "/assets/back2.png"
    ]
    malicious_payloads = [
        "/api/v1/search?q=' UNION SELECT CHAR(45,45) --",
        "/api/v1/users?id=1 OR 1=1; DROP TABLE users",
        "/api/v1/auth?token=<script>alert('XSS')</script>",
        "/api/v1/debug/env?dump=true"
    ]

    dataset = []
    
    # Benign samples (label = 0)
    for _ in range(int(num_samples * 0.8)):
        uri = random.choice(benign_uris)
        body = json.dumps({"action": "ping", "user_id": random.randint(100, 999)})
        dataset.append((uri, body, 0))

    # Malicious samples (label = 1)
    for _ in range(int(num_samples * 0.2)):
        uri = random.choice(malicious_payloads)
        body = json.dumps({"payload": "' UNION SELECT * FROM users--"})
        dataset.append((uri, body, 1))

    random.shuffle(dataset)
    return dataset

if __name__ == "__main__":
    ds = generate_synthetic_telemetry(100)
    print(f"Generated {len(ds)} synthetic HTTP telemetry records for training.")
