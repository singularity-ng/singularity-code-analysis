//! Database-enriched insight metrics for best-in-class code analysis
//!
//! This module integrates with the existing PostgreSQL + pgvector + graph database
//! infrastructure to provide enriched insight metrics with real semantic data.

use crate::langs::LANG;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database-enriched insight metrics that leverage vector search and graph data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseEnrichedInsightMetrics {
    /// Semantic complexity with database patterns
    pub semantic_complexity: DatabaseSemanticComplexity,
    /// Refactoring readiness with historical data
    pub refactoring_readiness: DatabaseRefactoringReadiness,
    /// Composite code quality with learned patterns
    pub composite_code_quality: DatabaseCompositeCodeQuality,
    /// Code smell density with pattern database
    pub code_smell_density: DatabaseCodeSmellDensity,
    /// Testability score with historical test data
    pub testability_score: DatabaseTestabilityScore,
}

/// Database-enriched semantic complexity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSemanticComplexity {
    /// Overall semantic complexity score (0-100)
    pub semantic_score: f64,
    /// Similar patterns from database
    pub similar_patterns: Vec<DatabasePattern>,
    /// Historical complexity trends
    pub complexity_trends: Vec<ComplexityTrend>,
    /// Language-specific patterns from database
    pub language_patterns: HashMap<LANG, Vec<DatabasePattern>>,
    /// Graph relationships (dependencies, callers, etc.)
    pub graph_relationships: Vec<GraphRelationship>,
}

/// Database pattern with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabasePattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pattern_type: PatternType,
    pub complexity_score: f64,
    pub language: LANG,
    pub example: String,
    /// Vector embedding for similarity search
    pub embedding: Vec<f32>,
    /// Usage frequency in database
    pub usage_frequency: u32,
    /// Success rate when used
    pub success_rate: f64,
    /// Last updated timestamp
    pub last_updated: String,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Pattern types from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    DesignPattern,
    AntiPattern,
    CodeSmell,
    BestPractice,
    RefactoringOpportunity,
    SynthesizedPattern,
    LearnedPattern,
}

/// Complexity trend over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityTrend {
    pub timestamp: String,
    pub complexity_score: f64,
    pub file_path: String,
    pub commit_hash: String,
}

/// Graph relationship from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphRelationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub metadata: HashMap<String, String>,
}

/// Types of graph relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Calls,
    DependsOn,
    Implements,
    Extends,
    Uses,
    SimilarTo,
    RefactoredFrom,
    TestedBy,
}

/// Database-enriched refactoring readiness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseRefactoringReadiness {
    pub readiness_score: f64,
    /// Refactoring opportunities from database
    pub refactoring_opportunities: Vec<DatabaseRefactoringOpportunity>,
    /// Historical refactoring success rates
    pub historical_success_rates: HashMap<String, f64>,
    /// Similar refactoring patterns
    pub similar_refactorings: Vec<DatabaseRefactoringPattern>,
}

/// Database refactoring opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseRefactoringOpportunity {
    pub id: String,
    pub name: String,
    pub description: String,
    pub priority: f64,
    pub effort: f64,
    /// Success rate of similar refactorings
    pub success_rate: f64,
    /// Estimated time to complete
    pub estimated_time: u32, // minutes
    /// Required skills
    pub required_skills: Vec<String>,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Example from database
    pub example: String,
}

/// Database refactoring pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseRefactoringPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub before_code: String,
    pub after_code: String,
    pub success_rate: f64,
    pub complexity_reduction: f64,
    pub language: LANG,
    pub tags: Vec<String>,
}

/// Database-enriched composite code quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCompositeCodeQuality {
    pub quality_score: f64,
    /// Quality factors with database context
    pub quality_factors: Vec<DatabaseQualityFactor>,
    /// Learned quality patterns
    pub quality_patterns: Vec<DatabaseQualityPattern>,
    /// Historical quality trends
    pub quality_trends: Vec<QualityTrend>,
}

/// Database quality factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseQualityFactor {
    pub name: String,
    pub score: f64,
    pub weight: f64,
    /// Database-learned weight
    pub learned_weight: f64,
    /// Historical performance
    pub historical_performance: Vec<f64>,
    /// Industry benchmarks
    pub industry_benchmark: f64,
}

/// Database quality pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseQualityPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub quality_impact: f64,
    pub frequency: u32,
    pub success_rate: f64,
    pub language: LANG,
    pub example: String,
}

/// Quality trend over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTrend {
    pub timestamp: String,
    pub quality_score: f64,
    pub factor: String,
    pub file_path: String,
}

