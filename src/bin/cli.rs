//! Singularity Analysis Engine CLI
//!
//! A command-line interface for analyzing code quality, complexity, and metrics
//! across multiple programming languages.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Parser)]
#[command(
    name = "singularity-rca",
    about = "Singularity Root Cause Analysis - Multi-language code analysis engine",
    version,
    author
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format
    #[arg(short, long, global = true, default_value = "table")]
    format: OutputFormat,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a file or directory
    Analyze {
        /// Path to file or directory
        path: PathBuf,

        /// Language to analyze (auto-detect if not specified)
        #[arg(short, long)]
        language: Option<String>,

        /// Include insight metrics (advanced analysis)
        #[arg(long)]
        insights: bool,

        /// Recursive directory analysis
        #[arg(short, long)]
        recursive: bool,
    },

    /// Get metrics for a specific file
    Metrics {
        /// Path to file
        path: PathBuf,

        /// Language to analyze
        #[arg(short, long)]
        language: Option<String>,

        /// Show only specific metric
        #[arg(short, long)]
        metric: Option<MetricType>,
    },

    /// List all supported languages
    Languages,

    /// Analyze code complexity
    Complexity {
        /// Path to file or directory
        path: PathBuf,

        /// Threshold for warning (default: 10)
        #[arg(short, long, default_value = "10")]
        threshold: u32,

        /// Show only functions above threshold
        #[arg(long)]
        only_high: bool,
    },

    /// Generate quality report
    Report {
        /// Path to directory
        path: PathBuf,

        /// Output file for report
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Report format
        #[arg(short, long, default_value = "html")]
        format: ReportFormat,
    },

    /// Compare two code versions
    Compare {
        /// Path to first version
        path1: PathBuf,

        /// Path to second version
        path2: PathBuf,

        /// Show improvement/regression
        #[arg(long)]
        diff: bool,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Table,
    Json,
    Pretty,
    Csv,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum MetricType {
    Cyclomatic,
    Cognitive,
    Halstead,
    Loc,
    Maintainability,
    All,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ReportFormat {
    Html,
    Markdown,
    Json,
    Pdf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger
    env_logger::Builder::new()
        .filter_level(if cli.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    log::info!("Singularity Analysis Engine v{}", env!("CARGO_PKG_VERSION"));

    match cli.command {
        Commands::Analyze {
            path,
            language,
            insights,
            recursive,
        } => analyze_command(&path, language, insights, recursive, cli.format)?,
        Commands::Metrics {
            path,
            language,
            metric,
        } => metrics_command(&path, language, metric, cli.format)?,
        Commands::Languages => languages_command(cli.format)?,
        Commands::Complexity {
            path,
            threshold,
            only_high,
        } => complexity_command(&path, threshold, only_high, cli.format)?,
        Commands::Report {
            path,
            output,
            format,
        } => report_command(&path, output, format)?,
        Commands::Compare { path1, path2, diff } => {
            compare_command(&path1, &path2, diff, cli.format)?
        }
    }

    Ok(())
}

fn analyze_command(
    path: &Path,
    _language: Option<String>,
    _insights: bool,
    recursive: bool,
    format: OutputFormat,
) -> Result<()> {
    let start = Instant::now();

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .context("Failed to create spinner template")?,
    );
    spinner.set_message("Analyzing code...");

    let files = if path.is_dir() && recursive {
        collect_files_recursive(path)?
    } else {
        vec![path.to_path_buf()]
    };

    spinner.finish_with_message(format!("Found {} files", files.len()));

    // TODO: Implement actual analysis using singularity_analysis_engine
    // For now, create mock data
    let results = mock_analyze_results(&files);

    match format {
        OutputFormat::Table => display_table(&results),
        OutputFormat::Json => display_json(&results)?,
        OutputFormat::Pretty => display_pretty(&results),
        OutputFormat::Csv => display_csv(&results),
    }

    let elapsed = start.elapsed();
    log::info!("Analysis completed in {:.2}s", elapsed.as_secs_f64());

    Ok(())
}

fn metrics_command(
    path: &Path,
    _language: Option<String>,
    metric: Option<MetricType>,
    format: OutputFormat,
) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File does not exist: {}", path.display());
    }

    if !path.is_file() {
        anyhow::bail!("Path must be a file: {}", path.display());
    }

    log::info!("Analyzing metrics for: {}", path.display());

    // TODO: Implement actual metrics using singularity_analysis_engine
    let metrics = mock_metrics(path);

    match format {
        OutputFormat::Table => display_metrics_table(&metrics, metric),
        OutputFormat::Json => display_metrics_json(&metrics, metric)?,
        OutputFormat::Pretty => display_metrics_pretty(&metrics, metric),
        OutputFormat::Csv => display_metrics_csv(&metrics, metric),
    }

    Ok(())
}

