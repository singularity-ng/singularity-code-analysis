//! Type Safety Score - Measures code's type safety and type coverage
//!
//! Evaluates type annotations, generics, unsafe code, and pattern matching coverage
//! to predict runtime errors and code correctness.

use std::collections::HashMap;

/// Type Safety Metrics
#[derive(Debug, Clone, PartialEq)]
pub struct TypeSafetyMetrics {
    /// Overall type safety score (0-100)
    pub type_safety_score: f64,
    /// Type annotation coverage percentage
    pub annotation_coverage: f64,
    /// Generic type usage score
    pub generic_usage_score: f64,
    /// Unsafe code ratio (0-1, lower is better)
    pub unsafe_ratio: f64,
    /// Explicit type ratio vs inferred
    pub explicit_type_ratio: f64,
    /// Pattern matching coverage (Rust)
    pub pattern_matching_score: f64,
    /// Per-language breakdown
    pub language_scores: HashMap<String, LanguageTypeSafety>,
}

/// Language-specific type safety metrics
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageTypeSafety {
    pub score: f64,
    pub annotation_count: usize,
    pub total_declarations: usize,
    pub unsafe_blocks: usize,
    pub generic_types: usize,
    pub pattern_matches: usize,
}

impl TypeSafetyMetrics {
    /// Calculate type safety metrics from code analysis
    pub fn calculate(
        language: &str,
        annotation_coverage: f64,
        generic_usage: f64,
        unsafe_ratio: f64,
        explicit_type_ratio: f64,
        pattern_matching: f64,
    ) -> Self {
        // Weighted formula:
        // 0.3 * annotation_coverage +
        // 0.2 * generic_usage_score +
        // 0.25 * (1 - unsafe_ratio) +
        // 0.15 * explicit_type_ratio +
        // 0.1 * pattern_matching_score

        let type_safety_score = (0.3 * annotation_coverage
            + 0.2 * generic_usage
            + 0.25 * (1.0 - unsafe_ratio)
            + 0.15 * explicit_type_ratio
            + 0.1 * pattern_matching)
            * 100.0;

        let mut language_scores = HashMap::new();
        language_scores.insert(
            language.to_string(),
            LanguageTypeSafety {
                score: type_safety_score,
                annotation_count: (annotation_coverage * 100.0) as usize,
                total_declarations: 100,
                unsafe_blocks: (unsafe_ratio * 100.0) as usize,
                generic_types: (generic_usage * 100.0) as usize,
                pattern_matches: (pattern_matching * 100.0) as usize,
            },
        );

        Self {
            type_safety_score: type_safety_score.clamp(0.0, 100.0),
            annotation_coverage: (annotation_coverage * 100.0).clamp(0.0, 100.0),
            generic_usage_score: (generic_usage * 100.0).clamp(0.0, 100.0),
            unsafe_ratio: unsafe_ratio.clamp(0.0, 1.0),
            explicit_type_ratio: (explicit_type_ratio * 100.0).clamp(0.0, 100.0),
            pattern_matching_score: (pattern_matching * 100.0).clamp(0.0, 100.0),
            language_scores,
        }
    }
}

/// Analyze type safety for Rust code
pub fn analyze_rust_type_safety(code: &str) -> TypeSafetyMetrics {
    let annotation_count = code.matches(": ").count();
    let annotation_coverage = if annotation_count > 0 { 0.8 } else { 0.0 };
    let explicit_types = annotation_count;

    let generic_usage = code.matches('<').count().min(10) as f64 / 10.0;

    let line_count = code.lines().count().max(1) as f64;
    let unsafe_blocks = code.matches("unsafe {").count();
    let unsafe_ratio = (unsafe_blocks as f64 / line_count).clamp(0.0, 1.0);

    let total_declarations = code.matches("let ").count().max(1);
    let explicit_type_ratio = (explicit_types as f64 / total_declarations as f64).clamp(0.0, 1.0);

    let pattern_matches = code.matches("match ").count() + code.matches("if let ").count();
    let pattern_matching_score = (pattern_matches as f64 / line_count).clamp(0.0, 1.0);

    TypeSafetyMetrics::calculate(
        "rust",
        annotation_coverage,
        generic_usage,
        unsafe_ratio,
        explicit_type_ratio,
        pattern_matching_score,
    )
}

