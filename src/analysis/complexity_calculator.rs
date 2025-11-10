//! Language-aware complexity calculator.
//! Pure calculation functions for comprehensive code complexity analysis.
//! Elixir handles orchestration, state management, and database operations.

use crate::langs::LANG;

#[inline]
#[must_use]
fn usize_to_f64(value: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    {
        value as f64
    }
}

#[inline]
#[must_use]
fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        usize_to_f64(numerator) / usize_to_f64(denominator)
    }
}

/// Calculate comprehensive complexity score for a given language.
///
/// This replaces simple string-based calculations with heuristics that consider
/// multiple complexity dimensions.
#[must_use]
#[inline]
pub fn calculate_language_complexity_score(code: &str, language: LANG) -> f64 {
    let features = extract_complexity_features(code, language);

    // Weighted complexity calculation
    let structural_complexity = calculate_structural_complexity(&features);
    let cognitive_complexity = calculate_cognitive_complexity(&features);
    let maintainability_complexity = calculate_maintainability_complexity(&features);

    // AI-optimized weighting for learning
    (structural_complexity * 0.4 + cognitive_complexity * 0.4 + maintainability_complexity * 0.2)
        .min(10.0) // Cap at 10.0 for consistency
}

/// Extract complexity features from code
#[must_use]
#[inline]
pub fn extract_complexity_features(code: &str, language: LANG) -> ComplexityFeatures {
    let lines: Vec<&str> = code.lines().collect();
    let non_empty_lines = lines.iter().filter(|line| !line.trim().is_empty()).count();

    ComplexityFeatures {
        total_lines: lines.len(),
        non_empty_lines,
        function_count: count_patterns(code, &get_function_patterns(language)),
        control_flow_count: count_patterns(code, &get_control_flow_patterns(language)),
        nesting_depth: calculate_max_nesting_depth(code, language),
        operator_count: count_patterns(code, &get_operator_patterns(language)),
        comment_ratio: calculate_comment_ratio(code, language),
        identifier_length_avg: calculate_avg_identifier_length(code, language),
        cyclomatic_complexity: calculate_cyclomatic_complexity_estimate(code, language),
    }
}

/// Extract complexity features from code with custom patterns.
#[allow(clippy::too_many_arguments)]
#[must_use]
#[inline]
pub fn extract_complexity_features_with_patterns(
    code: &str,
    language: LANG,
    function_patterns: &[String],
    control_flow_patterns: &[String],
    operator_patterns: &[String],
    opening_delimiters: &[String],
    closing_delimiters: &[String],
    comment_patterns: &[String],
) -> ComplexityFeatures {
    let lines: Vec<&str> = code.lines().collect();
    let non_empty_lines = lines.iter().filter(|line| !line.trim().is_empty()).count();

    // Convert Vec<String> to Vec<&str> for compatibility
    let function_patterns_str: Vec<&str> = function_patterns.iter().map(String::as_str).collect();
    let control_flow_patterns_str: Vec<&str> =
        control_flow_patterns.iter().map(String::as_str).collect();
    let operator_patterns_str: Vec<&str> = operator_patterns.iter().map(String::as_str).collect();
    let opening_delimiters_str: Vec<&str> = opening_delimiters.iter().map(String::as_str).collect();
    let closing_delimiters_str: Vec<&str> = closing_delimiters.iter().map(String::as_str).collect();
    let comment_patterns_str: Vec<&str> = comment_patterns.iter().map(String::as_str).collect();

    ComplexityFeatures {
        total_lines: lines.len(),
        non_empty_lines,
        function_count: count_patterns(code, &function_patterns_str),
        control_flow_count: count_patterns(code, &control_flow_patterns_str),
        nesting_depth: calculate_max_nesting_depth_with_patterns(
            code,
            &opening_delimiters_str,
            &closing_delimiters_str,
        ),
        operator_count: count_patterns(code, &operator_patterns_str),
        comment_ratio: calculate_comment_ratio_with_patterns(code, &comment_patterns_str),
        identifier_length_avg: calculate_avg_identifier_length(code, language), // This doesn't need patterns
        cyclomatic_complexity: calculate_cyclomatic_complexity_estimate(code, language), // This doesn't need patterns
    }
}

