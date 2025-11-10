//! Code Evolution Tracking for AI Learning
//!
//! Pure calculation functions for tracking code evolution patterns.
//! Elixir handles orchestration, state management, and database operations.

/// Calculate code evolution trends from version history
///
/// # Arguments
/// * `complexity_values` - Historical complexity values
/// * `maintainability_values` - Historical maintainability values  
/// * `test_coverage_values` - Historical test coverage values
///
/// # Returns
/// * `(complexity_trend, maintainability_trend, test_coverage_trend)`
#[inline]
fn len_to_f64(len: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    {
        len as f64
    }
}

#[inline]
fn usize_to_rate(count: usize, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    len_to_f64(count) / len_to_f64(total)
}

#[inline]
#[must_use]
pub fn calculate_evolution_trends(
    complexity_values: &[f64],
    maintainability_values: &[f64],
    test_coverage_values: &[f64],
) -> (TrendDirection, TrendDirection, TrendDirection) {
    let complexity_trend = calculate_trend(complexity_values);
    let maintainability_trend = calculate_trend(maintainability_values);
    let test_coverage_trend = calculate_trend(test_coverage_values);

    (complexity_trend, maintainability_trend, test_coverage_trend)
}

/// Calculate trend direction from a series of values
#[inline]
#[must_use]
pub fn calculate_trend(values: &[f64]) -> TrendDirection {
    if values.len() < 2 {
        return TrendDirection::Stable;
    }

    let first_half = &values[..values.len() / 2];
    let second_half = &values[values.len() / 2..];

    let first_avg = first_half.iter().sum::<f64>() / len_to_f64(first_half.len());
    let second_avg = second_half.iter().sum::<f64>() / len_to_f64(second_half.len());

    let change_percentage = (second_avg - first_avg) / first_avg * 100.0;

    if change_percentage > 5.0 {
        TrendDirection::Increasing
    } else if change_percentage < -5.0 {
        TrendDirection::Decreasing
    } else {
        TrendDirection::Stable
    }
}

/// Detect refactoring events from before/after metrics
///
/// # Arguments
/// * `before_metrics` - Metrics before change
/// * `after_metrics` - Metrics after change
///
/// # Returns
/// * Vector of detected refactoring events
#[inline]
#[must_use]
pub fn detect_refactoring_events(
    before_metrics: &EvolutionMetrics,
    after_metrics: &EvolutionMetrics,
) -> Vec<RefactoringEvent> {
    let mut events = Vec::new();

    // Detect extract method refactoring
    if let Some(event) = detect_extract_method(before_metrics, after_metrics) {
        events.push(event);
    }

    // Detect extract class refactoring
    if let Some(event) = detect_extract_class(before_metrics, after_metrics) {
        events.push(event);
    }

    // Detect remove duplication refactoring
    if let Some(event) = detect_remove_duplication(before_metrics, after_metrics) {
        events.push(event);
    }

    // Detect simplify conditional refactoring
    if let Some(event) = detect_simplify_conditional(before_metrics, after_metrics) {
        events.push(event);
    }

    events
}

/// Calculate improvement score between two metric sets
#[inline]
#[must_use]
pub fn calculate_improvement_score(before: &EvolutionMetrics, after: &EvolutionMetrics) -> f64 {
    let complexity_improvement = (f64::from(before.cyclomatic_complexity)
        - f64::from(after.cyclomatic_complexity))
        / f64::from(before.cyclomatic_complexity.max(1));
    let maintainability_improvement =
        (after.maintainability_index - before.maintainability_index) / 100.0;
    let test_coverage_improvement = (after.test_coverage - before.test_coverage) / 100.0;

    (complexity_improvement + maintainability_improvement + test_coverage_improvement) / 3.0
}

/// Calculate bug introduction rate from version history
#[inline]
#[must_use]
pub fn calculate_bug_introduction_rate(technical_debt_values: &[f64]) -> f64 {
    if technical_debt_values.len() < 2 {
        return 0.0;
    }

    let increases = technical_debt_values
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();

    usize_to_rate(increases, technical_debt_values.len() - 1)
}

