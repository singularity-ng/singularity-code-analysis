//! Dependency Coupling Score - Measures inter-module/inter-package coupling strength
//!
//! Detects cyclic dependencies, import chains, and architectural violations
//! to assess maintainability and testability of code structure.

use std::collections::{HashMap, HashSet};

/// Dependency Coupling Metrics
#[derive(Debug, Clone, PartialEq)]
pub struct DependencyCouplingMetrics {
    /// Overall coupling score (0-100, higher = lower coupling = better)
    pub coupling_score: f64,
    /// Number of direct imports (normalized)
    pub import_density: f64,
    /// Number of cyclic dependencies detected
    pub cyclic_dependencies: usize,
    /// Deepest import chain found
    pub max_import_chain_depth: usize,
    /// Number of layer violations
    pub layer_violations: usize,
    /// Ratio of external imports to total imports
    pub external_import_ratio: f64,
    /// Cyclic dependency chains
    pub cycles: Vec<Vec<String>>,
    /// All detected imports with their targets
    pub import_graph: HashMap<String, Vec<String>>,
}

impl DependencyCouplingMetrics {
    /// Calculate coupling score using the formula:
    /// Score = 100 - (
    ///   0.3 * (imports/LOC).clamp(0,10) * 10 +
    ///   0.25 * cyclic_count +
    ///   0.2 * depth.clamp(0,5) * 20 +
    ///   0.15 * violations +
    ///   0.1 * (external/total).clamp(0,1) * 10
    /// )
    pub fn calculate(
        import_density: f64,
        cyclic_count: usize,
        max_depth: usize,
        violations: usize,
        external_ratio: f64,
        cycles: Vec<Vec<String>>,
        import_graph: HashMap<String, Vec<String>>,
    ) -> Self {
        let density_penalty = (import_density / 10.0).clamp(0.0, 1.0) * 10.0 * 0.3;
        let cyclic_penalty = (cyclic_count as f64) * 0.25;
        let depth_penalty = (max_depth as f64 / 5.0).clamp(0.0, 1.0) * 20.0 * 0.2;
        let violation_penalty = (violations as f64) * 0.15;
        let external_penalty = external_ratio.clamp(0.0, 1.0) * 10.0 * 0.1;

        let total_penalty =
            density_penalty + cyclic_penalty + depth_penalty + violation_penalty + external_penalty;
        let coupling_score = (100.0 - total_penalty).clamp(0.0, 100.0);

        Self {
            coupling_score,
            import_density,
            cyclic_dependencies: cyclic_count,
            max_import_chain_depth: max_depth,
            layer_violations: violations,
            external_import_ratio: external_ratio.clamp(0.0, 1.0),
            cycles,
            import_graph,
        }
    }

    /// Analyze coupling from import statements
    pub fn from_imports(imports: &[(String, String)]) -> Self {
        let mut import_graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut all_modules = HashSet::new();

        for (from, to) in imports {
            all_modules.insert(from.clone());
            all_modules.insert(to.clone());
            import_graph
                .entry(from.clone())
                .or_default()
                .push(to.clone());
        }

        let import_density = imports.len() as f64;
        let cycles = Self::detect_cycles(&import_graph);
        let cyclic_count = cycles.len();
        let max_depth = Self::find_max_chain_depth(&import_graph);

        // Estimate external imports (paths containing node_modules, vendor, etc.)
        let external_count = imports
            .iter()
            .filter(|(_, to)| {
                to.contains("node_modules") || to.contains("vendor") || to.contains("std::")
            })
            .count();
        let external_ratio = if imports.is_empty() {
            0.0
        } else {
            external_count as f64 / imports.len() as f64
        };

        // Layer violations: imports going "backward" in module hierarchy
        let violations = imports
            .iter()
            .filter(|(from, to)| Self::is_layer_violation(from, to))
            .count();

        Self::calculate(
            import_density,
            cyclic_count,
            max_depth,
            violations,
            external_ratio,
            cycles,
            import_graph,
        )
    }
}

