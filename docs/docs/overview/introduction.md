---
sidebar_position: 1
---

# 🚀 Introduction

Welcome to **Ferrox** (v0.5.0), a progressive Rust framework for building efficient, reliable, enterprise-grade server-side applications.

Ferrox is built on top of [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/), bringing the developer experience and architectural patterns of frameworks like **NestJS** and **Spring Boot** to the Rust ecosystem.

---

## Philosophy

In recent years, JavaScript/TypeScript and frameworks like NestJS have become dominant for web backends. However, as applications scale to enterprise levels, developers often face performance bottlenecks, massive memory footprints, and single-threaded CPU limitations.

Rust solves all of these concurrency and memory problems, but historically lacked the rapid *"Developer Experience (DX)"* that web developers love.

**Ferrox bridges this gap.**

It provides an out-of-the-box application architecture that allows developers and teams to create highly testable, scalable, loosely coupled, and easily maintainable applications with Angular/NestJS-style **Inversion of Control** and modularity.

---

## 🤖 35 SOTA Literature Innovations & Zero-Trust Mesh

Starting in version `0.5.0`, Ferrox features **Ferrox Sentinel** (`ferrox-sentinel`) and **Ferrox Selftest** (`ferrox-selftest`) — an exhaustive 7-tier AI/ML security mesh implementing **35 SOTA Literature Innovations** synthesized from 11 authoritative technical security and AI publications:

- **AI & Multi-Agent Cognitive Security**: Direct/indirect prompt injection sanitizer, ChatML delimiter stripping, RAG hallucination groundedness scoring, and AI agent tool sandboxing.
- **Host & EDR Anti-Evasion Watchdog**: LSASS credential dumping guard (`ObRegisterCallbacks`), ETW-TI direct syscall trampoline interception, ETW NOPing patch detection, and unbacked executable memory execution protection.
- **Deep Packet Inspection & Network Mesh**: Raw magic-byte signature matching for non-standard ports (gRPC, TLS, SSH, Modbus/ICS), DNS TXT exfiltration tunneling detection, NetFlow volume asymmetry, and Gratuitous ARP traps.
- **Zero-Trust & Cryptographic Core**: Double Ratchet forward secrecy, Post-Quantum Kyber ML-KEM lattice key encapsulation, Yao's Garbled Circuits 2PC confidential scoring, ZK-SNARK Burraco game attestation, and Laplacian local differential privacy.
- **SOAR Auto-Remediation & MTD**: Ephemeral ingress port mutation (`T_rotate = 60s`), polymorphic API route rotation, automated `nftables` / XDP eBPF kernel packet drop dispatch.

---

## High-Level vs Low-Level

Ferrox is designed for both Junior developers and Senior Architects:
- **High-Level Usage**: Simple `#[derive]` macros and plug-and-play code snippets to get work done fast.
- **Low-Level Internals**: Deep dives into how Ferrox manipulates the Axum request lifecycle, allowing you to build custom Interceptors, Guards, and Sentinel algorithms.

---

## 💬 Join the Community

Have questions or want to collaborate? Join the official **Ferrox Community**:
- 💬 **Discord Server:** [https://discord.gg/Bx3CzGec7d](https://discord.gg/Bx3CzGec7d)
- 🤖 **Reddit Subreddit:** [r/Ferrox](https://www.reddit.com/r/Ferrox/)