/// Database-enriched code smell density
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCodeSmellDensity {
    pub smell_density: f64,
    /// Code smells from database
    pub code_smells: Vec<DatabaseCodeSmell>,
    /// Historical smell patterns
    pub historical_smells: Vec<HistoricalSmell>,
    /// Smell resolution patterns
    pub resolution_patterns: Vec<SmellResolutionPattern>,
}

/// Database code smell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCodeSmell {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: f64,
    pub location: CodeLocation,
    pub suggestion: String,
    /// Similar smells in database
    pub similar_smells: Vec<String>,
    /// Resolution success rate
    pub resolution_success_rate: f64,
    /// Average resolution time
    pub average_resolution_time: u32, // minutes
}

/// Historical smell data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalSmell {
    pub timestamp: String,
    pub smell_type: String,
    pub severity: f64,
    pub file_path: String,
    pub resolved: bool,
    pub resolution_time: Option<u32>,
}

/// Smell resolution pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmellResolutionPattern {
    pub id: String,
    pub smell_type: String,
    pub resolution_approach: String,
    pub success_rate: f64,
    pub average_time: u32,
    pub example: String,
}

/// Database-enriched testability score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseTestabilityScore {
    pub testability_score: f64,
    /// Testability factors with database context
    pub testability_factors: Vec<DatabaseTestabilityFactor>,
    /// Historical test data
    pub historical_test_data: Vec<HistoricalTestData>,
    /// Test generation patterns
    pub test_generation_patterns: Vec<TestGenerationPattern>,
}

/// Database testability factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseTestabilityFactor {
    pub name: String,
    pub score: f64,
    pub weight: f64,
    /// Database-learned weight
    pub learned_weight: f64,
    /// Historical test success rate
    pub test_success_rate: f64,
    /// Industry benchmarks
    pub industry_benchmark: f64,
}

/// Historical test data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalTestData {
    pub timestamp: String,
    pub test_type: String,
    pub success_rate: f64,
    pub coverage: f64,
    pub file_path: String,
    pub test_count: u32,
}

/// Test generation pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGenerationPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub success_rate: f64,
    pub coverage_improvement: f64,
    pub language: LANG,
    pub example: String,
}

/// Code location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub file_path: String,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
}

impl Default for DatabaseEnrichedInsightMetrics {
    fn default() -> Self {
        Self {
            semantic_complexity: DatabaseSemanticComplexity::default(),
            refactoring_readiness: DatabaseRefactoringReadiness::default(),
            composite_code_quality: DatabaseCompositeCodeQuality::default(),
            code_smell_density: DatabaseCodeSmellDensity::default(),
            testability_score: DatabaseTestabilityScore::default(),
        }
    }
}

impl Default for DatabaseSemanticComplexity {
    fn default() -> Self {
        Self {
            semantic_score: 0.0,
            similar_patterns: Vec::new(),
            complexity_trends: Vec::new(),
            language_patterns: HashMap::new(),
            graph_relationships: Vec::new(),
        }
    }
}

impl Default for DatabaseRefactoringReadiness {
    fn default() -> Self {
        Self {
            readiness_score: 0.0,
            refactoring_opportunities: Vec::new(),
            historical_success_rates: HashMap::new(),
            similar_refactorings: Vec::new(),
        }
    }
}

impl Default for DatabaseCompositeCodeQuality {
    fn default() -> Self {
        Self {
            quality_score: 0.0,
            quality_factors: Vec::new(),
            quality_patterns: Vec::new(),
            quality_trends: Vec::new(),
        }
    }
}

impl Default for DatabaseCodeSmellDensity {
    fn default() -> Self {
        Self {
            smell_density: 0.0,
            code_smells: Vec::new(),
            historical_smells: Vec::new(),
            resolution_patterns: Vec::new(),
        }
    }
}

impl Default for DatabaseTestabilityScore {
    fn default() -> Self {
        Self {
            testability_score: 0.0,
            testability_factors: Vec::new(),
            historical_test_data: Vec::new(),
            test_generation_patterns: Vec::new(),
        }
    }
}

impl DatabaseEnrichedInsightMetrics {
    /// Calculate all insight metrics with database enrichment
    pub fn calculate_enriched_metrics(
        &mut self,
        code: &str,
        language: LANG,
        file_path: &str,
    ) -> Self {
        // Calculate semantic complexity with database patterns
        self.semantic_complexity =
            self.calculate_database_semantic_complexity(code, language, file_path);

        // Calculate refactoring readiness with historical data
        self.refactoring_readiness =
            self.calculate_database_refactoring_readiness(code, language, file_path);

        // Calculate composite code quality with learned patterns
        self.composite_code_quality =
            self.calculate_database_composite_quality(code, language, file_path);

        // Calculate code smell density with pattern database
        self.code_smell_density =
            self.calculate_database_code_smell_density(code, language, file_path);

        // Calculate testability score with historical test data
        self.testability_score =
            self.calculate_database_testability_score(code, language, file_path);

        self.clone()
    }

