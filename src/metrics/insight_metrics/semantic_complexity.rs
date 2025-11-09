//! Semantic complexity metric for insight-driven analysis

use crate::langs::LANG;
use serde::{Deserialize, Serialize};

/// Semantic complexity metric statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticComplexityStats {
    pub semantic_score: f64,
    pub average_complexity: f64,
    pub max_complexity: f64,
    pub min_complexity: f64,
}

impl Default for SemanticComplexityStats {
    fn default() -> Self {
        Self {
            semantic_score: 0.0,
            average_complexity: 0.0,
            max_complexity: 0.0,
            min_complexity: f64::MAX,
        }
    }
}

impl SemanticComplexityStats {
    pub fn calculate_semantic_complexity(&mut self, code: &str, language: LANG) -> f64 {
        let patterns = self.analyze_semantic_patterns(code, language);
        let total_complexity: f64 = patterns.iter().sum();
        let function_count = patterns.len();

        self.average_complexity = if function_count > 0 {
            total_complexity / function_count as f64
        } else {
            0.0
        };

        self.max_complexity = patterns.iter().fold(0.0, |acc, value| acc.max(*value));

        self.min_complexity = patterns.iter().fold(f64::MAX, |acc, value| acc.min(*value));

        self.semantic_score = (self.average_complexity / 100.0 * 100.0).min(100.0);
        self.semantic_score
    }

    fn analyze_semantic_patterns(&self, code: &str, language: LANG) -> Vec<f64> {
        let mut patterns = Vec::new();
        let functions = self.extract_functions(code, language);

        for func in functions {
            let complexity = self.analyze_function_complexity(&func, language);
            patterns.push(complexity);
        }

        patterns
    }

    fn extract_functions(&self, code: &str, language: LANG) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for line in lines.iter() {
            if self.is_function_line(line, language) {
                functions.push(FunctionInfo {
                    name: self.extract_function_name(line),
                    content: line.to_string(),
                });
            }
        }

