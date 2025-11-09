# 🗄️ Database Integration Guide for Singularity Code Analysis Library

## 📚 **Library Architecture Overview**

The `singularity-code-analysis` is a **Rust library (crate)** that provides:

### **Core Library Features**
- ✅ **Traditional Metrics** (12) - ABC, Cognitive, Cyclomatic, Halstead, LOC, etc.
- ✅ **AI/LLM Metrics** (5) - Semantic Complexity, Refactoring Readiness, etc.
- ✅ **Language Support** (11) - Rust, Elixir, JavaScript, Python, Java, etc.
- ✅ **Performance Optimized** - O(1) language detection, inline optimization
- ✅ **Thread-Safe** - Concurrent access support

### **Database Integration Points**

The library is designed to work with external database systems:

#### **1. PostgreSQL + pgvector Integration**
```rust
// Library provides the interface
pub trait DatabaseIntegration {
    fn find_similar_patterns(&self, embedding: &[f32], language: LANG) -> Vec<DatabasePattern>;
    fn get_complexity_trends(&self, file_path: &str) -> Vec<ComplexityTrend>;
    fn store_pattern(&self, pattern: DatabasePattern) -> Result<(), Error>;
}

// Application implements the trait
impl DatabaseIntegration for MyApp {
    fn find_similar_patterns(&self, embedding: &[f32], language: LANG) -> Vec<DatabasePattern> {
        // SQL: SELECT * FROM code_patterns WHERE language = ? ORDER BY embedding <-> ? LIMIT 10
        self.db.query("SELECT * FROM code_patterns WHERE language = $1 ORDER BY embedding <-> $2 LIMIT 10", &[&language, &embedding]).await
    }
}
```

#### **2. Graph Database Integration**
```rust
// Library provides the interface
pub trait GraphDatabaseIntegration {
    fn get_relationships(&self, file_path: &str) -> Vec<GraphRelationship>;
    fn find_similar_code(&self, code: &str) -> Vec<CodeSimilarity>;
    fn store_relationship(&self, rel: GraphRelationship) -> Result<(), Error>;
}

// Application implements the trait
impl GraphDatabaseIntegration for MyApp {
    fn get_relationships(&self, file_path: &str) -> Vec<GraphRelationship> {
        // Cypher: MATCH (n)-[r]->(m) WHERE n.file_path = $1 RETURN n, r, m
        self.neo4j.query("MATCH (n)-[r]->(m) WHERE n.file_path = $1 RETURN n, r, m", &[file_path]).await
    }
}
```

#### **3. Vector Search Integration**
```rust
// Library provides the interface
pub trait VectorSearchIntegration {
    fn generate_embedding(&self, code: &str) -> Vec<f32>;
    fn find_similar_code(&self, embedding: &[f32], threshold: f32) -> Vec<CodeMatch>;
    fn store_embedding(&self, code_id: &str, embedding: &[f32]) -> Result<(), Error>;
}

// Application implements the trait
impl VectorSearchIntegration for MyApp {
    fn generate_embedding(&self, code: &str) -> Vec<f32> {
        // Use your embedding service (Qodo + Jina v3, 2560-dim)
        self.embedding_service.embed(code).await
    }
}
```

## 🔧 **How to Use the Library**

### **1. Basic Usage (No Database)**
```rust
use singularity_code_analysis::{SemanticComplexityStats, LANG};

let mut stats = SemanticComplexityStats::default();
let complexity = stats.calculate_semantic_complexity(code, LANG::Rust);
println!("Semantic complexity: {}", complexity);
```

### **2. With Database Integration**
```rust
use singularity_code_analysis::{
    DatabaseEnrichedInsightMetrics,
    DatabaseIntegration, 
    GraphDatabaseIntegration,
    VectorSearchIntegration
};

// Your application struct
struct MyApp {
    db: PgPool,
    neo4j: Neo4jClient,
    embedding_service: EmbeddingService,
}

// Implement the traits
impl DatabaseIntegration for MyApp { /* ... */ }
impl GraphDatabaseIntegration for MyApp { /* ... */ }
impl VectorSearchIntegration for MyApp { /* ... */ }

// Use the library
let mut metrics = DatabaseEnrichedInsightMetrics::new(Box::new(my_app));
let result = metrics.calculate_enriched_metrics(code, LANG::Rust, "src/example.rs");
```

## 🏗️ **Database Schema Requirements**

