//! PostgreSQL + pgvector enriched insight metrics type definitions.
//!
//! The structures in this module model the payloads exchanged with a
//! PostgreSQL/pgvector backend.  They do not execute database calls on
//! their own—those live in the host integration layer (see
//! `POSTGRESQL_INTEGRATION_GUIDE.md` for wiring details).  Keeping the
//! types colocated with the rest of the insight metrics ensures that the
//! Rust <-> BEAM FFI boundary remains strongly typed even when the actual
//! queries are implemented elsewhere.

use crate::langs::LANG;
use std::collections::HashMap;

/// PostgreSQL-enriched insight metrics that leverage vector search and relational data
#[derive(Debug, Clone, Default)]
pub struct PostgreSQLEnrichedInsightMetrics {
    /// Semantic complexity with database patterns
    pub semantic_complexity: PostgreSQLSemanticComplexity,
    /// Refactoring readiness with historical data
    pub refactoring_readiness: PostgreSQLRefactoringReadiness,
    /// Composite code quality with learned patterns
    pub composite_code_quality: PostgreSQLCompositeCodeQuality,
    /// Code smell density with pattern database
    pub code_smell_density: PostgreSQLCodeSmellDensity,
    /// Testability score with historical test data
    pub testability_score: PostgreSQLTestabilityScore,
}

/// PostgreSQL-enriched semantic complexity
#[derive(Debug, Clone)]
pub struct PostgreSQLSemanticComplexity {
    /// Overall semantic complexity score (0-100)
    pub semantic_score: f64,
    /// Similar patterns from database using pgvector
    pub similar_patterns: Vec<PostgreSQLPattern>,
    /// Historical complexity trends
    pub complexity_trends: Vec<ComplexityTrend>,
    /// Language-specific patterns from database
    pub language_patterns: HashMap<LANG, Vec<PostgreSQLPattern>>,
    /// Code relationships from PostgreSQL
    pub code_relationships: Vec<CodeRelationship>,
}

/// PostgreSQL pattern with full metadata
#[derive(Debug, Clone)]
pub struct PostgreSQLPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pattern_type: PatternType,
    pub complexity_score: f64,
    pub language: LANG,
    pub example: String,
    /// Vector embedding for similarity search (pgvector)
    pub embedding: Vec<f32>,
    /// Usage frequency in database
    pub usage_frequency: u32,
    /// Success rate when used
    pub success_rate: f64,
    /// Last updated timestamp
    pub last_updated: String,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Similarity score from pgvector search
    pub similarity_score: f64,
}

/// Pattern types from database
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ComplexityTrend {
    pub timestamp: String,
    pub complexity_score: f64,
    pub file_path: String,
    pub commit_hash: String,
}

/// Code relationship from PostgreSQL
#[derive(Debug, Clone)]
pub struct CodeRelationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub metadata: HashMap<String, String>,
}

/// Types of code relationships
#[derive(Debug, Clone)]
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

/// PostgreSQL-enriched refactoring readiness
#[derive(Debug, Clone)]
pub struct PostgreSQLRefactoringReadiness {
    pub readiness_score: f64,
    /// Refactoring opportunities from database
    pub refactoring_opportunities: Vec<PostgreSQLRefactoringOpportunity>,
    /// Historical refactoring success rates
    pub historical_success_rates: HashMap<String, f64>,
    /// Similar refactoring patterns
    pub similar_refactorings: Vec<PostgreSQLRefactoringPattern>,
}

