//! Refactoring readiness score for insight-driven analysis

use serde::{Deserialize, Serialize};

/// Refactoring readiness score statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringReadinessStats {
    pub readiness_score: f64,
    pub refactoring_opportunities: Vec<RefactoringOpportunity>,
}

/// Refactoring opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringOpportunity {
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub effort: f64,
}

impl Default for RefactoringReadinessStats {
    fn default() -> Self {
        Self {
            readiness_score: 0.0,
            refactoring_opportunities: Vec::new(),
        }
    }
}

impl RefactoringReadinessStats {
    pub fn calculate_readiness_score(&mut self, code: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Analyze refactoring factors
        if self.has_long_functions(code) {
            score -= 20.0;
        }

        if self.has_duplicate_code(code) {
            score -= 15.0;
        }

        if self.has_complex_conditionals(code) {
            score -= 10.0;
        }

        if self.has_deep_nesting(code) {
            score -= 15.0;
        }

        self.readiness_score = score.max(0.0);
        self.readiness_score
    }

    fn has_long_functions(&self, code: &str) -> bool {
        code.lines().count() > 50
    }

    fn has_duplicate_code(&self, code: &str) -> bool {
        let lines: Vec<&str> = code.lines().collect();
        for i in 0..lines.len() {
            for j in (i + 1)..lines.len() {
                if lines[i] == lines[j] && !lines[i].trim().is_empty() {
                    return true;
                }
            }
        }
        false
    }

    fn has_complex_conditionals(&self, code: &str) -> bool {
        code.matches("if").count() > 5
    }

    fn has_deep_nesting(&self, code: &str) -> bool {
        let mut max_nesting = 0;
        let mut current_nesting = 0;

        for line in code.lines() {
            for ch in line.chars() {
                match ch {
                    '{' | '[' | '(' => current_nesting += 1,
                    '}' | ']' | ')' => {
                        if current_nesting > 0 {
                            current_nesting -= 1;
                        }
                    }
                    _ => {}
                }
            }
            max_nesting = max_nesting.max(current_nesting);
        }

        max_nesting > 4
    }
}
