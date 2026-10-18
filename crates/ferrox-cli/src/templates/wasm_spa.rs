use std::path::Path;
use std::io;
use super::write_file;

pub fn scaffold(name: &str, target_dir: &Path) -> io::Result<()> {
    // 1. Ferrox.toml
    let ferrox_toml = format!(r#"[package]
name = "{name}"
version = "0.1.0"
template = "wasm-spa"
runtime = "wasm"
description = "Ferrox Reactive WebAssembly Single Page Application"

[frontend]
framework = "ferrox-front"
signals = true
target_fps = 60
dev_port = 4000

[quality]
min_coverage = 100
forbid_debug = true
"#);
    write_file(&target_dir.join("Ferrox.toml"), &ferrox_toml)?;

    // 2. Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = {{ version = "0.3", features = ["Window", "Document", "Element", "HtmlElement"] }}
js-sys = "0.3"
"#);
    write_file(&target_dir.join("Cargo.toml"), &cargo_toml)?;

    // 3. index.html
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name} • Ferrox WebAssembly</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css">
    <style>
        body {{ background: #090d16; color: #f8fafc; font-family: ui-sans-serif, system-ui, sans-serif; }}
    </style>
</head>
<body>
    <div id="app" class="flex min-h-screen flex-col items-center justify-center p-6 text-center">
        <h1 class="text-3xl font-bold text-indigo-400 mb-2">{name}</h1>
        <p class="text-slate-400">Pure Rust WebAssembly Reactive UI Running at 60 FPS</p>
    </div>
</body>
</html>
"#);
    write_file(&target_dir.join("index.html"), &index_html)?;

    // 4. Trunk.toml
    let trunk_toml = r#"[build]
target = "index.html"

[watch]
watch = ["src", "index.html"]

[serve]
port = 4000
open = false
"#;
    write_file(&target_dir.join("Trunk.toml"), trunk_toml)?;

    // 5. .coderabbit.yaml
    let coderabbit = r#"language: en-US
tone_instructions: "Enforce WebAssembly reactive performance: zero virtual DOM diffing, fine-grained signals, 60 FPS vector SVG."
"#;
    write_file(&target_dir.join(".coderabbit.yaml"), coderabbit)?;

    // 6. Dockerfile
    let dockerfile = r#"FROM nginx:alpine
COPY index.html /usr/share/nginx/html/index.html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
"#;
    write_file(&target_dir.join("Dockerfile"), dockerfile)?;

    // 7. docs/ARCHITECTURE.md
    let docs = format!(r#"# {name} - Reactive WebAssembly SPA Architecture

## Zero Virtual DOM Overhead
- **Fine-Grained Signals**: Updates bypass Virtual DOM diffing, manipulating the DOM nodes directly.
- **Vector Graphics**: Pure inline SVG rendering with sub-pixel crispness on Retina displays.
"#);
    write_file(&target_dir.join("docs/ARCHITECTURE.md"), &docs)?;

    // 8. Source code
    let state_rs = r#"//! Reactive State & Signal Management

#[derive(Clone, Debug, PartialEq)]
pub struct CounterSignal {
    count: i64,
}

impl CounterSignal {
    pub fn new(initial: i64) -> Self {
        Self { count: initial }
    }

    pub fn value(&self) -> i64 {
        self.count
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_signal_reactivity() {
        let mut signal = CounterSignal::new(0);
        assert_eq!(signal.value(), 0);
        signal.increment();
        assert_eq!(signal.value(), 1);
        signal.decrement();
        assert_eq!(signal.value(), 0);
    }
}
"#;
    write_file(&target_dir.join("src/state.rs"), state_rs)?;

    let lib_rs = format!(r#"pub mod state;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {{
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");
    if let Some(app) = document.get_element_by_id("app") {{
        app.set_inner_html("<h1 class='text-2xl font-bold text-emerald-400'>Ferrox Wasm SPA Initialized!</h1>");
    }}
    Ok(())
}}
"#);
    write_file(&target_dir.join("src/lib.rs"), &lib_rs)?;

    Ok(())
}
