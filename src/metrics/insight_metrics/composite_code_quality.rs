//! Composite code quality metric with weighted factors

use serde::{Deserialize, Serialize};

/// Composite code quality statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeCodeQualityStats {
    pub quality_score: f64,
    pub quality_factors: Vec<QualityFactor>,
}

/// Quality factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityFactor {
    pub name: String,
    pub score: f64,
    pub weight: f64,
}

impl Default for CompositeCodeQualityStats {
    fn default() -> Self {
        Self {
            quality_score: 0.0,
            quality_factors: Vec::new(),
        }
    }
}

impl CompositeCodeQualityStats {
    pub fn calculate_quality_score(&mut self, code: &str) -> f64 {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        // Analyze various quality factors
        let readability = self.analyze_readability(code);
        let maintainability = self.analyze_maintainability(code);
        let performance = self.analyze_performance(code);
        let security = self.analyze_security(code);

        self.quality_factors = vec![
            QualityFactor {
                name: "Readability".to_string(),
                score: readability,
                weight: 0.3,
            },
            QualityFactor {
                name: "Maintainability".to_string(),
                score: maintainability,
                weight: 0.3,
            },
            QualityFactor {
                name: "Performance".to_string(),
                score: performance,
                weight: 0.2,
            },
            QualityFactor {
                name: "Security".to_string(),
                score: security,
                weight: 0.2,
            },
        ];

        for factor in &self.quality_factors {
            total_score += factor.score * factor.weight;
            total_weight += factor.weight;
        }

        self.quality_score = if total_weight > 0.0 {
            total_score / total_weight
        } else {
            0.0
        };

        self.quality_score
    }

    fn analyze_readability(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Check for clear naming
        if self.has_clear_naming(code) {
            score += 10.0;
        }

        // Check for appropriate comments
        if self.has_good_comments(code) {
            score += 15.0;
        }

        // Check for consistent formatting
        if self.has_consistent_formatting(code) {
            score += 10.0;
        }

        score.min(100.0_f64)
    }

    fn analyze_maintainability(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Check for modular structure
        if self.has_modular_structure(code) {
            score += 20.0;
        }

        // Check for low coupling
        if self.has_low_coupling(code) {
            score += 15.0;
        }

        // Check for high cohesion
        if self.has_high_cohesion(code) {
            score += 15.0;
        }

        score.min(100.0_f64)
    }

    fn analyze_performance(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Check for efficient algorithms
        if self.has_efficient_algorithms(code) {
            score += 20.0;
        }

        // Check for proper resource management
        if self.has_proper_resource_management(code) {
            score += 15.0;
        }

        score.min(100.0_f64)
    }

    fn analyze_security(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Check for input validation
        if self.has_input_validation(code) {
            score += 25.0;
        }

        // Check for secure coding practices
        if self.has_secure_practices(code) {
            score += 20.0;
        }

        score.min(100.0_f64)
    }

    fn has_clear_naming(&self, code: &str) -> bool {
        // Simple heuristic: check for descriptive variable names
        code.contains("user") || code.contains("order") || code.contains("result")
    }

    fn has_good_comments(&self, code: &str) -> bool {
        let comment_lines = code
            .lines()
            .filter(|line| line.trim().starts_with("//") || line.trim().starts_with("#"))
            .count();
        let total_lines = code.lines().count();
        comment_lines as f64 / total_lines as f64 > 0.1
    }

    fn has_consistent_formatting(&self, code: &str) -> bool {
        // Simple heuristic: check for consistent indentation
        let lines: Vec<&str> = code.lines().collect();
        if lines.is_empty() {
            return true;
        }

        let first_indent = lines[0].len() - lines[0].trim_start().len();
        lines.iter().all(|line| {
            let indent = line.len() - line.trim_start().len();
            indent == first_indent || indent == first_indent + 4 || indent == first_indent - 4
        })
    }

    fn has_modular_structure(&self, code: &str) -> bool {
        code.contains("fn ") || code.contains("def ") || code.contains("function ")
    }

    fn has_low_coupling(&self, code: &str) -> bool {
        // Simple heuristic: fewer external dependencies
        code.matches("import").count() < 10
    }

    fn has_high_cohesion(&self, code: &str) -> bool {
        // Simple heuristic: related functionality grouped together
        code.lines().count() < 100
    }

    fn has_efficient_algorithms(&self, code: &str) -> bool {
        // Simple heuristic: check for efficient patterns
        !code.contains("O(n^2)") && !code.contains("nested loop")
    }

    fn has_proper_resource_management(&self, code: &str) -> bool {
        // Simple heuristic: check for proper cleanup
        code.contains("close") || code.contains("dispose") || code.contains("free")
    }

    fn has_input_validation(&self, code: &str) -> bool {
        code.contains("validate") || code.contains("check") || code.contains("assert")
    }

    fn has_secure_practices(&self, code: &str) -> bool {
        !code.contains("password") || code.contains("hash") || code.contains("encrypt")
    }
}