/// Calculate improvement success rate from version history
#[inline]
#[must_use]
pub fn calculate_improvement_success_rate(maintainability_values: &[f64]) -> f64 {
    if maintainability_values.len() < 2 {
        return 0.0;
    }

    let improvements = maintainability_values
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();

    usize_to_rate(improvements, maintainability_values.len() - 1)
}

/// Predict future quality based on trends
#[inline]
#[must_use]
pub fn predict_future_quality(
    current_metrics: &EvolutionMetrics,
    complexity_trend: TrendDirection,
    maintainability_trend: TrendDirection,
    test_coverage_trend: TrendDirection,
) -> EvolutionPrediction {
    EvolutionPrediction {
        predicted_complexity: predict_complexity(current_metrics, complexity_trend),
        predicted_maintainability: predict_maintainability(current_metrics, maintainability_trend),
        predicted_test_coverage: predict_test_coverage(current_metrics, test_coverage_trend),
        confidence_score: calculate_prediction_confidence(complexity_trend, maintainability_trend),
    }
}

// Private helper functions

fn detect_extract_method(
    before: &EvolutionMetrics,
    after: &EvolutionMetrics,
) -> Option<RefactoringEvent> {
    if after.function_count > before.function_count
        && after.cyclomatic_complexity < before.cyclomatic_complexity
    {
        Some(RefactoringEvent {
            refactoring_type: RefactoringType::ExtractMethod,
            improvement_score: calculate_improvement_score(before, after),
            complexity_reduction: f64::from(before.cyclomatic_complexity)
                - f64::from(after.cyclomatic_complexity),
            maintainability_improvement: after.maintainability_index - before.maintainability_index,
        })
    } else {
        None
    }
}

fn detect_extract_class(
    before: &EvolutionMetrics,
    after: &EvolutionMetrics,
) -> Option<RefactoringEvent> {
    if after.class_count > before.class_count && after.function_count > before.function_count {
        Some(RefactoringEvent {
            refactoring_type: RefactoringType::ExtractClass,
            improvement_score: calculate_improvement_score(before, after),
            complexity_reduction: f64::from(before.cyclomatic_complexity)
                - f64::from(after.cyclomatic_complexity),
            maintainability_improvement: after.maintainability_index - before.maintainability_index,
        })
    } else {
        None
    }
}

fn detect_remove_duplication(
    before: &EvolutionMetrics,
    after: &EvolutionMetrics,
) -> Option<RefactoringEvent> {
    if after.lines_of_code < before.lines_of_code
        && after.cyclomatic_complexity < before.cyclomatic_complexity
    {
        Some(RefactoringEvent {
            refactoring_type: RefactoringType::RemoveDuplication,
            improvement_score: calculate_improvement_score(before, after),
            complexity_reduction: f64::from(before.cyclomatic_complexity)
                - f64::from(after.cyclomatic_complexity),
            maintainability_improvement: after.maintainability_index - before.maintainability_index,
        })
    } else {
        None
    }
}

fn detect_simplify_conditional(
    before: &EvolutionMetrics,
    after: &EvolutionMetrics,
) -> Option<RefactoringEvent> {
    if after.cyclomatic_complexity < before.cyclomatic_complexity
        && after.cognitive_complexity < before.cognitive_complexity
    {
        Some(RefactoringEvent {
            refactoring_type: RefactoringType::SimplifyConditional,
            improvement_score: calculate_improvement_score(before, after),
            complexity_reduction: f64::from(before.cyclomatic_complexity)
                - f64::from(after.cyclomatic_complexity),
            maintainability_improvement: after.maintainability_index - before.maintainability_index,
        })
    } else {
        None
    }
}

fn predict_complexity(current: &EvolutionMetrics, trend: TrendDirection) -> f64 {
    match trend {
        TrendDirection::Increasing => f64::from(current.cyclomatic_complexity) * 1.1,
        TrendDirection::Decreasing => f64::from(current.cyclomatic_complexity) * 0.9,
        TrendDirection::Stable => f64::from(current.cyclomatic_complexity),
    }
}

