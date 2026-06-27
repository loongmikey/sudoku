# Contributing to 数独 · Sudoku

Thank you for considering contributing! We welcome bug reports, feature requests, and pull requests.

## Getting Started

1. Fork the repository
2. Create a new branch for your feature/fix: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run Clippy: `cargo clippy -- -D warnings`
6. Format code: `cargo fmt`
7. Push and open a Pull Request

## Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use 4-space indentation
- Keep functions focused and small
- Write doc comments for public items
- Avoid `unsafe` code unless absolutely necessary

## Testing

- All new features should include unit tests
- Run `cargo test` before submitting
- Ensure no Clippy warnings with `cargo clippy -- -D warnings`

## Pull Request Process

1. Ensure all CI checks pass
2. Update the README.md if your change affects the public API or adds features
3. Update `Cargo.toml` version if needed
4. Your PR will be reviewed by a maintainer

## Code of Conduct

All contributors must adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).