        functions
    }

    fn is_function_line(&self, line: &str, language: LANG) -> bool {
        match language {
            LANG::Rust => line.trim().starts_with("fn "),
            LANG::Python => line.trim().starts_with("def "),
            LANG::Javascript | LANG::Typescript => {
                line.trim().starts_with("function ") || line.contains("=>")
            }
            LANG::Java => line.trim().contains("(") && line.trim().contains(")"),
            LANG::Cpp => line.trim().contains("(") && line.trim().contains(")"),
            _ => false,
        }
    }

    /// Check if a line is a function definition using custom patterns
    #[allow(dead_code)]
    fn is_function_line_with_patterns(&self, line: &str, function_patterns: &[String]) -> bool {
        let trimmed = line.trim();
        function_patterns
            .iter()
            .any(|pattern| trimmed.starts_with(pattern))
    }

    /// Extract functions using custom patterns
    #[allow(dead_code)]
    fn extract_functions_with_patterns(
        &self,
        code: &str,
        function_patterns: &[String],
    ) -> Vec<FunctionInfo> {
        let mut functions = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for line in lines.iter() {
            if self.is_function_line_with_patterns(line, function_patterns) {
                functions.push(FunctionInfo {
                    name: self.extract_function_name(line),
                    content: line.to_string(),
                });
            }
        }

        functions
    }

    fn extract_function_name(&self, line: &str) -> String {
        let trimmed = line.trim();
        if let Some(start) = trimmed.find("fn ") {
            let after_fn = &trimmed[start + 3..];
            if let Some(end) = after_fn.find('(') {
                return after_fn[..end].trim().to_string();
            }
        }
        "unknown".to_string()
    }

    fn analyze_function_complexity(&self, func: &FunctionInfo, language: LANG) -> f64 {
        let mut complexity_weight = 0.0;

        complexity_weight += self.analyze_name_complexity(&func.name);
        complexity_weight += self.analyze_content_complexity(&func.content, language);

        if self.has_business_logic(&func.content) {
            complexity_weight += 10.0;
        }

        if self.has_multiple_responsibilities(&func.content) {
            complexity_weight += 15.0;
        }

        if self.has_data_transformation(&func.content) {
            complexity_weight += 8.0;
        }

        if self.has_complex_control_flow(&func.content) {
            complexity_weight += 12.0;
        }

        complexity_weight.clamp(0.0, 100.0)
    }

    fn analyze_name_complexity(&self, name: &str) -> f64 {
        let mut complexity = 0.0;

        if name.len() > 20 {
            complexity += 5.0;
        }

        let camel_case_count = name.chars().filter(|c| c.is_uppercase()).count();
        complexity += camel_case_count as f64 * 0.5;

        let underscore_count = name.matches('_').count();
        complexity += underscore_count as f64 * 0.3;

        complexity
    }

    fn analyze_content_complexity(&self, content: &str, language: LANG) -> f64 {
        let mut complexity = 0.0;

        let keywords = self.get_language_keywords(language);
        for keyword in keywords {
            let count = content.matches(&keyword).count();
            complexity += count as f64 * 0.5;
        }

        let operators = ["+", "-", "*", "/", "%", "==", "!=", "<", ">", "&&", "||"];
        for op in &operators {
            let count = content.matches(op).count();
            complexity += count as f64 * 0.3;
        }

        complexity
    }

    fn has_business_logic(&self, content: &str) -> bool {
        let business_keywords = [
            "calculate",
            "process",
            "validate",
            "transform",
            "business",
            "rule",
        ];
        business_keywords
            .iter()
            .any(|keyword| content.to_lowercase().contains(keyword))
    }

    fn has_multiple_responsibilities(&self, content: &str) -> bool {
        let responsibility_keywords = ["and", "also", "then", "additionally", "furthermore"];
        responsibility_keywords
            .iter()
            .any(|keyword| content.to_lowercase().contains(keyword))
    }

    fn has_data_transformation(&self, content: &str) -> bool {
        let transform_keywords = ["map", "filter", "reduce", "transform", "convert", "parse"];
        transform_keywords
            .iter()
            .any(|keyword| content.to_lowercase().contains(keyword))
    }

    fn has_complex_control_flow(&self, content: &str) -> bool {
        let control_keywords = ["if", "else", "switch", "case", "for", "while", "do"];
        control_keywords
            .iter()
            .any(|keyword| content.to_lowercase().contains(keyword))
    }

    fn get_language_keywords(&self, language: LANG) -> Vec<&'static str> {
        match language {
            LANG::Rust => vec![
                "fn", "struct", "impl", "trait", "enum", "match", "if", "let", "mut",
            ],
            LANG::Python => vec![
                "def", "class", "if", "for", "while", "try", "except", "with",
            ],
            LANG::Javascript | LANG::Typescript => {
                vec!["function", "class", "if", "for", "while", "try", "catch"]
            }
            LANG::Java => vec![
                "public",
                "private",
                "class",
                "interface",
                "if",
                "for",
                "while",
                "try",
                "catch",
            ],
            LANG::Cpp => vec![
                "class", "struct", "if", "for", "while", "try", "catch", "public", "private",
            ],
            _ => vec!["if", "for", "while", "function", "class"],
        }
    }
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    name: String,
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_complexity_calculation() {
        let mut stats = SemanticComplexityStats::default();
        let code = r#"
        fn calculate_user_score(user: User, orders: Vec<Order>) -> f64 {
            let mut total_score = 0.0;
            for order in orders {
                if order.status == OrderStatus::Completed {
                    total_score += order.amount * 0.1;
                }
            }
            total_score
        }
        "#;

        let complexity = stats.calculate_semantic_complexity(code, LANG::Rust);
        assert!(complexity > 0.0);
        assert!(complexity <= 100.0);
    }
}
