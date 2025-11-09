use std::{collections::HashMap, fmt};

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::{
    analysis_context::node_text_equals_any, checker::Checker, macros::implement_metric_trait, *,
};

// LIMITATION: Recursive function detection
//
// Cognitive Complexity should ideally increment for recursive functions according
// to the original specification. However, detecting recursion through static
// analysis alone is challenging for several reasons:
//
// 1. Direct recursion (function calls itself) could be detected by name matching,
//    but this requires tracking function scope and name resolution.
//
// 2. Indirect recursion (A calls B, B calls A) requires full call graph analysis,
//    which is difficult without type information and cross-file analysis.
//
// 3. For languages like C++, virtual function calls, function pointers, and
//    template instantiation make the call graph impossible to resolve statically.
//
// Potential solutions:
// - Implement a lightweight static analyzer that builds call graphs within
//   translation units (files) to detect direct and simple indirect recursion.
// - For complex cases, document this as a known limitation and recommend
//   dynamic analysis tools for complete recursion detection.
//
// Current status: Recursion does NOT contribute to cognitive complexity scores.

/// The `Cognitive Complexity` metric.
#[derive(Debug, Clone)]
pub struct Stats {
    structural: usize,
    structural_sum: usize,
    structural_min: usize,
    structural_max: usize,
    nesting: usize,
    total_space_functions: usize,
    boolean_seq: BoolSequence,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            structural: 0,
            structural_sum: 0,
            structural_min: usize::MAX,
            structural_max: 0,
            nesting: 0,
            total_space_functions: 1,
            boolean_seq: BoolSequence::default(),
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("cognitive", 4)?;
        st.serialize_field("sum", &self.cognitive_sum())?;
        // For files with no functions, average should be null
        if self.total_space_functions <= 1 && self.structural_sum == 0 {
            st.serialize_field("average", &None::<f64>)?;
        } else {
            st.serialize_field("average", &self.cognitive_average())?;
        }
        // For files with no functions, min should be 0, not usize::MAX
        let min_val = if self.structural_min == usize::MAX {
            0.0
        } else {
            self.cognitive_min()
        };
        st.serialize_field("min", &min_val)?;
        st.serialize_field("max", &self.cognitive_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {}, min:{}, max: {}",
            self.cognitive(),
            self.cognitive_average(),
            self.cognitive_min(),
            self.cognitive_max()
        )
    }
}

impl Stats {
    /// Merges a second `Cognitive Complexity` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.structural_min = self.structural_min.min(other.structural_min);
        self.structural_max = self.structural_max.max(other.structural_max);
        self.structural_sum += other.structural_sum;
    }

    /// Returns the `Cognitive Complexity` metric value
    pub fn cognitive(&self) -> f64 {
        self.structural as f64
    }
    /// Returns the `Cognitive Complexity` sum metric value
    pub fn cognitive_sum(&self) -> f64 {
        self.structural_sum as f64
    }

    /// Returns the `Cognitive Complexity` minimum metric value
    pub fn cognitive_min(&self) -> f64 {
        self.structural_min as f64
    }
    /// Returns the `Cognitive Complexity` maximum metric value
    pub fn cognitive_max(&self) -> f64 {
        self.structural_max as f64
    }

    /// Returns the `Cognitive Complexity` metric average value
    ///
    /// This value is computed dividing the `Cognitive Complexity` value
    /// for the total number of functions/closures in a space.
    ///
    /// If there are no functions in a code, its value is `NAN`.
    pub fn cognitive_average(&self) -> f64 {
        self.cognitive_sum() / self.total_space_functions as f64
    }
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.structural_sum += self.structural;
    }
    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        self.structural_min = self.structural_min.min(self.structural);
        self.structural_max = self.structural_max.max(self.structural);
        self.compute_sum();
    }

    pub(crate) fn finalize(&mut self, total_space_functions: usize) {
        self.total_space_functions = total_space_functions;
    }
}

pub trait Cognitive
where
    Self: Checker,
{
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    );
}

fn compute_booleans<T: std::cmp::PartialEq + std::convert::From<u16>>(
    node: &Node,
    stats: &mut Stats,
    typs1: T,
    typs2: T,
) {
    for child in node.children() {
        if typs1 == child.kind_id().into() || typs2 == child.kind_id().into() {
            stats.structural = stats
                .boolean_seq
                .eval_based_on_prev(child.kind_id(), stats.structural)
        }
    }
}

#[derive(Debug, Default, Clone)]
struct BoolSequence {
    boolean_op: Option<u16>,
}

impl BoolSequence {
    fn reset(&mut self) {
        self.boolean_op = None;
    }

    fn not_operator(&mut self, not_id: u16) {
        self.boolean_op = Some(not_id);
    }

