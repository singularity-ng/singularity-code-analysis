//! Error Handling Coverage Score - Measures completeness of error handling paths
//!
//! Analyzes error declarations, unhandled exceptions, logging coverage, and fallback paths
//! to predict runtime stability and debuggability.

/// Error Handling Metrics
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorHandlingMetrics {
    /// Overall error handling coverage score (0-100)
    pub error_handling_score: f64,
    /// Percentage of declared error types that are caught
    pub error_type_coverage: f64,
    /// Percentage of unhandled exception paths
    pub unhandled_paths_ratio: f64,
    /// Ratio of specific catches vs generic catches
    pub specific_catches_ratio: f64,
    /// Percentage of code paths with logging
    pub logging_coverage: f64,
    /// Percentage of error paths with fallback/default
    pub fallback_path_coverage: f64,
    /// Total error handlers found
    pub error_handlers: usize,
    /// Generic catch blocks detected
    pub generic_catches: usize,
    /// Logging statements in error paths
    pub log_statements: usize,
}

pub struct ErrorHandlingInputs {
    pub error_type_coverage: f64,
    pub unhandled_paths_ratio: f64,
    pub specific_catches_ratio: f64,
    pub logging_coverage: f64,
    pub fallback_coverage: f64,
    pub error_handlers: usize,
    pub generic_catches: usize,
    pub log_statements: usize,
}

impl ErrorHandlingMetrics {
    /// Calculate error handling score using weighted formula:
    /// Score = (
    ///   0.3 * error_type_coverage +
    ///   0.25 * (1 - unhandled_paths) +
    ///   0.2 * specific_catches +
    ///   0.15 * logging_coverage +
    ///   0.1 * fallback_coverage
    /// ) * 100
    pub fn calculate(inputs: ErrorHandlingInputs) -> Self {
        let ErrorHandlingInputs {
            error_type_coverage,
            unhandled_paths_ratio,
            specific_catches_ratio,
            logging_coverage,
            fallback_coverage,
            error_handlers,
            generic_catches,
            log_statements,
        } = inputs;

        let error_handling_score = (0.3 * error_type_coverage
            + 0.25 * (1.0 - unhandled_paths_ratio)
            + 0.2 * specific_catches_ratio
            + 0.15 * logging_coverage
            + 0.1 * fallback_coverage)
            * 100.0;

        Self {
            error_handling_score: error_handling_score.clamp(0.0, 100.0),
            error_type_coverage: (error_type_coverage * 100.0).clamp(0.0, 100.0),
            unhandled_paths_ratio: unhandled_paths_ratio.clamp(0.0, 1.0),
            specific_catches_ratio: (specific_catches_ratio * 100.0).clamp(0.0, 100.0),
            logging_coverage: (logging_coverage * 100.0).clamp(0.0, 100.0),
            fallback_path_coverage: (fallback_coverage * 100.0).clamp(0.0, 100.0),
            error_handlers,
            generic_catches,
            log_statements,
        }
    }

    /// Analyze error handling in code
    pub fn from_code(code: &str, language: &str) -> Self {
        match language {
            "rust" => Self::analyze_rust_errors(code),
            "python" => Self::analyze_python_errors(code),
            "javascript" | "typescript" => Self::analyze_js_errors(code),
            "java" => Self::analyze_java_errors(code),
            _ => Self::analyze_generic_errors(code),
        }
    }

    /// Analyze error handling in code with custom patterns
    pub fn from_code_with_patterns(code: &str, error_patterns: &[String]) -> Self {
        Self::analyze_with_patterns(code, error_patterns)
    }
}

impl ErrorHandlingMetrics {
    /// Analyze Rust error handling (Result, Option, match, unwrap, expect)
    fn analyze_rust_errors(code: &str) -> Self {
        let error_handlers = code.matches("Result").count() + code.matches("Option").count();
        let match_handlers = code.matches("match ").count();
        let if_let_handlers = code.matches("if let ").count();
        let unwrap_calls = code.matches(".unwrap").count();
        let expect_calls = code.matches(".expect").count();
        let question_marks = code.matches("?").count();

        let specific_catches_ratio = (match_handlers + if_let_handlers + question_marks) as f64
            / (error_handlers as f64 + 1.0);

        let unhandled_paths = unwrap_calls + expect_calls;
        let unhandled_ratio =
            unhandled_paths as f64 / (match_handlers as f64 + if_let_handlers as f64 + 1.0);

        let log_statements = code.matches("error!").count()
            + code.matches("warn!").count()
            + code.matches("eprintln!").count();

        let fallback_coverage = question_marks as f64 / (error_handlers as f64 + 1.0);

        Self::calculate(ErrorHandlingInputs {
            error_type_coverage: (error_handlers as f64 / error_handlers.max(1) as f64)
                .clamp(0.0, 1.0),
            unhandled_paths_ratio: unhandled_ratio.clamp(0.0, 1.0),
            specific_catches_ratio: specific_catches_ratio.clamp(0.0, 1.0),
            logging_coverage: (log_statements as f64 / error_handlers.max(1) as f64)
                .clamp(0.0, 1.0),
            fallback_coverage: fallback_coverage.clamp(0.0, 1.0),
            error_handlers,
            generic_catches: 0,
            log_statements,
        })
    }