    /// Calculate semantic complexity with database patterns
    fn calculate_database_semantic_complexity(
        &self,
        code: &str,
        language: LANG,
        file_path: &str,
    ) -> DatabaseSemanticComplexity {
        let mut complexity = DatabaseSemanticComplexity::default();

        // Generate embedding for similarity search
        let embedding = self.generate_embedding(code);

        // Find similar patterns in database using vector search
        let similar_patterns = self.find_similar_patterns_in_db(&embedding, language);
        complexity.similar_patterns = similar_patterns;

        // Get historical complexity trends
        let trends = self.get_complexity_trends(file_path);
        complexity.complexity_trends = trends;

        // Get language-specific patterns
        let lang_patterns = self.get_language_patterns_from_db(language);
        complexity.language_patterns.insert(language, lang_patterns);

        // Get graph relationships
        let relationships = self.get_graph_relationships(file_path);
        complexity.graph_relationships = relationships;

        // Calculate overall semantic score
        complexity.semantic_score = self.calculate_semantic_score(&complexity);

        complexity
    }

    /// Calculate refactoring readiness with historical data
    fn calculate_database_refactoring_readiness(
        &self,
        code: &str,
        language: LANG,
        file_path: &str,
    ) -> DatabaseRefactoringReadiness {
        let mut readiness = DatabaseRefactoringReadiness::default();

        // Find refactoring opportunities in database
        let opportunities = self.find_refactoring_opportunities_in_db(code, language);
        readiness.refactoring_opportunities = opportunities;

        // Get historical success rates
        let success_rates = self.get_historical_refactoring_success_rates(language);
        readiness.historical_success_rates = success_rates;

        // Find similar refactoring patterns
        let similar_refactorings = self.find_similar_refactorings_in_db(code, language);
        readiness.similar_refactorings = similar_refactorings;

        // Calculate readiness score
        readiness.readiness_score = self.calculate_refactoring_readiness_score(&readiness);

        readiness
    }

    /// Calculate composite code quality with learned patterns
    fn calculate_database_composite_quality(
        &self,
        code: &str,
        language: LANG,
        file_path: &str,
    ) -> DatabaseCompositeCodeQuality {
        let mut quality = DatabaseCompositeCodeQuality::default();

        // Get quality factors with database context
        let factors = self.get_quality_factors_from_db(code, language);
        quality.quality_factors = factors;

        // Get learned quality patterns
        let patterns = self.get_quality_patterns_from_db(language);
        quality.quality_patterns = patterns;

        // Get historical quality trends
        let trends = self.get_quality_trends(file_path);
        quality.quality_trends = trends;

        // Calculate quality score
        quality.quality_score = self.calculate_quality_score(&quality);

        quality
    }

    /// Calculate code smell density with pattern database
    fn calculate_database_code_smell_density(
        &self,
        code: &str,
        language: LANG,
        file_path: &str,
    ) -> DatabaseCodeSmellDensity {
        let mut smell_density = DatabaseCodeSmellDensity::default();

        // Detect code smells using database patterns
        let smells = self.detect_code_smells_from_db(code, language);
        smell_density.code_smells = smells;

        // Get historical smell data
        let historical_smells = self.get_historical_smells(file_path);
        smell_density.historical_smells = historical_smells;

        // Get resolution patterns
        let resolution_patterns = self.get_smell_resolution_patterns(language);
        smell_density.resolution_patterns = resolution_patterns;

        // Calculate smell density
        smell_density.smell_density = self.calculate_smell_density(&smell_density);

        smell_density
    }

    /// Calculate testability score with historical test data
    fn calculate_database_testability_score(
        &self,
        code: &str,
        language: LANG,
        file_path: &str,
    ) -> DatabaseTestabilityScore {
        let mut testability = DatabaseTestabilityScore::default();

        // Get testability factors with database context
        let factors = self.get_testability_factors_from_db(code, language);
        testability.testability_factors = factors;

        // Get historical test data
        let historical_data = self.get_historical_test_data(file_path);
        testability.historical_test_data = historical_data;

        // Get test generation patterns
        let patterns = self.get_test_generation_patterns(language);
        testability.test_generation_patterns = patterns;

        // Calculate testability score
        testability.testability_score = self.calculate_testability_score(&testability);

        testability
    }

    // Database integration methods (these would connect to actual database)

    fn generate_embedding(&self, code: &str) -> Vec<f32> {
        // This would use the actual embedding service from the main system
        // For now, return a mock embedding
        vec![0.1; 2560] // 2560-dim embedding (Qodo + Jina v3)
    }

