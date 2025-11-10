# Agent / Automation Guidelines

## Approved Tasks
- Run Rust lint/test/build commands inside the devcontainer environment.
- Update documentation, workflows, and code within this repository.

## Restricted Tasks
- Do **not** modify dependency repositories (`~/.cargo/git` is mounted read-only).
- Do **not** push tags or publish releases without human approval.

## Environment
- Use `.devcontainer/devcontainer.json` when working via Copilot, Codespaces, or VS Code containers.
- GitHub Actions run in `mcr.microsoft.com/devcontainers/rust:1-bullseye`.
