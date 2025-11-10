//! Language-aware code quality prediction.
//!
//! These helpers rely on static analysis heuristics only; the Elixir layer or
//! other orchestrators can call into them to build higher level services.

use crate::langs::LANG;

/// Predict quality of AI-generated code before generation
///
/// # Arguments
/// * `code_features` - Features extracted from code specification
/// * `language` - Target programming language
///
/// # Returns
/// * Quality prediction with confidence score
#[must_use]
#[inline]
pub fn predict_language_quality(code_features: &CodeFeatures, language: LANG) -> QualityPrediction {
    let baseline = get_language_baseline(language);
    let predicted_quality = calculate_predicted_quality(code_features, &baseline);
    let confidence_score = calculate_confidence(code_features);
    let risk_factors = identify_risk_factors(code_features, &baseline);
    let improvement_suggestions = generate_improvement_suggestions(code_features, &baseline);

    QualityPrediction {
        predicted_quality,
        confidence_score,
        risk_factors,
        improvement_suggestions,
    }
}

/// Calculate predicted quality score based on code features
#[must_use]
#[inline]
pub fn calculate_predicted_quality(
    features: &CodeFeatures,
    baseline: &QualityBaseline,
) -> QualityScore {
    let mut quality = QualityScore {
        overall_score: baseline.average_maintainability,
        maintainability: baseline.average_maintainability,
        readability: baseline.average_readability,
        testability: 70.0,
        performance: 75.0,
        security: 80.0,
        reliability: 75.0,
    };

    // Adjust based on complexity level
    match features.complexity_level {
        ComplexityLevel::Simple => {
            quality.maintainability += 10.0;
            quality.readability += 15.0;
        }
        ComplexityLevel::Medium => {
            quality.maintainability += 5.0;
            quality.readability += 5.0;
        }
        ComplexityLevel::Complex => {
            quality.maintainability -= 10.0;
            quality.readability -= 5.0;
        }
        ComplexityLevel::VeryComplex => {
            quality.maintainability -= 20.0;
            quality.readability -= 15.0;
        }
    }

    // Adjust based on error handling
    if features.error_handling_present {
        quality.reliability += 10.0;
        quality.security += 5.0;
    }

    // Adjust based on documentation
    if features.documentation_present {
        quality.readability += 10.0;
        quality.maintainability += 5.0;
    }

    // Adjust based on test coverage
    quality.testability = features.test_coverage;

    // Calculate overall score
    quality.overall_score = (quality.maintainability
        + quality.readability
        + quality.testability
        + quality.performance
        + quality.security
        + quality.reliability)
        / 6.0;

    quality
}

/// Calculate confidence score for quality prediction
#[must_use]
#[inline]
pub fn calculate_confidence(features: &CodeFeatures) -> f64 {
    let mut confidence = 0.7_f64; // Base confidence

    // Increase confidence for simpler code
    match features.complexity_level {
        ComplexityLevel::Simple => confidence += 0.2,
        ComplexityLevel::Medium => confidence += 0.1,
        ComplexityLevel::Complex => confidence -= 0.1,
        ComplexityLevel::VeryComplex => confidence -= 0.2,
    }

    // Increase confidence for well-documented specifications
    if features.documentation_present {
        confidence += 0.05;
    }

    confidence.clamp(0.0_f64, 1.0_f64)
}

/// Identify risk factors that could affect quality
#[must_use]
#[inline]
pub fn identify_risk_factors(
    features: &CodeFeatures,
    baseline: &QualityBaseline,
) -> Vec<RiskFactor> {
    let mut risks = Vec::new();

    if features.complexity_level == ComplexityLevel::VeryComplex {
        risks.push(RiskFactor {
            factor_type: RiskFactorType::HighComplexity,
            description: "Very complex code may be difficult to maintain".to_string(),
            severity: RiskSeverity::High,
        });
    }

    if features.naming_convention_score < 0.7 {
        risks.push(RiskFactor {
            factor_type: RiskFactorType::PoorNaming,
            description: "Poor naming conventions may reduce readability".to_string(),
            severity: RiskSeverity::Medium,
        });
    }

    if !features.error_handling_present {
        risks.push(RiskFactor {
            factor_type: RiskFactorType::MissingErrorHandling,
            description: "Missing error handling may cause runtime failures".to_string(),
            severity: RiskSeverity::High,
        });
    }

    if !features.documentation_present {
        risks.push(RiskFactor {
            factor_type: RiskFactorType::InsufficientDocumentation,
            description: "Lack of documentation may reduce maintainability".to_string(),
            severity: RiskSeverity::Medium,
        });
    }

    if features.test_coverage < baseline.quality_thresholds.min_test_coverage {
        risks.push(RiskFactor {
            factor_type: RiskFactorType::LowTestability,
            description: "Low test coverage may indicate poor testability".to_string(),
            severity: RiskSeverity::Medium,
        });
    }

    risks
}

