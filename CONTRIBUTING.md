# Contributing to singularity-code-analysis

Thank you for your interest in contributing to singularity-code-analysis! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/Singularity-ng/singularity-analysis.git
cd singularity-analysis

# Build the project
cargo build

# Run tests
cargo test --lib

# Check code quality
cargo fmt --check
cargo clippy
```

## Development Workflow

### Making Changes

1. **Create a feature branch**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make your changes**
   - Follow Rust conventions and best practices
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**
   ```bash
   # Run all tests
   cargo test --lib
   
   # Run specific tests
   cargo test --lib test_name
   ```

4. **Format and lint**
   ```bash
   # Format code
   cargo fmt
   
   # Run linter
   cargo clippy --lib
   ```

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: Add new feature"
   ```
   
   Use conventional commit messages:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes
   - `test:` for test changes
   - `refactor:` for refactoring
   - `chore:` for maintenance

6. **Push and create a pull request**
   ```bash
   git push origin feature/my-feature
   ```

## Code Style

- Follow standard Rust conventions (use `rustfmt`)
- Write descriptive variable and function names
- Add doc comments to public APIs using `///`
- Keep functions focused and reasonably sized
- Use meaningful commit messages

## Testing

- Write unit tests for new functions in the same file using `#[cfg(test)]` modules
- Use snapshot tests with `insta` for complex output verification
- Ensure all tests pass before submitting a PR
- Aim for good test coverage of new code

## Documentation

- Add doc comments to all public APIs
- Update README.md if adding major features
- Keep examples up to date
- Document any breaking changes in CHANGELOG.md

## Pull Request Guidelines

- Ensure CI passes (tests, formatting, linting)
- Provide a clear description of the changes
- Reference any related issues
- Keep PRs focused on a single feature or fix
- Respond to review comments promptly

## Adding Language Support

When adding support for a new programming language:

1. Add the tree-sitter grammar dependency to `Cargo.toml`
2. Update the `mk_langs!` macro in `src/langs.rs`
3. Implement metrics for the language (at minimum: cognitive, cyclomatic, halstead)
4. Add test cases in the relevant metric test files
5. Update the README.md supported languages table

## Reporting Issues

- Use GitHub Issues for bug reports and feature requests
- Provide a clear description and reproduction steps for bugs
- Include version information and environment details
- Search existing issues before creating new ones

## Security

- Do NOT open public issues for security vulnerabilities
- Report security issues privately to the maintainers
- See SECURITY.md for details

## Getting Help

- Check existing documentation in `docs/`
- Review closed issues and PRs for similar questions
- Ask questions in GitHub Discussions (if enabled)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
