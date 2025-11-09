# Security Policy

## Reporting Security Vulnerabilities

**DO NOT** open public GitHub issues for security vulnerabilities.

If you discover a security vulnerability in singularity-code-analysis, please report it privately to help us address it responsibly.

### How to Report

1. **Use GitHub's Private Vulnerability Reporting**
   - Go to the Security tab in the repository
   - Click "Report a vulnerability"
   - Fill out the form with details

2. **Or Contact Maintainers Directly**
   - Email the maintainers (check GitHub profiles for contact info)
   - Use encrypted communication if possible

### What to Include

When reporting a security issue, please include:

1. **Description**: Clear description of the vulnerability
2. **Impact**: What could an attacker accomplish?
3. **Reproduction**: Step-by-step instructions to reproduce
4. **Affected Versions**: Which versions are vulnerable?
5. **Suggested Fix**: If you have ideas for a fix (optional)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 1 week
- **Fix Timeline**: Depends on severity
  - Critical: Within days
  - High: Within 1-2 weeks
  - Medium: Within 1 month
  - Low: Next release cycle

## Security Best Practices

### For Users

1. **Keep Updated**: Use the latest version of singularity-code-analysis
2. **Review Dependencies**: Run `cargo audit` regularly
3. **Input Validation**: Validate and sanitize any user-provided source code paths
4. **Resource Limits**: Be aware of potential resource exhaustion on very large codebases

### For Contributors

1. **Code Review**: All changes require review before merging
2. **Dependency Audits**: New dependencies are vetted for security issues
3. **Minimal Unsafe Code**: Use `unsafe` only when necessary and document thoroughly
4. **Input Handling**: Never trust input from untrusted sources
5. **CI/CD**: Automated security checks run on all PRs

## Known Security Considerations

### Parser Safety

- Tree-sitter parsers are generally memory-safe and handle malformed input gracefully
- Extremely large files may cause high memory usage (not a security issue but resource concern)

### Unsafe Code

- This project contains minimal `unsafe` code
- All `unsafe` blocks are documented with SAFETY comments
- Located primarily in performance-critical parsing code

### Dependencies

- We regularly audit dependencies with `cargo audit`
- Dependencies are kept up-to-date with security patches
- Tree-sitter grammars are from trusted official sources

## Disclosure Policy

- We practice responsible disclosure
- Security fixes are released as soon as safely possible
- Security advisories are published after fixes are available
- Credit is given to reporters (unless they prefer anonymity)

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Features

- **Memory Safety**: Written in Rust for memory safety guarantees
- **No Eval**: No dynamic code execution from untrusted input
- **Sandboxed Parsing**: Tree-sitter parsers are sandboxed and handle errors gracefully
- **Thread Safety**: All public APIs are thread-safe (Send + Sync)

## Acknowledgments

We appreciate the security research community's efforts in helping keep this project secure.