/// Generate improvement suggestions based on code features
#[must_use]
#[inline]
pub fn generate_improvement_suggestions(
    features: &CodeFeatures,
    _baseline: &QualityBaseline,
) -> Vec<String> {
    let mut suggestions = Vec::new();

    if features.complexity_level == ComplexityLevel::VeryComplex {
        suggestions.push("Break down complex logic into smaller, focused functions".to_string());
    }

    if features.nesting_depth > 3 {
        suggestions.push("Reduce nesting depth using early returns or guard clauses".to_string());
    }

    if !features.error_handling_present {
        suggestions.push("Add comprehensive error handling and validation".to_string());
    }

    if !features.documentation_present {
        suggestions.push("Include detailed documentation and code comments".to_string());
    }

    if features.test_coverage < 80.0 {
        suggestions.push("Ensure comprehensive test coverage for all code paths".to_string());
    }

    suggestions
}

/// Extract code features from specification
#[must_use]
#[inline]
pub fn extract_features_from_spec(spec: &CodeSpecification, language: LANG) -> CodeFeatures {
    let mut features = CodeFeatures {
        complexity_level: estimate_complexity_level(spec),
        function_count: spec.expected_function_count,
        class_count: spec.expected_class_count,
        nesting_depth: spec.expected_nesting_depth,
        parameter_count: spec.expected_parameter_count,
        return_type_complexity: estimate_return_type_complexity(spec),
        error_handling_present: spec.requires_error_handling,
        documentation_present: spec.requires_documentation,
        test_coverage: spec.expected_test_coverage,
        naming_convention_score: assess_naming_convention(spec),
        design_pattern_usage: identify_design_patterns(spec),
    };

    match language {
        LANG::Rust | LANG::Go | LANG::Cpp => {
            features.naming_convention_score = (features.naming_convention_score + 0.05).min(1.0);
        }
        LANG::Elixir | LANG::Erlang | LANG::Gleam => {
            if spec.requires_error_handling && !features.error_handling_present {
                features.error_handling_present = true;
            }
        }
        LANG::Javascript | LANG::Typescript | LANG::Csharp => {
            if !features
                .design_pattern_usage
                .contains(&"Module".to_string())
            {
                features.design_pattern_usage.push("Module".to_string());
            }
        }
        _ => {}
    }

    features
}

/// Calculate improvement score between two quality scores
#[must_use]
#[inline]
pub fn calculate_quality_improvement_score(before: &QualityScore, after: &QualityScore) -> f64 {
    let maintainability_improvement = (after.maintainability - before.maintainability) / 100.0;
    let readability_improvement = (after.readability - before.readability) / 100.0;
    let testability_improvement = (after.testability - before.testability) / 100.0;

    (maintainability_improvement + readability_improvement + testability_improvement) / 3.0
}

// Private helper functions

fn get_language_baseline(language: LANG) -> QualityBaseline {
    match language {
        LANG::Rust => QualityBaseline {
            language: LANG::Rust,
            average_complexity: 5.0,
            average_maintainability: 80.0,
            average_readability: 85.0,
            quality_thresholds: QualityThresholds {
                min_maintainability: 70.0,
                min_readability: 75.0,
                max_complexity: 10.0,
                min_test_coverage: 80.0,
            },
        },
        LANG::Javascript => QualityBaseline {
            language: LANG::Javascript,
            average_complexity: 6.0,
            average_maintainability: 75.0,
            average_readability: 80.0,
            quality_thresholds: QualityThresholds {
                min_maintainability: 65.0,
                min_readability: 70.0,
                max_complexity: 12.0,
                min_test_coverage: 70.0,
            },
        },
        LANG::Python => QualityBaseline {
            language: LANG::Python,
            average_complexity: 4.0,
            average_maintainability: 85.0,
            average_readability: 90.0,
            quality_thresholds: QualityThresholds {
                min_maintainability: 75.0,
                min_readability: 80.0,
                max_complexity: 8.0,
                min_test_coverage: 85.0,
            },
        },
        _ => QualityBaseline {
            language: LANG::Rust, // Default fallback
            average_complexity: 5.0,
            average_maintainability: 75.0,
            average_readability: 80.0,
            quality_thresholds: QualityThresholds {
                min_maintainability: 70.0,
                min_readability: 75.0,
                max_complexity: 10.0,
                min_test_coverage: 75.0,
            },
        },
    }
}

fn estimate_complexity_level(spec: &CodeSpecification) -> ComplexityLevel {
    match spec.complexity_hint.as_str() {
        "simple" => ComplexityLevel::Simple,
        "medium" => ComplexityLevel::Medium,
        "complex" => ComplexityLevel::Complex,
        "very_complex" => ComplexityLevel::VeryComplex,
        _ => {
            if spec.expected_function_count > 10 || spec.expected_nesting_depth > 3 {
                ComplexityLevel::Complex
            } else if spec.expected_function_count > 5 || spec.expected_nesting_depth > 2 {
                ComplexityLevel::Medium
            } else {
                ComplexityLevel::Simple
            }
        }
    }
}

fn estimate_return_type_complexity(spec: &CodeSpecification) -> f64 {
    match spec.return_type_complexity.as_str() {
        "simple" => 1.0,
        "medium" => 2.0,
        "complex" => 3.0,
        _ => 1.5,
    }
}