fn predict_maintainability(current: &EvolutionMetrics, trend: TrendDirection) -> f64 {
    match trend {
        TrendDirection::Increasing => (current.maintainability_index + 5.0).min(100.0),
        TrendDirection::Decreasing => (current.maintainability_index - 5.0).max(0.0),
        TrendDirection::Stable => current.maintainability_index,
    }
}

fn predict_test_coverage(current: &EvolutionMetrics, trend: TrendDirection) -> f64 {
    match trend {
        TrendDirection::Increasing => (current.test_coverage + 5.0).min(100.0),
        TrendDirection::Decreasing => (current.test_coverage - 5.0).max(0.0),
        TrendDirection::Stable => current.test_coverage,
    }
}

fn calculate_prediction_confidence(
    complexity_trend: TrendDirection,
    maintainability_trend: TrendDirection,
) -> f64 {
    let mut confidence = 0.7_f64; // Base confidence

    // Increase confidence if trends are consistent
    if complexity_trend == maintainability_trend {
        confidence += 0.1;
    }

    // Increase confidence for stable trends
    if complexity_trend == TrendDirection::Stable && maintainability_trend == TrendDirection::Stable
    {
        confidence += 0.1;
    }

    confidence.clamp(0.0_f64, 1.0_f64)
}

/// Trend direction
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

/// Code metrics at a point in time
#[derive(Debug, Clone)]
pub struct EvolutionMetrics {
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: f64,
    pub lines_of_code: u32,
    pub function_count: u32,
    pub class_count: u32,
    pub test_coverage: f64,
    pub maintainability_index: f64,
    pub technical_debt_score: f64,
}

/// A refactoring event detected in code evolution
#[derive(Debug, Clone)]
pub struct RefactoringEvent {
    pub refactoring_type: RefactoringType,
    pub improvement_score: f64,
    pub complexity_reduction: f64,
    pub maintainability_improvement: f64,
}

/// Types of refactoring events
#[derive(Debug, Clone)]
pub enum RefactoringType {
    ExtractMethod,
    ExtractClass,
    RemoveDuplication,
    SimplifyConditional,
}

/// Quality prediction based on evolution patterns
#[derive(Debug, Clone)]
pub struct EvolutionPrediction {
    pub predicted_complexity: f64,
    pub predicted_maintainability: f64,
    pub predicted_test_coverage: f64,
    pub confidence_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_trend() {
        let increasing = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let decreasing = vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let stable = vec![3.0, 3.1, 2.9, 3.0, 3.2, 2.8];

        assert_eq!(calculate_trend(&increasing), TrendDirection::Increasing);
        assert_eq!(calculate_trend(&decreasing), TrendDirection::Decreasing);
        assert_eq!(calculate_trend(&stable), TrendDirection::Stable);
    }

    #[test]
    fn test_calculate_improvement_score() {
        let before = EvolutionMetrics {
            cyclomatic_complexity: 10,
            cognitive_complexity: 8.0,
            lines_of_code: 100,
            function_count: 5,
            class_count: 1,
            test_coverage: 60.0,
            maintainability_index: 50.0,
            technical_debt_score: 40.0,
        };

        let after = EvolutionMetrics {
            cyclomatic_complexity: 8,
            cognitive_complexity: 6.0,
            lines_of_code: 90,
            function_count: 7,
            class_count: 2,
            test_coverage: 75.0,
            maintainability_index: 65.0,
            technical_debt_score: 25.0,
        };

        let score = calculate_improvement_score(&before, &after);
        assert!(score > 0.0);
    }

    #[test]
    fn test_detect_refactoring_events() {
        let before = EvolutionMetrics {
            cyclomatic_complexity: 15,
            cognitive_complexity: 10.0,
            lines_of_code: 200,
            function_count: 5,
            class_count: 1,
            test_coverage: 60.0,
            maintainability_index: 50.0,
            technical_debt_score: 40.0,
        };

        let after = EvolutionMetrics {
            cyclomatic_complexity: 10,
            cognitive_complexity: 7.0,
            lines_of_code: 180,
            function_count: 8,
            class_count: 2,
            test_coverage: 75.0,
            maintainability_index: 65.0,
            technical_debt_score: 25.0,
        };

        let events = detect_refactoring_events(&before, &after);
        assert!(!events.is_empty());
    }
}