fn languages_command(format: OutputFormat) -> Result<()> {
    let languages = vec![
        ("Rust", "✓", "Full support"),
        ("Python", "✓", "Full support"),
        ("JavaScript", "✓", "Full support"),
        ("TypeScript", "✓", "Full support"),
        ("Java", "✓", "Full support with WMC"),
        ("C/C++", "✓", "Full support"),
        ("Elixir", "✓", "BEAM language support"),
        ("Erlang", "✓", "BEAM language support"),
        ("Gleam", "✓", "BEAM language support"),
        ("Go", "⚠", "Partial metrics"),
        ("Kotlin", "⚠", "Partial metrics"),
        ("C#", "✓", "Full support"),
        ("Lua", "✓", "Full support"),
    ];

    match format {
        OutputFormat::Table | OutputFormat::Pretty => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(vec!["Language", "Status", "Notes"]);

            for (lang, status, notes) in &languages {
                let status_cell = if *status == "✓" {
                    Cell::new(status).fg(Color::Green)
                } else {
                    Cell::new(status).fg(Color::Yellow)
                };
                table.add_row(vec![Cell::new(lang), status_cell, Cell::new(notes)]);
            }

            println!("{table}");
        }
        OutputFormat::Json => {
            let json = json!({
                "languages": languages.iter().map(|(lang, status, notes)| {
                    json!({
                        "name": lang,
                        "status": status,
                        "notes": notes
                    })
                }).collect::<Vec<_>>()
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Csv => {
            println!("Language,Status,Notes");
            for (lang, status, notes) in &languages {
                println!("{},{},{}", lang, status, notes);
            }
        }
    }

    Ok(())
}

fn complexity_command(
    path: &Path,
    threshold: u32,
    only_high: bool,
    format: OutputFormat,
) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    log::info!("Analyzing complexity (threshold: {})...", threshold);

    // TODO: Implement actual complexity analysis
    let complexities = mock_complexity_data(path, threshold, only_high);

    match format {
        OutputFormat::Table | OutputFormat::Pretty => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(vec![
                "File",
                "Function",
                "Cyclomatic",
                "Cognitive",
                "Status",
            ]);

            for item in &complexities {
                let status = if item.cyclomatic > threshold {
                    Cell::new("⚠ HIGH").fg(Color::Red)
                } else {
                    Cell::new("✓ OK").fg(Color::Green)
                };
                table.add_row(vec![
                    Cell::new(&item.file),
                    Cell::new(&item.function),
                    Cell::new(item.cyclomatic),
                    Cell::new(item.cognitive),
                    status,
                ]);
            }

            println!("{table}");
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&complexities)?);
        }
        OutputFormat::Csv => {
            println!("File,Function,Cyclomatic,Cognitive,Status");
            for item in &complexities {
                let status = if item.cyclomatic > threshold {
                    "HIGH"
                } else {
                    "OK"
                };
                println!(
                    "{},{},{},{},{}",
                    item.file, item.function, item.cyclomatic, item.cognitive, status
                );
            }
        }
    }

    Ok(())
}

fn report_command(path: &Path, output: Option<PathBuf>, format: ReportFormat) -> Result<()> {
    if !path.exists() || !path.is_dir() {
        anyhow::bail!("Path must be a valid directory: {}", path.display());
    }

    log::info!(
        "Generating {} report for: {}",
        format_name(format),
        path.display()
    );

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Generating report...");

    // TODO: Implement actual report generation
    let report_content = generate_mock_report(path, format);

    spinner.finish_and_clear();

    match output {
        Some(out_path) => {
            fs::write(&out_path, report_content)
                .context(format!("Failed to write report to: {}", out_path.display()))?;
            log::info!("Report saved to: {}", out_path.display());
        }
        None => {
            println!("{report_content}");
        }
    }

    Ok(())
}

