# Contributing to MAESMA

Thank you for your interest in contributing to the Modular Agentic Earth System Model Architecture (MAESMA). This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- **Rust nightly** (2024 edition) — install via [rustup](https://rustup.rs)
- **Node.js 20+** — for the dashboard and animation projects
- **Git** — for version control

### Building

```bash
# Clone the repository
git clone https://github.com/admercs/MAESMA.git
cd MAESMA

# Build the workspace
cargo build

# Run all tests
cargo test

# Run clippy (should produce zero warnings)
cargo clippy --workspace

# Format code
cargo fmt
```

### Project Structure

MAESMA is a Cargo workspace with 10 crates:

| Crate                  | Purpose                                          |
| ---------------------- | ------------------------------------------------ |
| `maesma-core`          | Domain types, traits, invariants                 |
| `maesma-knowledgebase` | SQLite-backed process knowledgebase              |
| `maesma-agents`        | 30 agent implementations                         |
| `maesma-compiler`      | SAPG conservation, closure, coupling validation  |
| `maesma-runtime`       | Simulation execution engine                      |
| `maesma-processes`     | 13 process family modules (R0–R1 fidelity rungs) |
| `maesma-inference`     | Neural inference abstraction                     |
| `maesma-federation`    | A2A federation client                            |
| `maesma-api`           | Axum REST + WebSocket server                     |
| `maesma-cli`           | CLI entry point                                  |

## How to Contribute

### Reporting Issues

- Use GitHub Issues to report bugs or request features
- Include reproduction steps, expected vs. actual behavior, and environment details
- For security vulnerabilities, please report privately (do not open a public issue)

### Submitting Changes

1. **Fork** the repository
2. **Create a branch** from `master` for your changes
3. **Make your changes** following the code style guidelines below
4. **Add tests** for any new functionality
5. **Run the full test suite** — all tests must pass
6. **Run clippy** — zero warnings required
7. **Format code** with `cargo fmt`
8. **Submit a pull request** with a clear description of the changes

### Code Style

- Follow standard Rust idioms and conventions
- Use `cargo fmt` for formatting (default settings)
- All code must pass `cargo clippy --workspace` with zero warnings
- Prefer descriptive names over abbreviations
- Use doc comments (`///`) for public APIs
- Add `#[cfg(test)]` module tests for new functionality

### Adding a New Process Family

1. Create a module in `crates/maesma-processes/src/<family>/mod.rs`
2. Implement `ProcessRunner` trait for R0 (minimum) and R1 rungs
3. Add the family variant to `ProcessFamily` enum in `maesma-core/src/families.rs`
4. Register in `create_runner()` and `create_default_runners()` in `maesma-processes/src/lib.rs`
5. Add coupling declarations in `coupling.rs`
6. Add tests

### Adding a New Agent

1. Create a module in `crates/maesma-agents/src/<agent_name>.rs`
2. Implement the `Agent` trait (async)
3. Add the role variant to `AgentRole` enum in `traits.rs`
4. Register in `lib.rs`
5. Add tests

### Commit Messages

Use clear, descriptive commit messages:

```
<scope>: <short description>

<optional body explaining what and why>
```

Examples:
- `processes: add geomorphology R0/R1 erosion models`
- `agents: implement active learning experiment selection`
- `core: add conservation check for sediment mass`

## Testing

- All PRs must pass the full test suite (`cargo test --workspace`)
- All PRs must pass clippy with zero warnings (`cargo clippy --workspace`)
- Add unit tests for new types and functions
- Add integration tests for cross-crate interactions

## License

By contributing to MAESMA, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE).