### **PostgreSQL Tables**
```sql
-- Code patterns with vector embeddings
CREATE TABLE code_patterns (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    pattern_type VARCHAR(50) NOT NULL,
    complexity_score FLOAT,
    language VARCHAR(20) NOT NULL,
    example TEXT,
    embedding VECTOR(2560), -- pgvector column
    usage_frequency INTEGER DEFAULT 0,
    success_rate FLOAT DEFAULT 0.0,
    last_updated TIMESTAMP DEFAULT NOW(),
    tags TEXT[]
);

-- Create vector index for similarity search
CREATE INDEX ON code_patterns USING ivfflat (embedding vector_cosine_ops);

-- Complexity history
CREATE TABLE complexity_history (
    id UUID PRIMARY KEY,
    file_path VARCHAR(500) NOT NULL,
    complexity_score FLOAT NOT NULL,
    timestamp TIMESTAMP DEFAULT NOW(),
    commit_hash VARCHAR(40)
);

-- Quality trends
CREATE TABLE quality_trends (
    id UUID PRIMARY KEY,
    file_path VARCHAR(500) NOT NULL,
    quality_score FLOAT NOT NULL,
    factor VARCHAR(100) NOT NULL,
    timestamp TIMESTAMP DEFAULT NOW()
);
```

### **Graph Database Schema**
```cypher
// Node types
CREATE CONSTRAINT code_file IF NOT EXISTS FOR (n:CodeFile) REQUIRE n.file_path IS UNIQUE;
CREATE CONSTRAINT code_function IF NOT EXISTS FOR (n:CodeFunction) REQUIRE n.function_id IS UNIQUE;
CREATE CONSTRAINT code_class IF NOT EXISTS FOR (n:CodeClass) REQUIRE n.class_id IS UNIQUE;

// Relationship types
// (CodeFile)-[:CONTAINS]->(CodeFunction)
// (CodeFunction)-[:CALLS]->(CodeFunction)
// (CodeFunction)-[:DEPENDS_ON]->(CodeFunction)
// (CodeClass)-[:INHERITS]->(CodeClass)
```

## 🚀 **Integration with Singularity Main System**

### **1. Add to Cargo.toml**
```toml
[dependencies]
singularity-code-analysis = { path = "packages/singularity-code-analysis" }
```

### **2. Use in Elixir via Rustler**
```elixir
# In your Elixir application
defmodule MyApp.CodeAnalysis do
  use Rustler, otp_app: :my_app, crate: :singularity_code_analysis

  def analyze_code(_code, _language), do: :erlang.nif_error(:nif_not_loaded)
end
```

### **3. Database Integration in Elixir**
```elixir
defmodule MyApp.DatabaseIntegration do
  @behaviour SingularityCodeAnalysis.DatabaseIntegration

  def find_similar_patterns(embedding, language) do
    # Query PostgreSQL with pgvector
    query = """
    SELECT * FROM code_patterns 
    WHERE language = $1 
    ORDER BY embedding <-> $2 
    LIMIT 10
    """
    
    MyApp.Repo.query(query, [language, embedding])
  end
end
```

## 📊 **What the Library Provides**

### **Core Metrics (17 Total)**
1. **Traditional (12)**: ABC, Cognitive, Cyclomatic, Halstead, LOC, MI, NARGS, NOM, NPA, NPM, WMC, Exit
2. **Insight Metrics (5)**: Semantic Complexity, Refactoring Readiness, Composite Code Quality, Code Smell Density, Testability Score

### **Database-Enriched Features**
- **Vector Search** - Find similar code patterns using embeddings
- **Graph Analysis** - Understand code relationships and dependencies
- **Historical Trends** - Track complexity and quality over time
- **Pattern Learning** - Learn from successful refactoring patterns
- **Quality Benchmarks** - Compare against industry standards

### **Performance Features**
- **O(1) Language Detection** - Hash map lookup
- **Thread-Safe** - Concurrent access support
- **Memory Efficient** - Optimized data structures
- **Fast Execution** - < 100ms for most operations

## 🎯 **Benefits for AI/LLM Systems**

### **For Code Generation**
- **Quality Validation** - Ensure AI-generated code meets standards
- **Pattern Recognition** - Identify best practices and anti-patterns
- **Refactoring Guidance** - Suggest improvements to AI-generated code

### **For Code Analysis**
- **Semantic Understanding** - Understand code meaning, not just syntax
- **Comprehensive Metrics** - 17 different quality measures
- **Multi-Language Support** - Works with 11 programming languages

### **For Code Refactoring**
- **Readiness Assessment** - Predict refactoring success
- **Opportunity Identification** - Find refactoring opportunities
- **Historical Learning** - Learn from past refactoring successes

## 🏆 **Conclusion**

The `singularity-code-analysis` library is a **powerful, database-agnostic** code analysis tool that:

1. **Provides Core Metrics** - 17 different code quality measures
2. **Supports Database Integration** - Works with PostgreSQL, Neo4j, etc.
3. **Enables AI/LLM Features** - Semantic understanding, pattern recognition
4. **Offers High Performance** - Optimized for speed and efficiency
5. **Supports Multiple Languages** - 11 programming languages

It's designed to be **integrated into larger systems** like the main Singularity application, where it can leverage the existing database infrastructure for enriched analysis.

The library provides the **interface and algorithms**, while the application provides the **database implementation** - giving you the best of both worlds! 🚀
