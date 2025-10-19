<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: CC0-1.0
-->

# Contributing to cstring-array

Thank you for your interest in contributing to cstring-array! This document provides guidelines and instructions for contributing to this project.

## Development Setup

### Prerequisites

- Rust 1.90 or later (Edition 2024)
- cargo, rustfmt, clippy
- Git

### Getting Started

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/cstring-array.git
   cd cstring-array
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/RAprogramm/cstring-array.git
   ```
4. Install development dependencies:
   ```bash
   cargo build --all-features
   cargo test --all-features
   ```

## Code Style

This project follows strict coding standards:

### Rust Edition and Version

- **Edition**: 2024
- **Minimum Rust Version**: 1.90

### Code Formatting

- Use `rustfmt` for all Rust code
- Run before committing:
  ```bash
  cargo +nightly fmt
  ```

### Linting

- All code must pass clippy with no warnings:
  ```bash
  cargo clippy --all-targets --all-features -- -D warnings
  ```

### Documentation

- All public APIs must have documentation comments (`///`)
- Module-level documentation is required
- Use `::` only in imports
- Avoid inline comments except for `///` doc comments

### Code Structure

- Follow RAII patterns for resource management
- Prefer zero-copy operations where possible
- Write safe code - `unsafe` requires detailed justification

## Testing Requirements

### Test Coverage

- Maintain 95%+ test coverage
- All new features must include tests
- Bug fixes must include regression tests

### Running Tests

```bash
# Run all tests
cargo test --all-features

# Run tests with coverage
cargo install cargo-llvm-cov
cargo llvm-cov --all-features

# Run doctests
cargo test --all-features --doc

# Run benchmarks
cargo bench
```

### Test Organization

- Unit tests in module files (mod tests)
- Integration tests in tests/ directory
- Doctests for usage examples
- Benchmarks in benches/ directory

## Pull Request Process

### Before Submitting

1. Update to latest upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Run full test suite:
   ```bash
   cargo +nightly fmt -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-features
   cargo build --release --all-features
   ```

3. Update documentation if needed
4. Update CHANGELOG.md (auto-generated from commits)

### Commit Messages

Follow conventional commits format:

- `feat: add new feature` - New feature
- `fix: resolve bug` - Bug fix
- `docs: update documentation` - Documentation changes
- `perf: improve performance` - Performance improvements
- `refactor: restructure code` - Code refactoring
- `test: add tests` - Test additions
- `chore: maintenance task` - Chores and maintenance

### PR Guidelines

1. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the code style above

3. Commit with conventional commit messages

4. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

5. Create a pull request on GitHub with:
   - Clear description of changes
   - Reference to related issues
   - Screenshots/examples if applicable

### CI/CD Pipeline

All PRs must pass:
- Formatting check (rustfmt)
- Linting (clippy)
- REUSE compliance
- Security audit (cargo audit)
- Tests (Linux, macOS, Windows × stable, beta, nightly)
- Coverage check
- Documentation build

## Licensing

This project uses:
- **Code**: MIT License
- **Configuration files**: CC0-1.0 (Public Domain)

All contributions must comply with [REUSE Specification 3.3](https://reuse.software/spec/).

### Adding License Headers

For new Rust files:
```rust
// SPDX-FileCopyrightText: 2025 YourName <your@email.com>
// SPDX-License-Identifier: MIT
```

For new config files:
```yaml
# SPDX-FileCopyrightText: 2025 YourName <your@email.com>
# SPDX-License-Identifier: CC0-1.0
```

## Getting Help

- Open an issue for bugs or feature requests
- Check existing issues and discussions
- Read the documentation at [docs.rs/cstring-array](https://docs.rs/cstring-array)

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on technical merits
- Welcome newcomers

Thank you for contributing to cstring-array!
