# TypeCT Contribution Guidelines (CONTRIBUTING)

Thank you for contributing to `TypeCT`!
Please review the guidelines below before submitting pull requests or issues.

---

## 1. Development Workflow

1. Fork the repository.
2. Create a topic branch (`git checkout -b feature/my-feature`).
3. Make your modifications, adhering to privacy safeguards and style guidelines.
4. Run all verification checks:
   ```powershell
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```
5. Commit using clear, descriptive commit messages, push, and submit a Pull Request.

---

## 2. Standards & Quality

- **Rust Best Practices**: All code must pass `cargo clippy` and `cargo fmt`.
- **RustDoc Documentation**: Document all public types and functions with practical examples (`/// ```rust ...`).
- **Strict Privacy**: PRs introducing textual buffering, telemetry, or network transmission will be rejected.
- **Markdown Tables**: Maintain strict vertical alignment of `|` and `-` in documentation.