/// Analyze type safety for TypeScript code
pub fn analyze_typescript_type_safety(code: &str) -> TypeSafetyMetrics {
    let line_count = code.lines().count().max(1) as f64;
    let annotation_count = code.matches(": ").count();
    let annotation_coverage = (annotation_count as f64 / line_count).clamp(0.0, 1.0);

    let generic_usage = code.matches('<').count().min(10) as f64 / 10.0;

    let explicit_types = code.matches("as ").count() + annotation_count;
    let total_declarations = (code.matches("let ").count()
        + code.matches("const ").count()
        + code.matches("var ").count())
    .max(1);
    let explicit_type_ratio = (explicit_types as f64 / total_declarations as f64).clamp(0.0, 1.0);

    let pattern_matches = code.matches("switch ").count() + code.matches("as ").count();
    let pattern_matching_score = (pattern_matches as f64 / line_count).clamp(0.0, 1.0);

    // TypeScript has no unsafe, so that ratio is 0
    TypeSafetyMetrics::calculate(
        "typescript",
        annotation_coverage,
        generic_usage,
        0.0,
        explicit_type_ratio,
        pattern_matching_score,
    )
}

/// Analyze type safety for Python code
pub fn analyze_python_type_safety(code: &str) -> TypeSafetyMetrics {
    let line_count = code.lines().count().max(1) as f64;
    let annotation_count = code.matches(": ").count();
    let annotation_coverage = (annotation_count as f64 / line_count).clamp(0.0, 1.0);

    let generic_usage =
        (code.matches("Generic").count() + code.matches("TypeVar").count()) as f64 / 10.0;

    let total_declarations = (code.matches("def ").count() + code.matches("class ").count()).max(1);
    let explicit_type_ratio = (annotation_count as f64 / total_declarations as f64).clamp(0.0, 1.0);

    let pattern_matches = code.matches("match ").count();
    let pattern_matching_score = (pattern_matches as f64 / line_count).clamp(0.0, 1.0);

    // Python has no unsafe
    TypeSafetyMetrics::calculate(
        "python",
        annotation_coverage,
        generic_usage,
        0.0,
        explicit_type_ratio,
        pattern_matching_score,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_type_safety() {
        let code = r#"
            let x: i32 = 42;
            let y = vec![1, 2, 3];
            unsafe { println!("unsafe"); }
            match result {
                Ok(val) => println!("{}", val),
                Err(_) => eprintln!("error"),
            }
        "#;

        let metrics = analyze_rust_type_safety(code);
        assert!(metrics.type_safety_score > 0.0);
        assert!(metrics.explicit_type_ratio > 0.0);
        assert!(metrics.pattern_matching_score > 0.0);
    }

    #[test]
    fn test_typescript_type_safety() {
        let code = r#"
            const x: number = 42;
            interface Config { name: string; }
            const config: Config = { name: "test" };
            function process<T>(input: T): T { return input; }
        "#;

        let metrics = analyze_typescript_type_safety(code);
        assert!(metrics.type_safety_score > 0.0);
        assert_eq!(metrics.unsafe_ratio, 0.0);
    }

    #[test]
    fn test_python_type_safety() {
        let code = r#"
            def calculate(x: int, y: int) -> int:
                return x + y

            class Handler:
                def process(self, data: str) -> bool:
                    return len(data) > 0
        "#;

        let metrics = analyze_python_type_safety(code);
        assert!(metrics.type_safety_score > 0.0);
        assert!(metrics.annotation_coverage > 0.0);
    }

    #[test]
    fn test_calculate_formula() {
        let metrics = TypeSafetyMetrics::calculate(
            "test", 0.8, // annotation_coverage
            0.5, // generic_usage
            0.1, // unsafe_ratio
            0.7, // explicit_type_ratio
            0.6, // pattern_matching
        );

        // Expected: 0.3*0.8 + 0.2*0.5 + 0.25*(1-0.1) + 0.15*0.7 + 0.1*0.6
        // = 0.24 + 0.1 + 0.225 + 0.105 + 0.06 = 0.73 * 100 = 73.0
        assert!((metrics.type_safety_score - 73.0).abs() < 1.0);
    }
}