    fn eval_based_on_prev(&mut self, bool_id: u16, structural: usize) -> usize {
        if let Some(prev) = self.boolean_op {
            if prev != bool_id {
                // The boolean operator is different from the previous one, so
                // the counter is incremented.
                structural + 1
            } else {
                // The boolean operator is equal to the previous one, so
                // the counter is not incremented.
                structural
            }
        } else {
            // Save the first boolean operator in a sequence of
            // logical operators and increment the counter.
            self.boolean_op = Some(bool_id);
            structural + 1
        }
    }
}

#[inline(always)]
fn increment(stats: &mut Stats) {
    stats.structural += stats.nesting + 1;
}

#[inline(always)]
fn increment_by_one(stats: &mut Stats) {
    stats.structural += 1;
}

fn get_nesting_from_map(
    node: &Node,
    nesting_map: &HashMap<usize, (usize, usize, usize)>,
) -> (usize, usize, usize) {
    if let Some(parent) = node.parent() {
        if let Some(n) = nesting_map.get(&parent.id()) {
            *n
        } else {
            (0, 0, 0)
        }
    } else {
        (0, 0, 0)
    }
}

fn increment_function_depth<T: std::cmp::PartialEq + std::convert::From<u16>>(
    depth: &mut usize,
    node: &Node,
    stop: T,
) {
    // Increase depth function nesting if needed
    let mut child = *node;
    while let Some(parent) = child.parent() {
        if stop == parent.kind_id().into() {
            *depth += 1;
            break;
        }
        child = parent;
    }
}

#[inline(always)]
fn increase_nesting(stats: &mut Stats, nesting: &mut usize, depth: usize, lambda: usize) {
    stats.nesting = *nesting + depth + lambda;
    increment(stats);
    *nesting += 1;
    // Reset boolean sequence after processing each control structure
    // to prevent boolean operator context from carrying over to next statement
    stats.boolean_seq.reset();
}

fn elixir_call_matches(node: &Node, keywords: &[&str]) -> bool {
    if node.kind_id() != Elixir::Call {
        return false;
    }
    node.child(0)
        .filter(|child| child.kind_id() == Elixir::Identifier)
        .map(|child| node_text_equals_any(&child, keywords))
        .unwrap_or(false)
}

