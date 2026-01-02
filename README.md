# UnifyRE 🚀

Next-generation, CLI-based reverse engineering and binary analysis tool.

## Overview

UnifyRE is a high-performance tool designed to unify static analysis, dynamic debugging, and automation into a single, coherent workflow. Built with Rust for memory safety and speed, it provides security engineers with a professional-grade tool for binary inspection.

## Features

- **Multi-format Support**: ELF, PE, and Mach-O support via a unified interface.
- **Static Analysis**: Extract sections, symbols, architecture details, and entry points.
- **Disassembly Engine**: Powered by Capstone for accurate instruction decoding across x86, x64, ARM, and ARM64.
- **Pattern Scanning**: High-speed hex pattern matching in binary data.
- **Structured Reporting**: Human-readable tables and machine-readable JSON output.
- **Automated Intelligence**: Quick report generation for automated pipelines.

## Installation

### From Source

Ensure you have Rust installed (1.80+ recommended).

```bash
git clone https://github.com/ismailtsdln/UnifyRE.git
cd UnifyRE
cargo build --release
```

### Quick Start

#### Analyze a binary

```bash
unifyre analyze /bin/ls --format human
```

#### Disassemble entry point

```bash
unifyre disasm /bin/ls --entry
```

#### Scan for patterns

```bash
unifyre scan patterns /bin/ls --pattern 4889e5
```

#### Generate JSON report

```bash
unifyre report /bin/ls --out report.json
```

## Command Reference

| Command | Description |
|---------|-------------|
| `analyze` | Perform static analysis |
| `disasm` | Disassemble instructions |
| `debug` | Start a dynamic debugging session (WIP) |
| `scan` | Advanced pattern and signature scanning |
| `report` | Generate comprehensive reports |

## Error Handling Philosophy

UnifyRE is built to be fail-safe. It uses structured error handling to ensure that malformed binaries or permission issues result in clear, human-readable error messages rather than crashes or panics.

## License

This project is licensed under the MIT License.