/// PostgreSQL refactoring opportunity
#[derive(Debug, Clone)]
pub struct PostgreSQLRefactoringOpportunity {
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

/// PostgreSQL refactoring pattern
#[derive(Debug, Clone)]
pub struct PostgreSQLRefactoringPattern {
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

/// PostgreSQL-enriched composite code quality
#[derive(Debug, Clone)]
pub struct PostgreSQLCompositeCodeQuality {
    pub quality_score: f64,
    /// Quality factors with database context
    pub quality_factors: Vec<PostgreSQLQualityFactor>,
    /// Learned quality patterns
    pub quality_patterns: Vec<PostgreSQLQualityPattern>,
    /// Historical quality trends
    pub quality_trends: Vec<QualityTrend>,
}

/// PostgreSQL quality factor
#[derive(Debug, Clone)]
pub struct PostgreSQLQualityFactor {
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

/// PostgreSQL quality pattern
#[derive(Debug, Clone)]
pub struct PostgreSQLQualityPattern {
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
#[derive(Debug, Clone)]
pub struct QualityTrend {
    pub timestamp: String,
    pub quality_score: f64,
    pub factor: String,
    pub file_path: String,
}

/// PostgreSQL-enriched code smell density
#[derive(Debug, Clone)]
pub struct PostgreSQLCodeSmellDensity {
    pub smell_density: f64,
    /// Code smells from database
    pub code_smells: Vec<PostgreSQLCodeSmell>,
    /// Historical smell patterns
    pub historical_smells: Vec<HistoricalSmell>,
    /// Smell resolution patterns
    pub resolution_patterns: Vec<SmellResolutionPattern>,
}

/// PostgreSQL code smell
#[derive(Debug, Clone)]
pub struct PostgreSQLCodeSmell {
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
#[derive(Debug, Clone)]
pub struct HistoricalSmell {
    pub timestamp: String,
    pub smell_type: String,
    pub severity: f64,
    pub file_path: String,
    pub resolved: bool,
    pub resolution_time: Option<u32>,
}

/// Smell resolution pattern
#[derive(Debug, Clone)]
pub struct SmellResolutionPattern {
    pub id: String,
    pub smell_type: String,
    pub resolution_approach: String,
    pub success_rate: f64,
    pub average_time: u32,
    pub example: String,
}

/// PostgreSQL-enriched testability score
#[derive(Debug, Clone)]
pub struct PostgreSQLTestabilityScore {
    pub testability_score: f64,
    /// Testability factors with database context
    pub testability_factors: Vec<PostgreSQLTestabilityFactor>,
    /// Historical test data
    pub historical_test_data: Vec<HistoricalTestData>,
    /// Test generation patterns
    pub test_generation_patterns: Vec<TestGenerationPattern>,
}

/// PostgreSQL testability factor
#[derive(Debug, Clone)]
pub struct PostgreSQLTestabilityFactor {
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
#[derive(Debug, Clone)]
pub struct HistoricalTestData {
    pub timestamp: String,
    pub test_type: String,
    pub success_rate: f64,
    pub coverage: f64,
    pub file_path: String,
    pub test_count: u32,
}

/// Test generation pattern
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct CodeLocation {
    pub file_path: String,
    pub line_start: usize,
    pub line_end: usize,
    pub column_start: usize,
    pub column_end: usize,
}

/// Code features extracted from embedding
#[derive(Debug, Clone)]
pub struct CodeFeatures {
    pub complexity: f32,
    pub function_count: u32,
    pub loop_count: u32,
    pub condition_count: u32,
    pub nesting_depth: u32,
    pub comment_ratio: f32,
    pub string_literal_count: u32,
    pub keyword_scores: Vec<f32>,
}

/// Language-specific pattern template
#[derive(Debug, Clone)]
pub struct LanguagePattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pattern_type: PatternType,
    pub complexity_score: f64,
    pub example: String,
    pub usage_frequency: u32,
    pub success_rate: f64,
    pub last_updated: String,
    pub tags: Vec<String>,
    pub features: CodeFeatures,
}

impl Default for PostgreSQLSemanticComplexity {
    fn default() -> Self {
        Self {
            semantic_score: 0.0,
            similar_patterns: Vec::new(),
            complexity_trends: Vec::new(),
            language_patterns: HashMap::new(),
            code_relationships: Vec::new(),
        }
    }
}

impl Default for PostgreSQLRefactoringReadiness {
    fn default() -> Self {
        Self {
            readiness_score: 0.0,
            refactoring_opportunities: Vec::new(),
            historical_success_rates: HashMap::new(),
            similar_refactorings: Vec::new(),
        }
    }
}

impl Default for PostgreSQLCompositeCodeQuality {
    fn default() -> Self {
        Self {
            quality_score: 0.0,
            quality_factors: Vec::new(),
            quality_patterns: Vec::new(),
            quality_trends: Vec::new(),
        }
    }
}

impl Default for PostgreSQLCodeSmellDensity {
    fn default() -> Self {
        Self {
            smell_density: 0.0,
            code_smells: Vec::new(),
            historical_smells: Vec::new(),
            resolution_patterns: Vec::new(),
        }
    }
}

impl Default for PostgreSQLTestabilityScore {
    fn default() -> Self {
        Self {
            testability_score: 0.0,
            testability_factors: Vec::new(),
            historical_test_data: Vec::new(),
            test_generation_patterns: Vec::new(),
        }
    }
}