impl Cognitive for PythonCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        use Python::*;

        // Get nesting of the parent
        let (mut nesting, mut depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind_id().into() {
            IfStatement | ForStatement | WhileStatement | ConditionalExpression => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            ElifClause => {
                // No nesting increment for them because their cost has already
                // been paid by the if construct
                increment_by_one(stats);
                // Reset the boolean sequence
                stats.boolean_seq.reset();
            }
            ElseClause | FinallyClause => {
                // No nesting increment for them because their cost has already
                // been paid by the if construct
                increment_by_one(stats);
            }
            ExceptClause => {
                nesting += 1;
                increment(stats);
            }
            ExpressionList | ExpressionStatement | Tuple => {
                stats.boolean_seq.reset();
            }
            NotOperator => {
                stats.boolean_seq.not_operator(node.kind_id());
            }
            BooleanOperator => {
                if node.count_specific_ancestors::<PythonParser>(
                    |node| node.kind_id() == BooleanOperator,
                    |node| node.kind_id() == Lambda,
                ) == 0
                {
                    stats.structural += node.count_specific_ancestors::<PythonParser>(
                        |node| node.kind_id() == Lambda,
                        |node| {
                            matches!(
                                node.kind_id().into(),
                                ExpressionList | IfStatement | ForStatement | WhileStatement
                            )
                        },
                    );
                }
                compute_booleans::<language_python::Python>(node, stats, And, Or);
            }
            Lambda => {
                // Increase lambda nesting
                lambda += 1;
            }
            FunctionDefinition => {
                // Increase depth function nesting if needed
                increment_function_depth::<language_python::Python>(
                    &mut depth,
                    node,
                    FunctionDefinition,
                );
            }
            _ => {}
        }
        // Add node to nesting map
        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

impl Cognitive for RustCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        use Rust::*;
        // LIMITATION: Macro expansion is not analyzed
        // Rust macros can expand to arbitrary code including control flow structures.
        // To properly account for complexity in macros, we would need to:
        // 1. Expand macros during parsing (requires full Rust compiler integration)
        // 2. Analyze the expanded code rather than the macro invocation
        // Current behavior: Macro invocations are ignored, only explicit code is analyzed.
        let (mut nesting, mut depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind_id().into() {
            IfExpression => {
                // Check if a node is not an else-if
                if !Self::is_else_if(node) {
                    increase_nesting(stats,&mut nesting, depth, lambda);
                }
            }
            ForExpression | WhileExpression | MatchExpression => {
                increase_nesting(stats,&mut nesting, depth, lambda);
            }
            Else /*else-if also */ => {
                increment_by_one(stats);
            }
            BreakExpression | ContinueExpression => {
                if let Some(label_child) = node.child(1) {
                    if let Label = label_child.kind_id().into() {
                        increment_by_one(stats);
                    }
                }
            }
            UnaryExpression => {
                stats.boolean_seq.not_operator(node.kind_id());
            }
            BinaryExpression => {
                compute_booleans::<language_rust::Rust>(node, stats, AMPAMP, PIPEPIPE);
            }
            FunctionItem  => {
                nesting = 0;
                // Increase depth function nesting if needed
                increment_function_depth::<language_rust::Rust>(&mut depth, node, FunctionItem);
            }
            ClosureExpression => {
                lambda += 1;
            }
            _ => {}
        }
        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

impl Cognitive for CppCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        use Cpp::*;

        // LIMITATION: Preprocessor macro expansion is not analyzed
        // C/C++ macros can expand to arbitrary code including control flow structures.
        // To properly account for complexity in macros, we would need to:
        // 1. Run the preprocessor and expand all macros
        // 2. Parse the expanded code rather than the source
        // Current behavior: Macro invocations are ignored, only explicit code is analyzed.
        let (mut nesting, depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind_id().into() {
            IfStatement => {
                if !Self::is_else_if(node) {
                    increase_nesting(stats,&mut nesting, depth, lambda);
                }
            }
            ForStatement | WhileStatement | DoStatement | SwitchStatement | CatchClause => {
                increase_nesting(stats,&mut nesting, depth, lambda);
            }
            GotoStatement | Else /* else-if also */ => {
                increment_by_one(stats);
            }
            UnaryExpression2 => {
                stats.boolean_seq.not_operator(node.kind_id());
            }
            BinaryExpression2 => {
                compute_booleans::<language_cpp::Cpp>(node, stats, AMPAMP, PIPEPIPE);
            }
            LambdaExpression => {
                lambda += 1;
            }
            _ => {}
        }
        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

macro_rules! js_cognitive {
    ($lang:ident) => {
        fn compute(node: &Node, stats: &mut Stats, nesting_map: &mut HashMap<usize, (usize, usize, usize)>) {
            use $lang::*;
            let (mut nesting, mut depth, mut lambda) = get_nesting_from_map(node, nesting_map);

            match node.kind_id().into() {
                IfStatement => {
                    if !Self::is_else_if(&node) {
                        increase_nesting(stats,&mut nesting, depth, lambda);
                    }
                }
                ForStatement | ForInStatement | WhileStatement | DoStatement | SwitchStatement | CatchClause | TernaryExpression => {
                    increase_nesting(stats,&mut nesting, depth, lambda);
                }
                Else /* else-if also */ => {
                    increment_by_one(stats);
                }
                ExpressionStatement => {
                    // Reset the boolean sequence
                    stats.boolean_seq.reset();
                }
                UnaryExpression => {
                    stats.boolean_seq.not_operator(node.kind_id());
                }
                BinaryExpression => {
                    compute_booleans::<$lang>(node, stats, AMPAMP, PIPEPIPE);
                }
                FunctionDeclaration => {
                    // Reset lambda nesting at function for JS
                    nesting = 0;
                    lambda = 0;
                    // Increase depth function nesting if needed
                    increment_function_depth::<$lang>(&mut depth, node, FunctionDeclaration);
                }
                ArrowFunction => {
                    lambda += 1;
                }
                _ => {}
            }
            nesting_map.insert(node.id(), (nesting, depth, lambda));
        }
    };
}

impl Cognitive for MozjsCode {
    js_cognitive!(Mozjs);
}

impl Cognitive for JavascriptCode {
    js_cognitive!(Javascript);
}

impl Cognitive for TypescriptCode {
    js_cognitive!(Typescript);
}

impl Cognitive for TsxCode {
    js_cognitive!(Tsx);
}

impl Cognitive for JavaCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        use Java::*;

        let (mut nesting, depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind_id().into() {
            IfStatement => {
                if !Self::is_else_if(node) {
                    increase_nesting(stats,&mut nesting, depth, lambda);
                }
            }
            ForStatement | WhileStatement | DoStatement | SwitchBlock | CatchClause => {
                increase_nesting(stats,&mut nesting, depth, lambda);
            }
            Else /* else-if also */ => {
                increment_by_one(stats);
            }
            UnaryExpression => {
                stats.boolean_seq.not_operator(node.kind_id());
            }
            BinaryExpression => {
                compute_booleans::<language_java::Java>(node, stats, AMPAMP, PIPEPIPE);
            }
            LambdaExpression => {
                lambda += 1;
            }
            _ => {}
        }
        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

// BEAM language implementations
impl Cognitive for ElixirCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        use Elixir::*;

        let (mut nesting, depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind_id().into() {
            Call => {
                if elixir_call_matches(
                    node,
                    &[
                        "if", "unless", "cond", "case", "with", "receive", "try", "for",
                    ],
                ) {
                    increase_nesting(stats, &mut nesting, depth, lambda);
                } else {
                    stats.boolean_seq.reset();
                }
            }
            StabClause => {
                increment(stats);
                stats.boolean_seq.reset();
            }
            ElseBlock => {
                increment_by_one(stats);
                stats.boolean_seq.reset();
            }
            AnonymousFunction => {
                lambda += 1;
                stats.boolean_seq.reset();
            }
            _ => {}
        }

        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

impl Cognitive for ErlangCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        use Erlang::*;

        let (mut nesting, depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind_id().into() {
            IfExpr | CaseExpr | ReceiveExpr | TryExpr | TryAfter => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            CrClause | GuardClause => {
                increment(stats);
                stats.boolean_seq.reset();
            }
            FunctionClause => {
                if let Some(prev) = node.previous_named_sibling() {
                    if Into::<Erlang>::into(prev.kind_id()) == Erlang::FunctionClause {
                        increment(stats);
                    }
                }
            }
            AnonymousFun => {
                lambda += 1;
            }
            BinaryOpExpr => {
                stats.boolean_seq.reset();
            }
            _ => {}
        }

        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

impl Cognitive for GleamCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        use Gleam::*;

        let (mut nesting, depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind_id().into() {
            Case => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            CaseClause => {
                if let Some(prev) = node.previous_named_sibling() {
                    if Into::<Gleam>::into(prev.kind_id()) == Gleam::CaseClause {
                        increment(stats);
                    } else {
                        increment_by_one(stats);
                    }
                } else {
                    increment_by_one(stats);
                }
            }
            Function => {
                stats.boolean_seq.reset();
            }
            AnonymousFunction => {
                lambda += 1;
            }
            BinaryExpression => {
                stats.boolean_seq.reset();
            }
            _ => {}
        }

        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

impl Cognitive for KotlinCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        let (mut nesting, depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind() {
            "if_expression" => {
                // Check if this is part of an else-if chain
                if let Some(parent) = node.parent() {
                    if parent.kind() == "if_expression" {
                        // This is an else-if, only increment by one
                        increment_by_one(stats);
                    } else {
                        increase_nesting(stats, &mut nesting, depth, lambda);
                    }
                } else {
                    increase_nesting(stats, &mut nesting, depth, lambda);
                }
            }
            "when_expression" | "for_statement" | "while_statement" | "do_while_statement"
            | "try_expression" => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            "catch_block" => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            "binary_expression" => {
                // Handle && and || operators
                if let Some(operator) = node.child_by_field_name("operator") {
                    match operator.kind() {
                        "&&" | "||" => {
                            stats.boolean_seq.reset();
                            // In Kotlin, just increment by 1 for boolean operators
                            increment_by_one(stats);
                        }
                        _ => {}
                    }
                }
            }
            "lambda_literal" | "anonymous_function" => {
                lambda += 1;
            }
            "function_declaration" => {
                nesting = 0;
            }
            _ => {}
        }
        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

impl Cognitive for LuaCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        let (mut nesting, depth, lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind() {
            "if_statement" | "while_statement" | "repeat_statement" | "for_statement" => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            "elseif_statement" => {
                increment_by_one(stats);
            }
            "else_statement" => {
                increment_by_one(stats);
            }
            "binary_expression" => {
                // Lua uses 'and'/'or' for boolean operators
                if let Some(operator) = node.child_by_field_name("operator") {
                    match operator.kind() {
                        "and" | "or" => {
                            stats.boolean_seq.reset();
                            increment_by_one(stats);
                        }
                        _ => {}
                    }
                }
            }
            "function_declaration" | "function_definition" => {
                nesting = 0;
            }
            _ => {}
        }
        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

impl Cognitive for GoCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        let (mut nesting, depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind() {
            "if_statement" | "for_statement" | "switch_statement" | "select_statement"
            | "type_switch_statement" => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            "func_literal" => {
                lambda += 1;
            }
            "binary_expression" => {
                // Handle && and || operators
                if let Some(operator) = node.child_by_field_name("operator") {
                    match operator.kind() {
                        "&&" | "||" => {
                            stats.boolean_seq.reset();
                            increment_by_one(stats);
                        }
                        _ => {}
                    }
                }
            }
            "function_declaration" | "method_declaration" => {
                nesting = 0;
            }
            _ => {}
        }
        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

impl Cognitive for CsharpCode {
    fn compute(
        node: &Node,
        stats: &mut Stats,
        nesting_map: &mut HashMap<usize, (usize, usize, usize)>,
    ) {
        let (mut nesting, depth, mut lambda) = get_nesting_from_map(node, nesting_map);

        match node.kind() {
            "if_statement" => {
                // Check if this is an else-if
                if let Some(parent) = node.parent() {
                    if parent.kind() == "else_clause" {
                        increment_by_one(stats);
                    } else {
                        increase_nesting(stats, &mut nesting, depth, lambda);
                    }
                } else {
                    increase_nesting(stats, &mut nesting, depth, lambda);
                }
            }
            "switch_statement" | "for_statement" | "foreach_statement" | "while_statement"
            | "do_statement" | "try_statement" => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            "catch_clause" => {
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            "else_clause" => {
                increment_by_one(stats);
            }
            "conditional_expression" => {
                // Ternary operator in C#
                increase_nesting(stats, &mut nesting, depth, lambda);
            }
            "binary_expression" => {
                // Handle && and || operators
                if let Some(operator) = node.child_by_field_name("operator") {
                    match operator.kind() {
                        "&&" | "||" => {
                            stats.boolean_seq.reset();
                            increment_by_one(stats);
                        }
                        _ => {}
                    }
                }
            }
            "lambda_expression" | "anonymous_method_expression" | "anonymous_function" => {
                lambda += 1;
            }
            "method_declaration" | "constructor_declaration" | "local_function_statement" => {
                nesting = 0;
            }
            _ => {}
        }
        nesting_map.insert(node.id(), (nesting, depth, lambda));
    }
}

// PreprocCode and CcommentCode are for preprocessor directives and comments
// They don't have control flow, so empty implementations are appropriate
implement_metric_trait!(Cognitive, PreprocCode, CcommentCode);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::check_metrics;

    #[test]
    fn python_no_cognitive() {
        check_metrics::<PythonParser>("a = 42", "foo.py", |metric| {
            insta::assert_json_snapshot!(
                metric.cognitive,
                @r###"
                    {
                      "sum": 0.0,
                      "average": null,
                      "min": 0.0,
                      "max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn rust_no_cognitive() {
        check_metrics::<ParserEngineRust>("let a = 42;", "foo.rs", |metric| {
            insta::assert_json_snapshot!(
                metric.cognitive,
                @r###"
                    {
                      "sum": 0.0,
                      "average": null,
                      "min": 0.0,
                      "max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn c_no_cognitive() {
        check_metrics::<CppParser>("int a = 42;", "foo.c", |metric| {
            insta::assert_json_snapshot!(
                metric.cognitive,
                @r###"
                    {
                      "sum": 0.0,
                      "average": null,
                      "min": 0.0,
                      "max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn mozjs_no_cognitive() {
        check_metrics::<MozjsParser>("var a = 42;", "foo.js", |metric| {
            insta::assert_json_snapshot!(
                metric.cognitive,
                @r###"
                    {
                      "sum": 0.0,
                      "average": null,
                      "min": 0.0,
                      "max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn python_simple_function() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                if a and b:  # +2 (+1 and)
                   return 1
                if c and d: # +2 (+1 and)
                   return 1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 4.0,
                  "average": 4.0,
                  "min": 0.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_expression_statement() {
        // Boolean expressions containing `And` and `Or` operators were not
        // considered in assignments
        check_metrics::<PythonParser>(
            "def f(a, b):
                c = True and True",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 1.0,
                  "average": 1.0,
                  "min": 0.0,
                  "max": 1.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_tuple() {
        // Boolean expressions containing `And` and `Or` operators were not
        // considered inside tuples
        check_metrics::<PythonParser>(
            "def f(a, b):
                return \"%s%s\" % (a and \"Get\" or \"Set\", b)",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 2.0,
                  "average": 2.0,
                  "min": 0.0,
                  "max": 2.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_elif_function() {
        // Boolean expressions containing `And` and `Or` operators were not
        // considered in `elif` statements
        check_metrics::<PythonParser>(
            "def f(a, b):
                if a and b:  # +2 (+1 and)
                   return 1
                elif c and d: # +2 (+1 and)
                   return 1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 4.0,
                  "average": 4.0,
                  "min": 0.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_more_elifs_function() {
        // Boolean expressions containing `And` and `Or` operators were not
        // considered when there were more `elif` statements
        check_metrics::<PythonParser>(
            "def f(a, b):
                if a and b:  # +2 (+1 and)
                   return 1
                elif c and d: # +2 (+1 and)
                   return 1
                elif e and f: # +2 (+1 and)
                   return 1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 6.0,
                  "average": 6.0,
                  "min": 0.0,
                  "max": 6.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn rust_simple_function() {
        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if a && b { // +2 (+1 &&)
                     println!(\"test\");
                 }
                 if c && d { // +2 (+1 &&)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 4.0,
                      "average": 4.0,
                      "min": 0.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_simple_function() {
        check_metrics::<CppParser>(
            "void f() {
                 if (a && b) { // +2 (+1 &&)
                     printf(\"test\");
                 }
                 if (c && d) { // +2 (+1 &&)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 0.0,
                  "average": null,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn mozjs_simple_function() {
        check_metrics::<MozjsParser>(
            "function f() {
                 if (a && b) { // +2 (+1 &&)
                     window.print(\"test\");
                 }
                 if (c && d) { // +2 (+1 &&)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 4.0,
                  "average": 4.0,
                  "min": 0.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_sequence_same_booleans() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                if a and b and True:  # +2 (+1 sequence of and)
                   return 1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 2.0,
                  "average": 2.0,
                  "min": 0.0,
                  "max": 2.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn rust_sequence_same_booleans() {
        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if a && b && true { // +2 (+1 sequence of &&)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 2.0,
                      "average": 2.0,
                      "min": 0.0,
                      "max": 2.0
                    }"###
                );
            },
        );

        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if a || b || c || d { // +2 (+1 sequence of ||)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 2.0,
                      "average": 2.0,
                      "min": 0.0,
                      "max": 2.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_sequence_same_booleans() {
        check_metrics::<CppParser>(
            "void f() {
                 if (a && b && 1 == 1) { // +2 (+1 sequence of &&)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 0.0,
                  "average": null,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );

        check_metrics::<CppParser>(
            "void f() {
                 if (a || b || c || d) { // +2 (+1 sequence of ||)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 0.0,
                  "average": null,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn mozjs_sequence_same_booleans() {
        check_metrics::<MozjsParser>(
            "function f() {
                 if (a && b && 1 == 1) { // +2 (+1 sequence of &&)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 2.0,
                  "average": 2.0,
                  "min": 0.0,
                  "max": 2.0
                }
                "#
                );
            },
        );

        check_metrics::<MozjsParser>(
            "function f() {
                 if (a || b || c || d) { // +2 (+1 sequence of ||)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 2.0,
                      "average": 2.0,
                      "min": 0.0,
                      "max": 2.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_not_booleans() {
        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if !a && !b { // +2 (+1 &&)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 2.0,
                      "average": 2.0,
                      "min": 0.0,
                      "max": 2.0
                    }"###
                );
            },
        );

        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if a && !(b && c) { // +3 (+1 &&, +1 &&)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 3.0,
                      "average": 3.0,
                      "min": 0.0,
                      "max": 3.0
                    }"###
                );
            },
        );

        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if !(a || b) && !(c || d) { // +4 (+1 ||, +1 &&, +1 ||)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 4.0,
                      "average": 4.0,
                      "min": 0.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_not_booleans() {
        check_metrics::<CppParser>(
            "void f() {
                 if (a && !(b && c)) { // +3 (+1 &&, +1 &&)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 0.0,
                  "average": null,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );

        check_metrics::<CppParser>(
            "void f() {
                 if (!(a || b) && !(c || d)) { // +4 (+1 ||, +1 &&, +1 ||)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 0.0,
                  "average": null,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn mozjs_not_booleans() {
        check_metrics::<MozjsParser>(
            "function f() {
                 if (a && !(b && c)) { // +3 (+1 &&, +1 &&)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 3.0,
                  "average": 3.0,
                  "min": 0.0,
                  "max": 3.0
                }
                "#
                );
            },
        );

        check_metrics::<MozjsParser>(
            "function f() {
                 if (!(a || b) && !(c || d)) { // +4 (+1 ||, +1 &&, +1 ||)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 4.0,
                      "average": 4.0,
                      "min": 0.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_sequence_different_booleans() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                if a and b or True:  # +3 (+1 and, +1 or)
                   return 1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 3.0,
                  "average": 3.0,
                  "min": 0.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn rust_sequence_different_booleans() {
        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if a && b || true { // +3 (+1 &&, +1 ||)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 3.0,
                      "average": 3.0,
                      "min": 0.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_sequence_different_booleans() {
        check_metrics::<CppParser>(
            "void f() {
                 if (a && b || 1 == 1) { // +3 (+1 &&, +1 ||)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 0.0,
                  "average": null,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn mozjs_sequence_different_booleans() {
        check_metrics::<MozjsParser>(
            "function f() {
                 if (a && b || 1 == 1) { // +3 (+1 &&, +1 ||)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 3.0,
                  "average": 3.0,
                  "min": 0.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_formatted_sequence_different_booleans() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                if (  # +1
                    a and b and  # +1
                    (c or d)  # +1
                ):
                   return 1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 3.0,
                  "average": 3.0,
                  "min": 0.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_1_level_nesting() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                if a:  # +1
                    for i in range(b):  # +2
                        return 1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 3.0,
                      "average": 3.0,
                      "min": 0.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_1_level_nesting() {
        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if true { // +1
                     if true { // +2 (nesting = 1)
                         println!(\"test\");
                     } else if 1 == 1 { // +1
                         if true { // +3 (nesting = 2)
                             println!(\"test\");
                         }
                     } else { // +1
                         if true { // +3 (nesting = 2)
                             println!(\"test\");
                         }
                     }
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 11.0,
                      "average": 11.0,
                      "min": 0.0,
                      "max": 11.0
                    }"###
                );
            },
        );

        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if true { // +1
                     match true { // +2 (nesting = 1)
                         true => println!(\"test\"),
                         false => println!(\"test\"),
                     }
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 3.0,
                      "average": 3.0,
                      "min": 0.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_1_level_nesting() {
        check_metrics::<CppParser>(
            "void f() {
                 if (1 == 1) { // +1
                     if (1 == 1) { // +2 (nesting = 1)
                         printf(\"test\");
                     } else if (1 == 1) { // +1
                         if (1 == 1) { // +3 (nesting = 2)
                             printf(\"test\");
                         }
                     } else { // +1
                         if (1 == 1) { // +3 (nesting = 2)
                             printf(\"test\");
                         }
                     }
                 }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 2.0,
                  "average": 2.0,
                  "min": 0.0,
                  "max": 2.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn mozjs_1_level_nesting() {
        check_metrics::<MozjsParser>(
            "function f() {
                 if (1 == 1) { // +1
                     if (1 == 1) { // +2 (nesting = 1)
                         window.print(\"test\");
                     } else if (1 == 1) { // +1
                         if (1 == 1) { // +3 (nesting = 2)
                             window.print(\"test\");
                         }
                     } else { // +1
                         if (1 == 1) { // +3 (nesting = 2)
                             window.print(\"test\");
                         }
                     }
                 }
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 16.0,
                  "average": 16.0,
                  "min": 0.0,
                  "max": 16.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_2_level_nesting() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                if a:  # +1
                    for i in range(b):  # +2
                        if b:  # +3
                            return 1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 6.0,
                      "average": 6.0,
                      "min": 0.0,
                      "max": 6.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_2_level_nesting() {
        check_metrics::<ParserEngineRust>(
            "fn f() {
                 if true { // +1
                     for i in 0..4 { // +2 (nesting = 1)
                         match true { // +3 (nesting = 2)
                             true => println!(\"test\"),
                             false => println!(\"test\"),
                         }
                     }
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 6.0,
                      "average": 6.0,
                      "min": 0.0,
                      "max": 6.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_try_construct() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                try:
                    for foo in bar:  # +1
                        return a
                except Exception:  # +1
                    if a < 0:  # +2
                        return a",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 4.0,
                      "average": 4.0,
                      "min": 0.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn mozjs_try_construct() {
        check_metrics::<MozjsParser>(
            "function asyncOnChannelRedirect(oldChannel, newChannel, flags, callback) {
                 for (const collector of this.collectors) {
                     try {
                         collector._onChannelRedirect(oldChannel, newChannel, flags);
                     } catch (ex) {
                         console.error(
                             \"StackTraceCollector.onChannelRedirect threw an exception\",
                              ex
                         );
                     }
                 }
                 callback.onRedirectVerifyCallback(Cr.NS_OK);
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 3.0,
                  "average": 3.0,
                  "min": 0.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn rust_break_continue() {
        // Only labeled break and continue statements are considered
        check_metrics::<ParserEngineRust>(
            "fn f() {
                 'tens: for ten in 0..3 { // +1
                     '_units: for unit in 0..=9 { // +2 (nesting = 1)
                         if unit % 2 == 0 { // +3 (nesting = 2)
                             continue;
                         } else if unit == 5 { // +1
                             continue 'tens; // +1
                         } else if unit == 6 { // +1
                             break;
                         } else { // +1
                             break 'tens; // +1
                         }
                     }
                 }
             }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 11.0,
                      "average": 11.0,
                      "min": 0.0,
                      "max": 11.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_goto() {
        check_metrics::<CppParser>(
            "void f() {
             OUT: for (int i = 1; i <= max; ++i) { // +1
                      for (int j = 2; j < i; ++j) { // +2 (nesting = 1)
                          if (i % j == 0) { // +3 (nesting = 2)
                              goto OUT; // +1
                          }
                      }
                  }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 0.0,
                  "average": null,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn c_switch() {
        check_metrics::<CppParser>(
            "void f() {
                 switch (1) { // +1
                     case 1:
                         printf(\"one\");
                         break;
                     case 2:
                         printf(\"two\");
                         break;
                     case 3:
                         printf(\"three\");
                         break;
                     default:
                         printf(\"all\");
                         break;
                 }
             }",
            "foo.c",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 0.0,
                  "average": null,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn mozjs_switch() {
        check_metrics::<MozjsParser>(
            "function f() {
                 switch (1) { // +1
                     case 1:
                         window.print(\"one\");
                         break;
                     case 2:
                         window.print(\"two\");
                         break;
                     case 3:
                         window.print(\"three\");
                         break;
                     default:
                         window.print(\"all\");
                         break;
                 }
             }",
            "foo.js",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 1.0,
                  "average": 1.0,
                  "min": 0.0,
                  "max": 1.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_ternary_operator() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                 if a % 2:  # +1
                     return 'c' if a else 'd'  # +2
                 return 'a' if a else 'b'  # +1",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 4.0,
                  "average": 4.0,
                  "min": 0.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_nested_functions_lambdas() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                 def foo(a):
                     if a:  # +2 (+1 nesting)
                         return 1
                 # +3 (+1 for boolean sequence +2 for lambda nesting)
                 bar = lambda a: lambda b: b or True or True
                 return bar(foo(a))(a)",
            "foo.py",
            |metric| {
                // 2 functions + 2 lambdas = 4
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 5.0,
                  "average": 0.8333333333333334,
                  "min": 0.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn python_real_function() {
        check_metrics::<PythonParser>(
            "def process_raw_constant(constant, min_word_length):
                 processed_words = []
                 raw_camelcase_words = []
                 for raw_word in re.findall(r'[a-z]+', constant):  # +1
                     word = raw_word.strip()
                         if (  # +2 (+1 if and +1 nesting)
                             len(word) >= min_word_length
                             and not (word.startswith('-') or word.endswith('-')) # +2 operators
                         ):
                             if is_camel_case_word(word):  # +3 (+1 if and +2 nesting)
                                 raw_camelcase_words.append(word)
                             else: # +1 else
                                 processed_words.append(word.lower())
                 return processed_words, raw_camelcase_words",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r#"
                {
                  "sum": 9.0,
                  "average": 9.0,
                  "min": 0.0,
                  "max": 9.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn rust_if_let_else_if_else() {
        check_metrics::<ParserEngineRust>(
            "pub fn create_usage_no_title(p: &Parser, used: &[&str]) -> String {
                 debugln!(\"usage::create_usage_no_title;\");
                 if let Some(u) = p.meta.usage_str { // +1
                     String::from(&*u)
                 } else if used.is_empty() { // +1
                     create_help_usage(p, true)
                 } else { // +1
                     create_smart_usage(p, used)
                }
            }",
            "foo.rs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 3.0,
                      "average": 3.0,
                      "min": 0.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn typescript_if_else_if_else() {
        check_metrics::<TypescriptParser>(
            "function foo() {
                 if (this._closed) return Promise.resolve(); // +1
                 if (this._tempDirectory) { // +1
                     this.kill();
                 } else if (this.connection) { // +1
                     this.kill();
                 } else { // +1
                     throw new Error(`Error`);
                }
                helper.removeEventListeners(this._listeners);
                return this._processClosing;
            }",
            "foo.ts",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 4.0,
                      "average": 4.0,
                      "min": 0.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_no_cognitive() {
        check_metrics::<JavaParser>("int a = 42;", "foo.java", |metric| {
            insta::assert_json_snapshot!(
                metric.cognitive,
                @r###"
            {
              "sum": 0.0,
              "average": null,
              "min": 0.0,
              "max": 0.0
            }
            "###
            );
        });
    }

    #[test]
    fn java_single_branch_function() {
        check_metrics::<JavaParser>(
            "class X {
                public static void print(boolean a){
                if(a){ // +1
                  System.out.println(\"test1\");
                }
              }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                {
                  "sum": 1.0,
                  "average": 1.0,
                  "min": 0.0,
                  "max": 1.0
                }
                "###
                );
            },
        );
    }

    #[test]
    fn java_multiple_branch_function() {
        check_metrics::<JavaParser>(
            "class X {
              public static void print(boolean a, boolean b){
                if(a){ // +1
                  System.out.println(\"test1\");
                }
                if(b){ // +1
                  System.out.println(\"test2\");
                }
                else { // +1
                  System.out.println(\"test3\");
                }
              }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                {
                  "sum": 3.0,
                  "average": 3.0,
                  "min": 0.0,
                  "max": 3.0
                }
                "###
                );
            },
        );
    }

    #[test]
    fn java_compound_conditions() {
        check_metrics::<JavaParser>(
            "class X {
              public static void print(boolean a, boolean b, boolean c, boolean d){
                if(a && b){ // +2 (+1 &&)
                  System.out.println(\"test1\");
                }
                if(c && d){ // +2 (+1 &&)
                  System.out.println(\"test2\");
                }
              }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 4.0,
                      "average": 4.0,
                      "min": 0.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_switch_statement() {
        check_metrics::<JavaParser>(
            "class X {
              public static void print(boolean a, boolean b, boolean c, boolean d){
                switch(expr){ //+1
                  case 1:
                    System.out.println(\"test1\");
                    break;
                  case 2:
                    System.out.println(\"test2\");
                    break;
                  default:
                    System.out.println(\"test\");
                }
              }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 1.0,
                      "average": 1.0,
                      "min": 0.0,
                      "max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_switch_expression() {
        check_metrics::<JavaParser>(
            "class X {
              public static void print(boolean a, boolean b, boolean c, boolean d){
                switch(expr){ // +1
                  case 1 -> System.out.println(\"test1\");
                  case 2 -> System.out.println(\"test2\");
                  default -> System.out.println(\"test\");
                }
              }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 1.0,
                      "average": 1.0,
                      "min": 0.0,
                      "max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_not_booleans() {
        check_metrics::<JavaParser>(
            "class X {
              public static void print(boolean a, boolean b, boolean c, boolean d){
                if (a && !(b && c)) { // +3 (+1 &&, +1 &&)
                  printf(\"test\");
                }
              }
            }",
            "foo.java",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.cognitive,
                    @r###"
                    {
                      "sum": 3.0,
                      "average": 3.0,
                      "min": 0.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }
}
