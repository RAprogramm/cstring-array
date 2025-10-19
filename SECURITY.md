<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
SPDX-License-Identifier: CC0-1.0
-->

# Security Policy

## Supported Versions

We actively support the following versions of cstring-array with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take the security of cstring-array seriously. If you discover a security vulnerability, please follow these steps:

### 1. Do NOT Open a Public Issue

Please **do not** open a public GitHub issue for security vulnerabilities, as this could put users at risk before a fix is available.

### 2. Contact Us Privately

Report security vulnerabilities by emailing:

**Email**: andrey.rozanov.vl@gmail.com

Include in your report:
- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Suggested fix (if available)
- Your contact information

### 3. Response Timeline

- **Initial Response**: Within 48 hours
- **Vulnerability Assessment**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: Within 7 days
  - High: Within 14 days
  - Medium: Within 30 days
  - Low: Next release

### 4. Disclosure Policy

We follow coordinated vulnerability disclosure:

1. We will confirm receipt of your report
2. We will investigate and assess the vulnerability
3. We will develop and test a fix
4. We will release a security update
5. We will publicly disclose the vulnerability after the fix is released

You will receive credit for the discovery unless you prefer to remain anonymous.

## Security Best Practices

When using cstring-array:

### Safe Usage

- Always handle `Result` types properly
- Validate input strings for null bytes before conversion
- Use RAII pattern - arrays are automatically cleaned up
- Avoid manual pointer manipulation

### Unsafe Code

This crate uses `unsafe` code for FFI interoperability:
- All unsafe blocks are carefully reviewed
- Safety invariants are documented
- Pointer validity is guaranteed through RAII

### Dependencies

- This crate has **zero runtime dependencies**
- Development dependencies are audited regularly
- CI/CD pipeline includes `cargo audit` security checks

## Security Features

### Memory Safety

- RAII-based lifetime management prevents dangling pointers
- Null termination guaranteed for C compatibility
- Automatic cleanup prevents memory leaks
- Safe abstractions over raw pointers

### Testing

- 95%+ test coverage
- Integration tests for FFI scenarios
- Property-based testing for edge cases
- Cross-platform testing (Linux, macOS, Windows)

### CI/CD Security

Our continuous integration includes:
- Dependency vulnerability scanning (`cargo audit`)
- REUSE compliance checking
- Cross-platform testing
- Code coverage analysis

## Known Limitations

### Not Thread-Safe by Design

`CStringArray` is `Send + Sync` but requires external synchronization for concurrent access. This is by design for FFI compatibility.

### Interior Null Bytes

Creating arrays from strings containing null bytes (`\0`) will return an error. This is expected behavior for C string compatibility.

## Security Updates

Security updates are released as:
- Patch versions for fixes (0.1.x)
- Documented in CHANGELOG.md
- GitHub Security Advisories
- crates.io release notes

Subscribe to releases on GitHub to receive security update notifications.

## Audit History

| Date       | Auditor | Scope | Result |
|------------|---------|-------|--------|
| 2025-10-18 | Internal | v0.1.0 | No issues |

## Contact

For security concerns: andrey.rozanov.vl@gmail.com
For general issues: https://github.com/RAprogramm/cstring-array/issues

## Acknowledgments

We appreciate responsible disclosure of security vulnerabilities. Contributors will be acknowledged in:
- Security advisories
- Release notes
- CHANGELOG.md

Thank you for helping keep cstring-array secure!