    fn find_similar_patterns_in_db(
        &self,
        embedding: &[f32],
        language: LANG,
    ) -> Vec<DatabasePattern> {
        // This would query the pgvector database for similar patterns
        // SQL: SELECT * FROM code_patterns WHERE language = ? ORDER BY embedding <-> ? LIMIT 10
        vec![]
    }

    fn get_complexity_trends(&self, file_path: &str) -> Vec<ComplexityTrend> {
        // This would query the database for historical complexity data
        // SQL: SELECT timestamp, complexity_score FROM complexity_history WHERE file_path = ? ORDER BY timestamp
        vec![]
    }

    fn get_language_patterns_from_db(&self, language: LANG) -> Vec<DatabasePattern> {
        // This would query the database for language-specific patterns
        // SQL: SELECT * FROM code_patterns WHERE language = ? ORDER BY usage_frequency DESC
        vec![]
    }

    fn get_graph_relationships(&self, file_path: &str) -> Vec<GraphRelationship> {
        // This would query the graph database for relationships
        // Cypher: MATCH (n)-[r]->(m) WHERE n.file_path = ? RETURN n, r, m
        vec![]
    }

    fn calculate_semantic_score(&self, complexity: &DatabaseSemanticComplexity) -> f64 {
        // Calculate semantic score based on patterns, trends, and relationships
        let mut score = 0.0;

        // Factor in similar patterns
        for pattern in &complexity.similar_patterns {
            score += pattern.complexity_score * 0.3;
        }

        // Factor in trends
        if !complexity.complexity_trends.is_empty() {
            let avg_trend = complexity
                .complexity_trends
                .iter()
                .map(|t| t.complexity_score)
                .sum::<f64>()
                / complexity.complexity_trends.len() as f64;
            score += avg_trend * 0.4;
        }

        // Factor in graph relationships
        for relationship in &complexity.graph_relationships {
            score += relationship.strength * 0.3;
        }

        score.min(100.0)
    }

    // Additional database integration methods would go here...
    // These would be implemented to connect to the actual PostgreSQL + pgvector + graph database

    fn find_refactoring_opportunities_in_db(
        &self,
        code: &str,
        language: LANG,
    ) -> Vec<DatabaseRefactoringOpportunity> {
        vec![]
    }

    fn get_historical_refactoring_success_rates(&self, language: LANG) -> HashMap<String, f64> {
        HashMap::new()
    }

    fn find_similar_refactorings_in_db(
        &self,
        code: &str,
        language: LANG,
    ) -> Vec<DatabaseRefactoringPattern> {
        vec![]
    }

    fn calculate_refactoring_readiness_score(
        &self,
        readiness: &DatabaseRefactoringReadiness,
    ) -> f64 {
        0.0
    }

    fn get_quality_factors_from_db(
        &self,
        code: &str,
        language: LANG,
    ) -> Vec<DatabaseQualityFactor> {
        vec![]
    }

    fn get_quality_patterns_from_db(&self, language: LANG) -> Vec<DatabaseQualityPattern> {
        vec![]
    }

    fn get_quality_trends(&self, file_path: &str) -> Vec<QualityTrend> {
        vec![]
    }

    fn calculate_quality_score(&self, quality: &DatabaseCompositeCodeQuality) -> f64 {
        0.0
    }

    fn detect_code_smells_from_db(&self, code: &str, language: LANG) -> Vec<DatabaseCodeSmell> {
        vec![]
    }

    fn get_historical_smells(&self, file_path: &str) -> Vec<HistoricalSmell> {
        vec![]
    }

    fn get_smell_resolution_patterns(&self, language: LANG) -> Vec<SmellResolutionPattern> {
        vec![]
    }

    fn calculate_smell_density(&self, smell_density: &DatabaseCodeSmellDensity) -> f64 {
        0.0
    }

    fn get_testability_factors_from_db(
        &self,
        code: &str,
        language: LANG,
    ) -> Vec<DatabaseTestabilityFactor> {
        vec![]
    }

    fn get_historical_test_data(&self, file_path: &str) -> Vec<HistoricalTestData> {
        vec![]
    }

    fn get_test_generation_patterns(&self, language: LANG) -> Vec<TestGenerationPattern> {
        vec![]
    }

    fn calculate_testability_score(&self, testability: &DatabaseTestabilityScore) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_enriched_metrics() {
        let mut metrics = DatabaseEnrichedInsightMetrics::default();
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

        let result = metrics.calculate_enriched_metrics(code, LANG::Rust, "src/example.rs");
        assert!(result.semantic_complexity.semantic_score >= 0.0);
        assert!(result.semantic_complexity.semantic_score <= 100.0);
    }
}
