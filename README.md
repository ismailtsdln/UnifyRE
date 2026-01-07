# UnifyRE 🚀

Next-generation, CLI-based reverse engineering and binary analysis tool.

## Overview

UnifyRE is a high-performance tool designed to unify static analysis, dynamic debugging, and automation. Built with Rust, it offers a professional-grade platform for binary security audits.

## Phase 2 Highlights (Professionalization & Intelligence)

- **Hardened Architecture**: Trait-based abstractions for massive extensibility.
- **Smart Intelligence**: Built-in Shannon entropy calculation and suspicious sequence detection (NOP sleds, etc.).
- **Plugin System**: Dynamic loading of shared libraries for custom analysis logic.
- **Headless Automation**: Orchestrate complex workflows using `.ure` scripts.
- **Standalone HTML Dashboards**: Professional reporting with modern aesthetics.

## Phase 3 Highlights (Productization & Authority)

- **Determinism**: Guaranteed reproducible results via stable address-based sorting.
- **Binary Comparison**: Advanced `diff` engine for sections, symbols, and entropy.
- **Trust Layer**: Built-in knowledge base (via `explain`) for technical deep-dives.
- **Analysis Profiles**: Tailor heuristics for `malware`, `exploit`, or `audit` scenarios.

## Breaking Changes Policy

As UnifyRE approaches v1.0, we adhere to the following stability guarantees:
1. **Semantic Versioning**: We use SEMVER for all releases.
2. **CLI Stability**: CLI arguments and flags are considered stable in minor versions (1.x).
3. **JSON Output**: The JSON schema is strictly versioned. Breaking changes will only occur in major releases.
4. **Plugin ABI**: The internal trait system is currently in a "Locked" candidate state for v1.0.

## Advanced Usage

### Generate an HTML Security Report

```bash
unifyre report /bin/ls --out report.html --html
```

### Run an Automation Script

```bash
unifyre run examples/security_audit.ure /bin/ls
```

### Pattern Scanning with Suspicious Detection

```bash
unifyre scan patterns /bin/ls --pattern 90909090
```

## Plugin Development

UnifyRE supports dynamic plugins. Implement the `AnalyzerComponent` trait and export the constructor using `declare_plugin!`.

```rust
use unifyre::core::traits::AnalyzerComponent;
use unifyre::declare_plugin;

struct MyPlugin;
impl AnalyzerComponent for MyPlugin {
    fn name(&self) -> &str { "MyPlugin" }
    fn run(&self, provider: &dyn BinaryProvider) -> Result<serde_json::Value> {
        // Custom logic
    }
}

declare_plugin!(MyPlugin, MyPlugin::new);
```

## Installation

```bash
cargo build --release
```

## License

MIT License
