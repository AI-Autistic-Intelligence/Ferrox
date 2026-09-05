---
sidebar_position: 1
---

# 🚀 Introduction

Welcome to **Ferrox**, a progressive Rust framework for building efficient, reliable, and scalable server-side applications.

Ferrox is built on top of [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/), bringing the developer experience and architectural patterns of frameworks like **NestJS** and **Spring Boot** to the Rust ecosystem.

## Philosophy

In recent years, thanks to Node.js and frameworks like NestJS, JavaScript has become the "lingua franca" of the web for both front and back-end applications. However, as applications scale to enterprise levels, developers often face performance bottlenecks, massive memory footprints, and single-threaded CPU limitations.

Rust solves all of these hardware and concurrency problems, but historically lacks the rapid *"Developer Experience (DX)"* that JavaScript developers love.

**Ferrox bridges this gap.**

It provides an out-of-the-box application architecture that allows developers and teams to create highly testable, scalable, loosely coupled, and easily maintainable applications. The architecture is heavily inspired by Angular/NestJS, promoting **Inversion of Control** and modularity.

## Zero-Trust by Default

Unlike other micro-frameworks where you start with a blank canvas, Ferrox assumes you are building an Enterprise application. 
Out of the box, it provides:
- **API Gateway Patterns**: PASETO JWT translation to internal headers.
- **Cache Stampede Protection**: Built-in `ferrox-singleflight` to protect your databases.
- **CQRS & Sagas**: Advanced patterns for distributed microservices.

## High-Level vs Low-Level

Ferrox is designed for both Junior developers and Senior Architects.
Throughout this documentation, you will find:
- **High-Level Usage**: Simple `#[derive]` macros and plug-and-play code snippets to get work done fast.
- **Low-Level Internals**: Deep dives into how Ferrox manipulates the Axum request lifecycle, allowing you to build your own custom Interceptors and Guards.

## 💬 Join the Community

Have questions or want to collaborate? Join the official **Ferrox Discord Community Server**:
👉 [https://discord.gg/Bx3CzGec7d](https://discord.gg/Bx3CzGec7d)

