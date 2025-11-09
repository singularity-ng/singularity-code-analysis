//! Testability score metric for insight-driven analysis

use serde::{Deserialize, Serialize};

/// Testability score statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestabilityScoreStats {
    pub testability_score: f64,
    pub testability_factors: Vec<TestabilityFactor>,
}

/// Testability factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestabilityFactor {
    pub name: String,
    pub score: f64,
    pub weight: f64,
}

impl Default for TestabilityScoreStats {
    fn default() -> Self {
        Self {
            testability_score: 0.0,
            testability_factors: Vec::new(),
        }
    }
}

impl TestabilityScoreStats {
    pub fn calculate_testability_score(&mut self, code: &str) -> f64 {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        // Analyze various testability factors
        let modularity = self.analyze_modularity(code);
        let dependency_injection = self.analyze_dependency_injection(code);
        let pure_functions = self.analyze_pure_functions(code);
        let error_handling = self.analyze_error_handling(code);

        self.testability_factors = vec![
            TestabilityFactor {
                name: "Modularity".to_string(),
                score: modularity,
                weight: 0.3,
            },
            TestabilityFactor {
                name: "Dependency Injection".to_string(),
                score: dependency_injection,
                weight: 0.25,
            },
            TestabilityFactor {
                name: "Pure Functions".to_string(),
                score: pure_functions,
                weight: 0.25,
            },
            TestabilityFactor {
                name: "Error Handling".to_string(),
                score: error_handling,
                weight: 0.2,
            },
        ];

        for factor in &self.testability_factors {
            total_score += factor.score * factor.weight;
            total_weight += factor.weight;
        }

        self.testability_score = if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0
        };

        self.testability_score
    }

    fn analyze_modularity(&self, code: &str) -> f64 {
        let mut score: f64 = 0.0;

        // Check for function/module structure
        if code.contains("fn ") || code.contains("def ") || code.contains("function ") {
            score += 30.0;
        }

        // Check for class/module organization
        if code.contains("struct ") || code.contains("class ") || code.contains("module ") {
            score += 20.0;
        }

        // Check for appropriate function size
        let lines = code.lines().count();
        if lines < 50 {
            score += 25.0;
        } else if lines < 100 {
            score += 15.0;
        }

        // Check for single responsibility
        if self.has_single_responsibility(code) {
            score += 25.0;
        }

        score.min(100.0_f64)
    }

    fn analyze_dependency_injection(&self, code: &str) -> f64 {
        let mut score: f64 = 0.0;

        // Check for constructor injection
        if code.contains("new ") || code.contains("::new") {
            score += 20.0;
        }

        // Check for interface usage
        if code.contains("trait ") || code.contains("interface ") || code.contains("protocol ") {
            score += 30.0;
        }

        // Check for dependency parameters
        if code.contains("deps") || code.contains("dependencies") {
            score += 25.0;
        }

        // Check for mockable patterns
        if code.contains("mock") || code.contains("stub") || code.contains("fake") {
            score += 25.0;
        }

        score.min(100.0_f64)
    }

    fn analyze_pure_functions(&self, code: &str) -> f64 {
        let mut score: f64 = 0.0;

        // Check for functions without side effects
        if self.has_pure_functions(code) {
            score += 40.0;
        }

        // Check for immutable data
        if code.contains("const ") || code.contains("final ") || code.contains("immutable") {
            score += 30.0;
        }

        // Check for functional programming patterns
        if code.contains("map") || code.contains("filter") || code.contains("reduce") {
            score += 30.0;
        }

        score.min(100.0_f64)
    }

    fn analyze_error_handling(&self, code: &str) -> f64 {
        let mut score: f64 = 0.0;

        // Check for proper error handling
        if code.contains("try")
            || code.contains("catch")
            || code.contains("Result")
            || code.contains("Option")
        {
            score += 40.0;
        }

        // Check for error propagation
        if code.contains("?") || code.contains("unwrap") || code.contains("expect") {
            score += 30.0;
        }

        // Check for logging
        if code.contains("log") || code.contains("debug") || code.contains("error") {
            score += 30.0;
        }

        score.min(100.0_f64)
    }

    fn has_single_responsibility(&self, code: &str) -> bool {
        // Simple heuristic: check for focused function names
        let function_keywords = ["calculate", "validate", "process", "format", "parse"];
        function_keywords
            .iter()
            .any(|keyword| code.to_lowercase().contains(keyword))
    }

    fn has_pure_functions(&self, code: &str) -> bool {
        // Simple heuristic: check for functions without side effects
        !code.contains("print") && !code.contains("log") && !code.contains("write")
    }
}