fn assess_naming_convention(spec: &CodeSpecification) -> f64 {
    if spec.description.len() > 50 && spec.description.contains("function") {
        0.8
    } else {
        0.6
    }
}

fn identify_design_patterns(spec: &CodeSpecification) -> Vec<String> {
    let mut patterns = Vec::new();

    if spec.description.contains("singleton") {
        patterns.push("Singleton".to_string());
    }
    if spec.description.contains("factory") {
        patterns.push("Factory".to_string());
    }
    if spec.description.contains("observer") {
        patterns.push("Observer".to_string());
    }

    patterns
}

/// Code features that influence quality
#[derive(Debug, Clone)]
pub struct CodeFeatures {
    pub complexity_level: ComplexityLevel,
    pub function_count: u32,
    pub class_count: u32,
    pub nesting_depth: u32,
    pub parameter_count: u32,
    pub return_type_complexity: f64,
    pub error_handling_present: bool,
    pub documentation_present: bool,
    pub test_coverage: f64,
    pub naming_convention_score: f64,
    pub design_pattern_usage: Vec<String>,
}

/// Complexity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ComplexityLevel {
    Simple,
    Medium,
    Complex,
    VeryComplex,
}

/// Quality score prediction
#[derive(Debug, Clone)]
pub struct QualityScore {
    pub overall_score: f64,
    pub maintainability: f64,
    pub readability: f64,
    pub testability: f64,
    pub performance: f64,
    pub security: f64,
    pub reliability: f64,
}

/// Language-specific quality baseline
#[derive(Debug, Clone)]
pub struct QualityBaseline {
    pub language: LANG,
    pub average_complexity: f64,
    pub average_maintainability: f64,
    pub average_readability: f64,
    pub quality_thresholds: QualityThresholds,
}

/// Quality thresholds for different languages
#[derive(Debug, Clone)]
pub struct QualityThresholds {
    pub min_maintainability: f64,
    pub min_readability: f64,
    pub max_complexity: f64,
    pub min_test_coverage: f64,
}

/// Quality prediction result
#[derive(Debug, Clone)]
pub struct QualityPrediction {
    pub predicted_quality: QualityScore,
    pub confidence_score: f64,
    pub risk_factors: Vec<RiskFactor>,
    pub improvement_suggestions: Vec<String>,
}

/// Risk factors that could affect quality
#[derive(Debug, Clone)]
pub struct RiskFactor {
    pub factor_type: RiskFactorType,
    pub description: String,
    pub severity: RiskSeverity,
}

/// Types of risk factors
#[derive(Debug, Clone)]
pub enum RiskFactorType {
    HighComplexity,
    PoorNaming,
    MissingErrorHandling,
    InsufficientDocumentation,
    LowTestability,
}

/// Risk severity levels
#[derive(Debug, Clone)]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Code specification for quality prediction
#[derive(Debug, Clone)]
pub struct CodeSpecification {
    pub description: String,
    pub complexity_hint: String,
    pub expected_function_count: u32,
    pub expected_class_count: u32,
    pub expected_nesting_depth: u32,
    pub expected_parameter_count: u32,
    pub return_type_complexity: String,
    pub requires_error_handling: bool,
    pub requires_documentation: bool,
    pub expected_test_coverage: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_language_quality() {
        let features = CodeFeatures {
            complexity_level: ComplexityLevel::Simple,
            function_count: 1,
            class_count: 0,
            nesting_depth: 1,
            parameter_count: 2,
            return_type_complexity: 1.0,
            error_handling_present: true,
            documentation_present: true,
            test_coverage: 90.0,
            naming_convention_score: 0.9,
            design_pattern_usage: vec![],
        };

        let prediction = predict_language_quality(&features, LANG::Rust);
        assert!(prediction.predicted_quality.overall_score > 0.0);
        assert!(prediction.confidence_score > 0.0);
    }

    #[test]
    fn test_calculate_predicted_quality() {
        let features = CodeFeatures {
            complexity_level: ComplexityLevel::Simple,
            function_count: 1,
            class_count: 0,
            nesting_depth: 1,
            parameter_count: 2,
            return_type_complexity: 1.0,
            error_handling_present: true,
            documentation_present: true,
            test_coverage: 90.0,
            naming_convention_score: 0.9,
            design_pattern_usage: vec![],
        };

        let baseline = get_language_baseline(LANG::Rust);
        let quality = calculate_predicted_quality(&features, &baseline);
        assert!(quality.overall_score > 0.0);
    }

    #[test]
    fn test_extract_features_from_spec() {
        let spec = CodeSpecification {
            description: "A simple function to add two numbers".to_string(),
            complexity_hint: "simple".to_string(),
            expected_function_count: 1,
            expected_class_count: 0,
            expected_nesting_depth: 1,
            expected_parameter_count: 2,
            return_type_complexity: "simple".to_string(),
            requires_error_handling: true,
            requires_documentation: true,
            expected_test_coverage: 90.0,
        };

        let features = extract_features_from_spec(&spec, LANG::Rust);
        assert_eq!(features.function_count, 1);
        assert_eq!(features.complexity_level, ComplexityLevel::Simple);
    }
}
