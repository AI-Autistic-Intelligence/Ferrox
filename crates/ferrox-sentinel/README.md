# 🛡️ Ferrox Sentinel (`ferrox-sentinel`)

`ferrox-sentinel` is the AI/ML Security Engine and Threat Analytics primitive for Ferrox Enterprise backends and the Founder Security Ecosystem (`security.ferrox-rust.dev`).

It features real-time feature extraction (Shannon Entropy, Feature Hashing Trick, character N-grams), velocity Z-score anomaly tracking, MinHash LSH campaign attribution, Isolation Forest scoring, **10 SOTA literature-backed security innovations**, and the **Autonomous Client Self-Onboarding Engine**.

---

## 🏛️ 10 SOTA Literature Innovations

1. **Moving Target Defense (MTD)** (*IEEE S&P*): Time-windowed ($T_{\text{rotate}} = 60\text{s}$) seed mutation & dynamic port offsets (`base_port + offset`).
2. **Distributed Honeynet Deception Mesh** (*USENIX Security*): Cross-node trap sharing & sub-second global shadow-bans.
3. **Self-Healing Micro-State Hot-Swap** (*ACM SIGSOFT*): Zero-downtime state snapshot attestation & hot-swap rollback.
4. **Zero-Knowledge Proof Burraco Attestation** (*IACR Cryptology*): Succinct ZK-SNARK game-rule verification (`BurracoZkProofPayload`).
5. **eBPF/XDP Kernel-Level Filter Generator** (*ACM SIGCOMM*): C-source XDP eBPF bytecode and nftables driver-layer packet drop rules.
6. **Behavioral Biometrics & Bot Cadence Detector** (*NDSS*): Inter-keystroke interval (IKI) and micro-cadence variance ($\sigma^2_{\text{jitter}}$) scoring to detect headless browser bots.
7. **Hidden Markov Model Sequence Predictor** (*ACM CCS*): State transition probability matrices $P(S_{t+1} \mid S_t)$ over client endpoint traversal paths.
8. **Local Differential Privacy Aggregator** (*EuroS&P*): Injects Laplacian noise $\text{Lap}(\frac{\Delta f}{\epsilon})$ into client metrics to guarantee data privacy.
9. **Deterministic Lockstep Replay Attestation** (*IEEE TDSC*): Multi-node lockstep hash verification to catch state tampering.
10. **Polymorphic API Route Mutation Engine** (*ACM SIGCOMM*): Ephemeral time-windowed HMAC path rotation for sensitive internal API endpoints.

---

## 🚀 Autonomous Client Self-Onboarding Engine

`AutonomousOnboardingEngine` automatically evaluates and registers client applications into the Founder Security Observation Hub:
- Evaluates `ClientOnboardingRequest` metadata, domain, IP, CISO email, and ZK guard attestation proofs.
- Verifies node is clean in `HoneynetMeshRegistry`.
- Issues unique `NodeId` and HMAC `NodeAuthToken`.
- Adds node to `FounderFleetRegistry` and `SecurityContactRegistry` for automated periodic active probe monitoring.