/// Calculate structural complexity based on code organization
#[must_use]
#[inline]
pub fn calculate_structural_complexity(features: &ComplexityFeatures) -> f64 {
    let function_density = ratio(features.function_count, features.non_empty_lines);
    let nesting_factor = usize_to_f64(features.nesting_depth).powi(2) / 10.0;
    let operator_density = ratio(features.operator_count, features.non_empty_lines);

    (function_density * 2.0 + nesting_factor + operator_density * 1.5).min(5.0)
}

/// Calculate cognitive complexity based on mental effort required
#[must_use]
#[inline]
pub fn calculate_cognitive_complexity(features: &ComplexityFeatures) -> f64 {
    let control_flow_factor = usize_to_f64(features.control_flow_count) * 0.5;
    let nesting_factor = usize_to_f64(features.nesting_depth) * 0.8;
    let cyclomatic_factor = features.cyclomatic_complexity * 0.3;

    (control_flow_factor + nesting_factor + cyclomatic_factor).min(5.0)
}

/// Calculate maintainability complexity based on code quality indicators
#[must_use]
#[inline]
pub fn calculate_maintainability_complexity(features: &ComplexityFeatures) -> f64 {
    let comment_factor: f64 = if features.comment_ratio > 0.2 {
        0.5
    } else {
        2.0
    };
    let identifier_factor: f64 = if features.identifier_length_avg > 8.0 {
        0.5
    } else {
        1.5
    };
    let length_factor: f64 = if features.non_empty_lines > 100 {
        1.5
    } else {
        0.5
    };

    (comment_factor + identifier_factor + length_factor).min(5.0)
}

/// Count patterns in code using language-specific patterns
#[must_use]
#[inline]
pub fn count_patterns(code: &str, patterns: &[&str]) -> usize {
    patterns
        .iter()
        .map(|pattern| code.matches(pattern).count())
        .sum()
}

/// Get function definition patterns for a language
#[must_use]
#[inline]
pub fn get_function_patterns(language: LANG) -> Vec<&'static str> {
    match language {
        LANG::Elixir => vec!["def ", "defp ", "defmacro "],
        LANG::Rust => vec!["fn ", "async fn "],
        LANG::Python => vec!["def ", "async def "],
        LANG::Javascript | LANG::Typescript | LANG::Tsx => {
            vec!["function ", "=> ", "async function "]
        }
        LANG::Java => vec!["public ", "private ", "protected "],
        LANG::Cpp => vec!["void ", "int ", "bool ", "string ", "char ", "float "],
        LANG::Go => vec!["func "],
        LANG::Kotlin => vec!["fun ", "class ", "object "],
        LANG::Csharp => vec!["void ", "public ", "private ", "async "],
        LANG::Erlang => vec!["-spec ", "when "],
        LANG::Gleam => vec!["pub fn ", "fn "],
        LANG::Lua => vec!["function "],
    }
}

/// Get control flow patterns for a language
#[must_use]
#[inline]
pub fn get_control_flow_patterns(language: LANG) -> Vec<&'static str> {
    match language {
        LANG::Elixir => vec![
            "if ", "unless ", "case ", "cond ", "with ", "for ", "while ",
        ],
        LANG::Rust => vec!["if ", "match ", "while ", "for ", "loop "],
        LANG::Python => vec!["if ", "elif ", "else ", "for ", "while ", "try "],
        LANG::Javascript | LANG::Typescript | LANG::Tsx | LANG::Java | LANG::Cpp | LANG::Csharp => {
            vec!["if ", "else ", "for ", "while ", "switch ", "try "]
        }
        LANG::Go => vec!["if ", "else ", "for ", "switch "],
        LANG::Kotlin => vec!["if ", "else ", "for ", "while ", "when ", "try "],
        LANG::Erlang => vec!["case ", "if ", "receive "],
        LANG::Gleam => vec!["case ", "if ", "try "],
        LANG::Lua => vec!["if ", "elseif ", "for ", "while "],
    }
}

/// Get operator patterns for a language
#[must_use]
#[inline]
pub fn get_operator_patterns(language: LANG) -> Vec<&'static str> {
    match language {
        LANG::Elixir => vec!["&&", "||", "and", "or", "|>", "->", "=>"],
        LANG::Rust => vec!["&&", "||", "&", "|", "->", "=>"],
        LANG::Python => vec!["and", "or", "not", "in", "is"],
        LANG::Javascript | LANG::Typescript | LANG::Tsx => vec!["&&", "||", "!", "===", "!=="],
        LANG::Java | LANG::Cpp | LANG::Go | LANG::Gleam => vec!["&&", "||", "!", "==", "!="],
        LANG::Kotlin => vec!["&&", "||", "!", "==", "!=", "===", "!=="],
        LANG::Csharp => vec!["&&", "||", "!", "==", "!=", "??"],
        LANG::Erlang => vec!["and", "or", "not", "andalso", "orelse"],
        LANG::Lua => vec!["and", "or", "not"],
    }
}