impl DependencyCouplingMetrics {
    /// Detect cycles using DFS
    fn detect_cycles(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut current_path = Vec::new();

        for node in graph.keys() {
            if !visited.contains(node) {
                Self::dfs_cycle_detection(
                    node,
                    graph,
                    &mut visited,
                    &mut rec_stack,
                    &mut current_path,
                    &mut cycles,
                );
            }
        }

        cycles
    }

    fn dfs_cycle_detection(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        current_path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        current_path.push(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    Self::dfs_cycle_detection(
                        neighbor,
                        graph,
                        visited,
                        rec_stack,
                        current_path,
                        cycles,
                    );
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle
                    if let Some(pos) = current_path.iter().position(|x| x == neighbor) {
                        let cycle = current_path[pos..].to_vec();
                        cycles.push(cycle);
                    }
                }
            }
        }

        current_path.pop();
        rec_stack.remove(node);
    }

    /// Find the deepest import chain
    fn find_max_chain_depth(graph: &HashMap<String, Vec<String>>) -> usize {
        let mut max_depth = 0;

        for start_node in graph.keys() {
            let depth = Self::dfs_max_depth(start_node, graph, &mut HashSet::new());
            max_depth = max_depth.max(depth);
        }

        max_depth
    }

    fn dfs_max_depth(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
    ) -> usize {
        if visited.contains(node) {
            return 0; // Avoid infinite loops
        }

        visited.insert(node.to_string());

        if let Some(neighbors) = graph.get(node) {
            let max_child_depth = neighbors
                .iter()
                .map(|n| Self::dfs_max_depth(n, graph, visited))
                .max()
                .unwrap_or(0);
            visited.remove(node);
            max_child_depth + 1
        } else {
            visited.remove(node);
            0
        }
    }

    /// Check if import goes "backward" in layer (e.g., utils importing from views)
    fn is_layer_violation(from: &str, to: &str) -> bool {
        let layer_order = ["lib", "core", "domain", "services", "controllers", "views"];

        let from_layer = layer_order.iter().position(|&layer| from.contains(layer));
        let to_layer = layer_order.iter().position(|&layer| to.contains(layer));

        match (from_layer, to_layer) {
            (Some(f), Some(t)) => f < t,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_coupling() {
        let imports = vec![
            ("app".to_string(), "lib".to_string()),
            ("lib".to_string(), "utils".to_string()),
        ];

        let metrics = DependencyCouplingMetrics::from_imports(&imports);
        assert!(metrics.coupling_score > 0.0);
        assert_eq!(metrics.cyclic_dependencies, 0);
    }

    #[test]
    fn test_cyclic_dependency() {
        let imports = vec![
            ("a".to_string(), "b".to_string()),
            ("b".to_string(), "c".to_string()),
            ("c".to_string(), "a".to_string()),
        ];

        let metrics = DependencyCouplingMetrics::from_imports(&imports);
        assert!(metrics.cyclic_dependencies > 0);
        assert!(!metrics.cycles.is_empty());
    }

    #[test]
    fn test_layer_violation() {
        let imports = vec![
            ("views".to_string(), "lib".to_string()),
            ("controllers".to_string(), "domain".to_string()),
        ];

        let metrics = DependencyCouplingMetrics::from_imports(&imports);
        assert!(metrics.layer_violations > 0);
    }

    #[test]
    fn test_import_density() {
        let imports = vec![
            ("a".to_string(), "b".to_string()),
            ("a".to_string(), "c".to_string()),
            ("a".to_string(), "d".to_string()),
        ];

        let metrics = DependencyCouplingMetrics::from_imports(&imports);
        assert_eq!(metrics.cyclic_dependencies, 0);
    }

    #[test]
    fn test_external_imports() {
        let imports = vec![
            ("my_code".to_string(), "std::io".to_string()),
            ("my_code".to_string(), "std::collections".to_string()),
            ("my_code".to_string(), "local_module".to_string()),
        ];

        let metrics = DependencyCouplingMetrics::from_imports(&imports);
        assert!(metrics.external_import_ratio > 0.5);
    }
}
