//! Insight-driven metrics for best-in-class code analysis
//!
//! This module provides complementary metrics that build on the
//! traditional code analysis suite with semantic understanding,
//! pattern recognition, and deep structural insights.
//!
//! ## Metric Categories
//!
//! ### Complexity & Maintainability
//! - `semantic_complexity` - Language-aware complexity analysis
//! - `refactoring_readiness` - Identifies refactoring opportunities
//! - `code_smell_density` - Detects and quantifies code smells
//!
//! ### Quality & Architecture
//! - `composite_code_quality` - Weighted quality score with factor breakdowns
//! - `testability_score` - Predicts test-ability and modularity
//! - `type_safety` - Type coverage and safety analysis
//!
//! ### Dependencies & Structure
//! - `dependency_coupling` - Measures inter-module coupling strength
//! - `error_handling` - Error path coverage and robustness
//!
//! ### Database Integration
//! - `postgresql_enriched` - PostgreSQL-backed pattern learning

pub mod code_smell_density;
pub mod composite_code_quality;
pub mod database_enriched;
pub mod dependency_coupling;
pub mod error_handling;
pub mod postgresql_enriched;
pub mod refactoring_readiness;
pub mod semantic_complexity;
pub mod testability_score;
pub mod type_safety;

pub use code_smell_density::*;
pub use composite_code_quality::*;
pub use database_enriched::*;
pub use dependency_coupling::*;
pub use error_handling::*;
pub use postgresql_enriched::*;
pub use refactoring_readiness::*;
pub use semantic_complexity::*;
pub use testability_score::*;
pub use type_safety::*;