/// Calculate maximum nesting depth in code
#[must_use]
#[inline]
pub fn calculate_max_nesting_depth(code: &str, language: LANG) -> usize {
    let mut max_depth = 0;
    let mut current_depth = 0;

    for line in code.lines() {
        let trimmed = line.trim();

        // Count opening braces/brackets
        current_depth += trimmed.matches(get_opening_patterns(language)).count();

        // Count closing braces/brackets
        current_depth =
            current_depth.saturating_sub(trimmed.matches(get_closing_patterns(language)).count());

        max_depth = max_depth.max(current_depth);
    }

    max_depth
}

/// Get opening patterns for nesting calculation
#[must_use]
#[inline]
pub fn get_opening_patterns(language: LANG) -> &'static str {
    match language {
        LANG::Python => ":",
        LANG::Erlang => "(",
        LANG::Lua => "do",
        _ => "{",
    }
}

/// Get closing patterns for nesting calculation
#[must_use]
#[inline]
pub fn get_closing_patterns(language: LANG) -> &'static str {
    match language {
        LANG::Python => "",
        LANG::Erlang => ")",
        LANG::Lua => "end",
        _ => "}",
    }
}

/// Calculate comment ratio in code
#[must_use]
#[inline]
pub fn calculate_comment_ratio(code: &str, language: LANG) -> f64 {
    let lines: Vec<&str> = code.lines().collect();
    let comment_patterns = get_comment_patterns(language);

    let comment_lines = lines
        .iter()
        .filter(|line| {
            let trimmed = line.trim();
            comment_patterns
                .iter()
                .any(|pattern| trimmed.starts_with(pattern))
        })
        .count();

    ratio(comment_lines, lines.len())
}

/// Get comment patterns for a language
#[must_use]
#[inline]
pub fn get_comment_patterns(language: LANG) -> Vec<&'static str> {
    match language {
        LANG::Elixir | LANG::Python => vec!["#"],
        LANG::Rust
        | LANG::Javascript
        | LANG::Typescript
        | LANG::Tsx
        | LANG::Java
        | LANG::Cpp
        | LANG::Go
        | LANG::Kotlin
        | LANG::Csharp => vec!["//", "/*"],
        LANG::Erlang => vec!["%"],
        LANG::Gleam => vec!["//"],
        LANG::Lua => vec!["--"],
    }
}

/// Calculate maximum nesting depth with custom patterns
#[must_use]
#[inline]
pub fn calculate_max_nesting_depth_with_patterns(
    code: &str,
    opening_patterns: &[&str],
    closing_patterns: &[&str],
) -> usize {
    let mut max_depth = 0;
    let mut current_depth = 0;

    for line in code.lines() {
        let trimmed = line.trim();

        // Count opening delimiters
        for pattern in opening_patterns {
            current_depth += trimmed.matches(pattern).count();
        }

        // Count closing delimiters
        for pattern in closing_patterns {
            current_depth = current_depth.saturating_sub(trimmed.matches(pattern).count());
        }

        max_depth = max_depth.max(current_depth);
    }

    max_depth
}

/// Calculate comment ratio with custom patterns
#[must_use]
#[inline]
pub fn calculate_comment_ratio_with_patterns(code: &str, comment_patterns: &[&str]) -> f64 {
    let lines: Vec<&str> = code.lines().collect();

    let comment_lines = lines
        .iter()
        .filter(|line| {
            let trimmed = line.trim();
            comment_patterns
                .iter()
                .any(|pattern| trimmed.starts_with(pattern))
        })
        .count();

    ratio(comment_lines, lines.len())
}

