# Changelog

All notable changes to singularity-code-analysis will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Moved development documentation to `docs/development/` directory
- Updated .gitignore to exclude test artifacts

## [0.1.0] - 2024-11-09

### Added
- Multi-language code analysis library supporting 11+ programming languages
- Support for Python, JavaScript, TypeScript, C++, Rust, Elixir, Erlang, Gleam, Lua, Kotlin, Java, Go, C#
- 7 comprehensive metrics engines:
  - Cognitive Complexity
  - Cyclomatic Complexity  
  - Halstead Metrics
  - Lines of Code (LOC)
  - Number of Arguments (NARGS)
  - Nominative Count (NOM)
  - Exit Path Analysis
- Tree-sitter 0.25.10 integration for fast, incremental parsing
- Full trait system with Send + Sync bounds for thread safety
- Comprehensive test suite with 388+ passing unit tests
- MIT/Apache-2.0 dual licensing
- Complete API documentation and examples

### Known Limitations
- Some snapshot tests for cognitive complexity metrics show differences (under investigation)
- Boolean operator cognitive complexity counting may not be fully accurate in some languages
- C/C++ macro parsing has known limitations with Mozilla-style macros (tracked in tree-sitter-cpp #1142)
- LOC and exit counting metrics not yet implemented for: Kotlin, Go, C#, Elixir, Erlang, Gleam, Lua
