defmodule SingularityCodeAnalysis do
  @moduledoc """
  Elixir NIF for comprehensive multi-language code analysis.

  This module provides access to the Rust singularity_code_analysis library NIF for:
  - Multi-language parsing (30+ languages via tree-sitter)
  - Code metrics and complexity analysis
  - AST analysis and introspection
  - Dependency extraction
  - Language detection
  - BEAM language support (Elixir, Erlang, Gleam)
  - AI-powered code insights

  ## Supported Languages

  - **BEAM**: Elixir, Erlang, Gleam
  - **Web**: JavaScript, TypeScript, JSX, TSX
  - **Systems**: Rust, C, C++, Go
  - **Data**: Python, Java
  - **Other**: Kotlin, C#, Lua

  ## Architecture

  singularity-code-analysis is a foundational library used by:
  - singularity-code-quality (metrics computation)
  - singularity-parser-engine (AST parsing)
  - Other analysis engines

  All use tree-sitter for parsing with Rust performance + Elixir integration.
  """

  use Rustler, otp_app: :singularity_code_analysis, crate: :singularity_code_analysis

  @doc """
  Analyze source code and extract function spaces with metrics.

  ## Parameters
  - `source_code`: The source code as a binary/string
  - `language`: The programming language atom (e.g., :rust, :python, :elixir)
  - `filename`: Optional filename for context
  - `options`: Analysis options (optional)

  ## Returns
  - `{:ok, function_spaces}` on success containing parsed functions with metrics
  - `{:error, reason}` on failure

  ## Examples

      iex> code = "fn add(x, y) do x + y end"
      iex> SingularityCodeAnalysis.analyze_code(code, :elixir, "math.ex")
      {:ok, %{functions: [%{name: "add", ...}]}}
  """
  def analyze_code(_source_code, _language, _filename \\ nil, _options \\ []), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Parse source code and return abstract syntax tree.

  ## Parameters
  - `source_code`: The source code as a binary/string
  - `language`: The programming language atom
  - `options`: Parsing options (optional)

  ## Returns
  - `{:ok, ast}` on success
  - `{:error, reason}` on failure
  """
  def parse_code(_source_code, _language, _options \\ []), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Extract function definitions from source code.

  ## Parameters
  - `source_code`: The source code as a binary/string
  - `language`: The programming language atom

  ## Returns
  - `{:ok, functions}` on success containing list of function definitions
  - `{:error, reason}` on failure
  """
  def extract_functions(_source_code, _language), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Extract dependencies/imports from source code.

  ## Parameters
  - `source_code`: The source code as a binary/string
  - `language`: The programming language atom

  ## Returns
  - `{:ok, dependencies}` on success
  - `{:error, reason}` on failure
  """
  def extract_dependencies(_source_code, _language), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Detect the programming language of source code.

  ## Parameters
  - `source_code`: The source code as a binary/string
  - `filename`: Optional filename hint

  ## Returns
  - `{:ok, language}` on success (e.g., :rust, :python, :elixir)
  - `{:error, reason}` on failure
  """
  def detect_language(_source_code, _filename \\ nil), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Calculate code metrics (complexity, LOC, etc.) for source code.

  ## Parameters
  - `source_code`: The source code as a binary/string
  - `language`: The programming language atom

  ## Returns
  - `{:ok, metrics}` on success
  - `{:error, reason}` on failure
  """
  def calculate_metrics(_source_code, _language), do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Get list of supported programming languages.

  ## Returns
  - List of supported language atoms
  """
  def supported_languages, do: :erlang.nif_error(:nif_not_loaded)

  @doc """
  Get version information.

  ## Returns
  - Version string
  """
  def version, do: :erlang.nif_error(:nif_not_loaded)
end