fn compare_command(path1: &Path, path2: &Path, _diff: bool, format: OutputFormat) -> Result<()> {
    if !path1.exists() || !path2.exists() {
        anyhow::bail!("Both paths must exist");
    }

    log::info!(
        "Comparing:\n  {} vs\n  {}",
        path1.display(),
        path2.display()
    );

    // TODO: Implement actual comparison
    let comparison = mock_comparison(path1, path2);

    match format {
        OutputFormat::Table | OutputFormat::Pretty => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(vec!["Metric", "Before", "After", "Change"]);

            for (metric, before, after, change) in &comparison {
                let change_cell = if change.starts_with('+') {
                    Cell::new(change).fg(Color::Red)
                } else if change.starts_with('-') {
                    Cell::new(change).fg(Color::Green)
                } else {
                    Cell::new(change)
                };
                table.add_row(vec![
                    Cell::new(metric),
                    Cell::new(before),
                    Cell::new(after),
                    change_cell,
                ]);
            }

            println!("{table}");
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&comparison)?);
        }
        OutputFormat::Csv => {
            println!("Metric,Before,After,Change");
            for (metric, before, after, change) in &comparison {
                println!("{},{},{},{}", metric, before, after, change);
            }
        }
    }

    Ok(())
}

// Helper functions

fn collect_files_recursive(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(dir).follow_links(true) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if is_source_file(ext.to_str().unwrap_or("")) {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    Ok(files)
}

fn collect_files_single(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(ext) = entry.path().extension() {
                if is_source_file(ext.to_str().unwrap_or("")) {
                    files.push(entry.path());
                }
            }
        }
    }
    Ok(files)
}

fn is_source_file(ext: &str) -> bool {
    matches!(
        ext,
        "rs" | "py"
            | "js"
            | "ts"
            | "tsx"
            | "jsx"
            | "java"
            | "c"
            | "cpp"
            | "h"
            | "hpp"
            | "ex"
            | "exs"
            | "erl"
            | "hrl"
            | "gleam"
            | "go"
            | "kt"
            | "cs"
            | "lua"
    )
}

fn format_name(format: ReportFormat) -> &'static str {
    match format {
        ReportFormat::Html => "HTML",
        ReportFormat::Markdown => "Markdown",
        ReportFormat::Json => "JSON",
        ReportFormat::Pdf => "PDF",
    }
}

// Mock data generators (TODO: Replace with actual implementation)

#[derive(serde::Serialize)]
struct AnalysisResult {
    file: String,
    lines: u32,
    complexity: u32,
    functions: u32,
}

fn mock_analyze_results(files: &[PathBuf]) -> Vec<AnalysisResult> {
    files
        .iter()
        .map(|f| AnalysisResult {
            file: f.display().to_string(),
            lines: 100,
            complexity: 5,
            functions: 3,
        })
        .collect()
}

#[derive(serde::Serialize)]
struct MetricsData {
    cyclomatic: u32,
    cognitive: u32,
    loc: u32,
    maintainability: f64,
}

fn mock_metrics(_path: &Path) -> MetricsData {
    MetricsData {
        cyclomatic: 5,
        cognitive: 8,
        loc: 120,
        maintainability: 85.5,
    }
}

#[derive(serde::Serialize)]
struct ComplexityItem {
    file: String,
    function: String,
    cyclomatic: u32,
    cognitive: u32,
}

fn mock_complexity_data(path: &Path, threshold: u32, only_high: bool) -> Vec<ComplexityItem> {
    let items = vec![
        ComplexityItem {
            file: path.display().to_string(),
            function: "example_fn".to_string(),
            cyclomatic: 15,
            cognitive: 20,
        },
        ComplexityItem {
            file: path.display().to_string(),
            function: "another_fn".to_string(),
            cyclomatic: 5,
            cognitive: 7,
        },
    ];

    if only_high {
        items
            .into_iter()
            .filter(|i| i.cyclomatic > threshold)
            .collect()
    } else {
        items
    }
}

