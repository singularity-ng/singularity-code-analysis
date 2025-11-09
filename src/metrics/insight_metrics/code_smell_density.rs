//! Code smell density metric for insight-driven analysis

use serde::{Deserialize, Serialize};

/// Code smell density statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSmellDensityStats {
    pub smell_density: f64,
    pub total_smells: usize,
    pub smell_types: Vec<SmellType>,
}

/// Code smell type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmellType {
    pub name: String,
    pub count: usize,
    pub severity: f64,
}

impl Default for CodeSmellDensityStats {
    fn default() -> Self {
        Self {
            smell_density: 0.0,
            total_smells: 0,
            smell_types: Vec::new(),
        }
    }
}

impl CodeSmellDensityStats {
    pub fn calculate_smell_density(&mut self, code: &str) -> f64 {
        let mut smells = Vec::new();

        // Detect various code smells
        if self.has_long_functions(code) {
            smells.push(SmellType {
                name: "Long Functions".to_string(),
                count: 1,
                severity: 0.8,
            });
        }

        if self.has_duplicate_code(code) {
            smells.push(SmellType {
                name: "Duplicate Code".to_string(),
                count: 1,
                severity: 0.7,
            });
        }

        if self.has_deep_nesting(code) {
            smells.push(SmellType {
                name: "Deep Nesting".to_string(),
                count: 1,
                severity: 0.9,
            });
        }

        if self.has_magic_numbers(code) {
            smells.push(SmellType {
                name: "Magic Numbers".to_string(),
                count: 1,
                severity: 0.5,
            });
        }

        if self.has_dead_code(code) {
            smells.push(SmellType {
                name: "Dead Code".to_string(),
                count: 1,
                severity: 0.6,
            });
        }

        self.smell_types = smells;
        self.total_smells = self.smell_types.len();

        let total_lines = code.lines().count();
        self.smell_density = if total_lines > 0 {
            self.total_smells as f64 / total_lines as f64 * 100.0
        } else {
            0.0
        };

        self.smell_density
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

    fn has_magic_numbers(&self, code: &str) -> bool {
        // Simple heuristic: check for hardcoded numbers
        code.matches(" 0 ").count() > 3 || code.matches(" 1 ").count() > 3
    }

    fn has_dead_code(&self, code: &str) -> bool {
        // Simple heuristic: check for unreachable code patterns
        code.contains("return") && code.contains("unreachable")
    }
}