    /// Analyze error handling with custom patterns from registry
    fn analyze_with_patterns(code: &str, error_patterns: &[String]) -> Self {
        let mut error_handlers = 0;
        let mut try_blocks = 0;
        let mut catch_blocks = 0;
        let generic_catches = 0;
        let mut log_statements = 0;
        let mut unhandled_calls = 0;

        // Count patterns using the provided error patterns
        for pattern in error_patterns {
            match pattern.as_str() {
                "Result" | "Option" => error_handlers += code.matches(pattern).count(),
                "try" => try_blocks += code.matches(pattern).count(),
                "catch" => catch_blocks += code.matches(pattern).count(),
                "unwrap" | "expect" => unhandled_calls += code.matches(pattern).count(),
                "error!" | "warn!" | "eprintln!" => log_statements += code.matches(pattern).count(),
                _ => {} // Other patterns can be handled as needed
            }
        }

        // Calculate metrics
        let error_type_coverage: f64 = if error_handlers == 0 {
            0.0_f64
        } else {
            1.0_f64
        };
        let unhandled_ratio = if (try_blocks + catch_blocks) == 0 {
            0.0
        } else {
            unhandled_calls as f64 / (try_blocks + catch_blocks) as f64
        };
        let specific_catches_ratio: f64 = if catch_blocks == 0 { 0.0_f64 } else { 1.0_f64 };
        let logging_coverage = if error_handlers == 0 {
            0.0
        } else {
            log_statements as f64 / error_handlers as f64
        };
        let fallback_coverage = if try_blocks == 0 {
            0.0
        } else {
            catch_blocks as f64 / try_blocks as f64
        };

        Self::calculate(ErrorHandlingInputs {
            error_type_coverage: error_type_coverage.clamp(0.0, 1.0),
            unhandled_paths_ratio: unhandled_ratio.clamp(0.0, 1.0),
            specific_catches_ratio: specific_catches_ratio.clamp(0.0, 1.0),
            logging_coverage: logging_coverage.clamp(0.0, 1.0),
            fallback_coverage: fallback_coverage.clamp(0.0, 1.0),
            error_handlers,
            generic_catches,
            log_statements,
        })
    }

    /// Analyze Python error handling (try/except, raise, Optional)
    fn analyze_python_errors(code: &str) -> Self {
        let try_blocks = code.matches("try:").count();
        let except_blocks = code.matches("except").count();
        let generic_excepts = code.matches("except:").count();
        let specific_excepts = except_blocks - generic_excepts;
        let raise_statements = code.matches("raise ").count();
        let finally_blocks = code.matches("finally:").count();

        let error_type_coverage = if except_blocks == 0 {
            0.0
        } else {
            specific_excepts as f64 / except_blocks as f64
        };

        let specific_ratio = if except_blocks == 0 {
            0.0
        } else {
            specific_excepts as f64 / except_blocks as f64
        };

        let unhandled_ratio = raise_statements as f64 / (try_blocks.max(1) as f64);

        let log_statements = code.matches("logging.error").count()
            + code.matches("logger.error").count()
            + code.matches("print(").count();

        let fallback_coverage = finally_blocks as f64 / (try_blocks.max(1) as f64);

        Self::calculate(ErrorHandlingInputs {
            error_type_coverage,
            unhandled_paths_ratio: unhandled_ratio.clamp(0.0, 1.0),
            specific_catches_ratio: specific_ratio.clamp(0.0, 1.0),
            logging_coverage: (log_statements as f64 / try_blocks.max(1) as f64).clamp(0.0, 1.0),
            fallback_coverage: fallback_coverage.clamp(0.0, 1.0),
            error_handlers: except_blocks,
            generic_catches: generic_excepts,
            log_statements,
        })
    }

    /// Analyze JavaScript/TypeScript error handling (try/catch, Promise, async/await)
    fn analyze_js_errors(code: &str) -> Self {
        let try_blocks = code.matches("try {").count();
        let catch_blocks = code.matches("catch (").count();
        let generic_catches =
            code.matches("catch (e)").count() + code.matches("catch (err)").count();
        let specific_catches = catch_blocks - generic_catches;
        let finally_blocks = code.matches("finally {").count();
        let throw_statements = code.matches("throw ").count();

        let error_type_coverage = if catch_blocks == 0 {
            0.0
        } else {
            specific_catches as f64 / catch_blocks as f64
        };

        let specific_ratio = error_type_coverage;
        let unhandled_ratio = throw_statements as f64 / (try_blocks.max(1) as f64);

        let log_statements =
            code.matches("console.error").count() + code.matches("logger.error").count();

        let fallback_coverage = finally_blocks as f64 / (try_blocks.max(1) as f64);

        Self::calculate(ErrorHandlingInputs {
            error_type_coverage,
            unhandled_paths_ratio: unhandled_ratio.clamp(0.0, 1.0),
            specific_catches_ratio: specific_ratio,
            logging_coverage: (log_statements as f64 / try_blocks.max(1) as f64).clamp(0.0, 1.0),
            fallback_coverage: fallback_coverage.clamp(0.0, 1.0),
            error_handlers: catch_blocks,
            generic_catches,
            log_statements,
        })
    }