fn generate_mock_report(_path: &Path, format: ReportFormat) -> String {
    match format {
        ReportFormat::Html => "<html><body><h1>Code Quality Report</h1></body></html>".to_string(),
        ReportFormat::Markdown => "# Code Quality Report\n\nGenerated report...".to_string(),
        ReportFormat::Json => r#"{"report": "mock data"}"#.to_string(),
        ReportFormat::Pdf => "PDF generation not yet implemented".to_string(),
    }
}

fn mock_comparison(_path1: &Path, _path2: &Path) -> Vec<(String, String, String, String)> {
    vec![
        (
            "Cyclomatic Complexity".to_string(),
            "45".to_string(),
            "38".to_string(),
            "-7 (↓ 15.6%)".to_string(),
        ),
        (
            "Lines of Code".to_string(),
            "1250".to_string(),
            "1180".to_string(),
            "-70 (↓ 5.6%)".to_string(),
        ),
        (
            "Functions".to_string(),
            "23".to_string(),
            "25".to_string(),
            "+2 (↑ 8.7%)".to_string(),
        ),
    ]
}

// Display functions

fn display_table(results: &[AnalysisResult]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["File", "Lines", "Complexity", "Functions"]);

    for result in results {
        table.add_row(vec![
            &result.file,
            &result.lines.to_string(),
            &result.complexity.to_string(),
            &result.functions.to_string(),
        ]);
    }

    println!("{table}");
}

fn display_json(results: &[AnalysisResult]) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(results)?);
    Ok(())
}

fn display_pretty(results: &[AnalysisResult]) {
    for result in results {
        println!("📄 {}", result.file);
        println!("  Lines: {}", result.lines);
        println!("  Complexity: {}", result.complexity);
        println!("  Functions: {}", result.functions);
        println!();
    }
}

fn display_csv(results: &[AnalysisResult]) {
    println!("File,Lines,Complexity,Functions");
    for result in results {
        println!(
            "{},{},{},{}",
            result.file, result.lines, result.complexity, result.functions
        );
    }
}

fn display_metrics_table(metrics: &MetricsData, filter: Option<MetricType>) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Metric", "Value"]);

    let show_all = filter.is_none() || matches!(filter, Some(MetricType::All));

    if show_all || matches!(filter, Some(MetricType::Cyclomatic)) {
        table.add_row(vec![
            "Cyclomatic Complexity",
            &metrics.cyclomatic.to_string(),
        ]);
    }
    if show_all || matches!(filter, Some(MetricType::Cognitive)) {
        table.add_row(vec!["Cognitive Complexity", &metrics.cognitive.to_string()]);
    }
    if show_all || matches!(filter, Some(MetricType::Loc)) {
        table.add_row(vec!["Lines of Code", &metrics.loc.to_string()]);
    }
    if show_all || matches!(filter, Some(MetricType::Maintainability)) {
        table.add_row(vec![
            "Maintainability Index",
            &format!("{:.1}", metrics.maintainability),
        ]);
    }

    println!("{table}");
}

fn display_metrics_json(metrics: &MetricsData, _filter: Option<MetricType>) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(metrics)?);
    Ok(())
}

fn display_metrics_pretty(metrics: &MetricsData, filter: Option<MetricType>) {
    let show_all = filter.is_none() || matches!(filter, Some(MetricType::All));

    println!("📊 Code Metrics\n");

    if show_all || matches!(filter, Some(MetricType::Cyclomatic)) {
        println!("  Cyclomatic Complexity: {}", metrics.cyclomatic);
    }
    if show_all || matches!(filter, Some(MetricType::Cognitive)) {
        println!("  Cognitive Complexity: {}", metrics.cognitive);
    }
    if show_all || matches!(filter, Some(MetricType::Loc)) {
        println!("  Lines of Code: {}", metrics.loc);
    }
    if show_all || matches!(filter, Some(MetricType::Maintainability)) {
        println!("  Maintainability Index: {:.1}", metrics.maintainability);
    }
}

fn display_metrics_csv(metrics: &MetricsData, _filter: Option<MetricType>) {
    println!("Metric,Value");
    println!("Cyclomatic Complexity,{}", metrics.cyclomatic);
    println!("Cognitive Complexity,{}", metrics.cognitive);
    println!("Lines of Code,{}", metrics.loc);
    println!("Maintainability Index,{:.1}", metrics.maintainability);
}
