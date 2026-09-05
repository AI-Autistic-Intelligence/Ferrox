---
sidebar_position: 2
---

# 🔬 CPU & Memory Profiling (Flamegraphs)

Identifying performance bottlenecks, CPU hot paths, and unexpected heap memory allocations in production systems requires low-overhead profiling tools.

This guide details generating CPU **Flamegraphs** and profiling memory allocations in Rust.

---

## 1. Generating Flamegraphs with `cargo-flamegraph`

Flamegraphs visualize CPU stack traces. The wider a function frame appears on the horizontal axis, the more CPU time it consumed.

### Installation

```bash
cargo install flamegraph
```

### Running Profiler

Enable debug symbols in release mode inside `Cargo.toml`:

```toml
[profile.release]
debug = true
```

Run flamegraph generator while applying load with `wrk` or `ab`:

```bash
cargo flamegraph --bin ferrox-showcase
```

Output: A vector SVG graph `flamegraph.svg` highlighting exact CPU hot spots down to line numbers.

---

## 2. Heap Profiling with `heaptrack` or `valgrind`

To detect memory leaks or excessive buffer allocations:

```bash
heaptrack ./target/release/ferrox-showcase
```

Launch heaptrack GUI to inspect allocations:

```bash
heaptrack_gui heaptrack.ferrox-showcase.12345.gz
```