/// Calculate comprehensive complexity score with custom patterns
#[allow(clippy::too_many_arguments)]
#[must_use]
#[inline]
pub fn calculate_language_complexity_score_with_patterns(
    code: &str,
    language: LANG,
    function_patterns: &[String],
    control_flow_patterns: &[String],
    operator_patterns: &[String],
    opening_delimiters: &[String],
    closing_delimiters: &[String],
    comment_patterns: &[String],
) -> f64 {
    let features = extract_complexity_features_with_patterns(
        code,
        language,
        function_patterns,
        control_flow_patterns,
        operator_patterns,
        opening_delimiters,
        closing_delimiters,
        comment_patterns,
    );

    // Weighted complexity calculation
    let structural_complexity = calculate_structural_complexity(&features);
    let cognitive_complexity = calculate_cognitive_complexity(&features);
    let maintainability_complexity = calculate_maintainability_complexity(&features);

    // AI-optimized weighting for learning
    (structural_complexity * 0.4 + cognitive_complexity * 0.4 + maintainability_complexity * 0.2)
        .min(10.0) // Cap at 10.0 for consistency
}

/// Calculate average identifier length
#[must_use]
#[inline]
pub fn calculate_avg_identifier_length(code: &str, _language: LANG) -> f64 {
    let identifiers: Vec<&str> = code
        .split_whitespace()
        .filter(|word| word.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .collect();

    if identifiers.is_empty() {
        0.0
    } else {
        let total_length: usize = identifiers.iter().map(|id| id.len()).sum();
        ratio(total_length, identifiers.len())
    }
}

/// Calculate cyclomatic complexity estimate
#[must_use]
#[inline]
pub fn calculate_cyclomatic_complexity_estimate(code: &str, language: LANG) -> f64 {
    let control_flow_patterns = get_control_flow_patterns(language);
    let operator_patterns = get_operator_patterns(language);

    let control_flow_count = count_patterns(code, &control_flow_patterns);
    let operator_count = count_patterns(code, &operator_patterns);

    // Basic cyclomatic complexity: 1 + control flow + logical operators
    1.0 + usize_to_f64(control_flow_count) + (usize_to_f64(operator_count) * 0.5)
}

/// Complexity features extracted from code
#[derive(Debug, Clone)]
pub struct ComplexityFeatures {
    pub total_lines: usize,
    pub non_empty_lines: usize,
    pub function_count: usize,
    pub control_flow_count: usize,
    pub nesting_depth: usize,
    pub operator_count: usize,
    pub comment_ratio: f64,
    pub identifier_length_avg: f64,
    pub cyclomatic_complexity: f64,
}

/// Calculate pattern effectiveness for AI learning
#[must_use]
#[inline]
pub fn calculate_pattern_effectiveness(_pattern: &str, metrics: &ComplexityFeatures) -> f64 {
    // Pattern effectiveness based on complexity reduction
    let complexity_reduction = if metrics.cyclomatic_complexity > 5.0 {
        0.8
    } else {
        0.3
    };
    let maintainability_boost = if metrics.comment_ratio > 0.2 {
        0.9
    } else {
        0.4
    };
    let readability_score = if metrics.identifier_length_avg > 6.0 {
        0.7
    } else {
        0.5
    };

    (complexity_reduction + maintainability_boost + readability_score) / 3.0
}

/// Calculate supervision complexity for BEAM languages
#[must_use]
#[inline]
pub fn calculate_supervision_complexity(modules: &[String]) -> f64 {
    if modules.is_empty() {
        return 0.0;
    }

    let supervisor_count = modules
        .iter()
        .filter(|module| module.contains("Supervisor") || module.contains("supervisor"))
        .count();

    let genserver_count = modules
        .iter()
        .filter(|module| module.contains("GenServer") || module.contains("gen_server"))
        .count();

    (usize_to_f64(supervisor_count) * 0.5 + usize_to_f64(genserver_count) * 0.3).min(10.0)
}

/// Calculate actor complexity for BEAM languages
#[must_use]
#[inline]
pub fn calculate_actor_complexity(functions: &[String]) -> f64 {
    if functions.is_empty() {
        return 0.0;
    }

    let spawn_count = functions
        .iter()
        .filter(|func| func.contains("spawn") || func.contains("Task.async"))
        .count();

    let send_count = functions
        .iter()
        .filter(|func| func.contains("send") || func.contains("cast"))
        .count();

    let receive_count = functions
        .iter()
        .filter(|func| func.contains("receive") || func.contains("call"))
        .count();

    (usize_to_f64(spawn_count) * 0.4
        + usize_to_f64(send_count) * 0.3
        + usize_to_f64(receive_count) * 0.3)
        .min(10.0)
}