    /// Analyze Java error handling (try/catch, throws, Optional)
    fn analyze_java_errors(code: &str) -> Self {
        let try_blocks = code.matches("try {").count();
        let catch_blocks = code.matches("catch (").count();
        let generic_catches = code.matches("catch (Exception").count();
        let specific_catches = catch_blocks - generic_catches;
        let finally_blocks = code.matches("finally {").count();
        let throws = code.matches("throws ").count();

        let error_type_coverage = if catch_blocks == 0 {
            0.0
        } else {
            specific_catches as f64 / catch_blocks as f64
        };

        let specific_ratio = error_type_coverage;
        let unhandled_ratio = throws as f64 / (try_blocks.max(1) as f64);

        let log_statements =
            code.matches("logger.error").count() + code.matches("e.printStackTrace").count();

        let fallback_coverage = finally_blocks as f64 / (try_blocks.max(1) as f64);

        Self::calculate(ErrorHandlingInputs {
            error_type_coverage,
            unhandled_paths_ratio: unhandled_ratio.clamp(0.0, 1.0),
            specific_catches_ratio: specific_ratio,
            logging_coverage: (log_statements as f64 / try_blocks.max(1) as f64).clamp(0.0, 1.0),
            fallback_coverage: fallback_coverage.clamp(0.0, 1.0),
            error_handlers: catch_blocks,
            generic_catches,
            log_statements,
        })
    }

    /// Fallback generic error analysis
    fn analyze_generic_errors(code: &str) -> Self {
        let error_keywords = code.matches("error").count() + code.matches("Error").count();
        let try_like = code.matches("try").count();
        let catch_like = code.matches("catch").count();

        let log_statements = code.matches("log").count() + code.matches("error").count();

        let error_type_coverage = if try_like == 0 {
            0.5
        } else {
            (catch_like as f64 / try_like as f64).clamp(0.0, 1.0)
        };

        Self::calculate(ErrorHandlingInputs {
            error_type_coverage,
            unhandled_paths_ratio: 0.3,
            specific_catches_ratio: 0.5,
            logging_coverage: (log_statements as f64 / 100.0).clamp(0.0, 1.0),
            fallback_coverage: 0.3,
            error_handlers: error_keywords,
            generic_catches: 0,
            log_statements,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_error_handling() {
        let code = r#"
            fn process(data: &[u8]) -> Result<String, Error> {
                let text = String::from_utf8(data.to_vec())?;
                match parse(&text) {
                    Ok(val) => Ok(val),
                    Err(e) => {
                        eprintln!("Parse error: {}", e);
                        Err(e)
                    }
                }
            }
        "#;

        let metrics = ErrorHandlingMetrics::analyze_rust_errors(code);
        assert!(metrics.error_handling_score > 0.0);
        assert!(metrics.log_statements > 0);
    }

    #[test]
    fn test_python_error_handling() {
        let code = r#"
            try:
                result = process_data(data)
            except ValueError as e:
                logging.error("Invalid value: %s", e)
                raise
            finally:
                cleanup()
        "#;

        let metrics = ErrorHandlingMetrics::analyze_python_errors(code);
        assert!(metrics.error_handling_score > 0.0);
        assert!(metrics.generic_catches == 0);
    }

    #[test]
    fn test_js_error_handling() {
        let code = r#"
            try {
                await processAsync(data);
            } catch (error) {
                console.error("Processing failed:", error);
                throw error;
            } finally {
                cleanup();
            }
        "#;

        let metrics = ErrorHandlingMetrics::analyze_js_errors(code);
        assert!(metrics.error_handling_score > 0.0);
    }

    #[test]
    fn test_generic_catches_detection() {
        let code = r#"
            try {
                something();
            } catch (e) {
                console.log(e);
            }
        "#;

        let metrics = ErrorHandlingMetrics::analyze_js_errors(code);
        assert!(metrics.generic_catches > 0);
    }

    #[test]
    fn test_calculate_formula() {
        let metrics = ErrorHandlingMetrics::calculate(ErrorHandlingInputs {
            error_type_coverage: 0.8,
            unhandled_paths_ratio: 0.1,
            specific_catches_ratio: 0.7,
            logging_coverage: 0.6,
            fallback_coverage: 0.5,
            error_handlers: 10,
            generic_catches: 1,
            log_statements: 5,
        });

        // Expected: 0.3*0.8 + 0.25*(1-0.1) + 0.2*0.7 + 0.15*0.6 + 0.1*0.5
        // = 0.24 + 0.225 + 0.14 + 0.09 + 0.05 = 0.745 * 100 = 74.5
        assert!((metrics.error_handling_score - 74.5).abs() < 1.0);
    }
}
