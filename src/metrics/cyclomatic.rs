use std::fmt;

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::{checker::Checker, macros::implement_metric_trait, *};

/// The `Cyclomatic` metric.
#[derive(Debug, Clone)]
pub struct Stats {
    cyclomatic_sum: f64,
    cyclomatic: f64,
    n: usize,
    cyclomatic_max: f64,
    cyclomatic_min: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            cyclomatic_sum: 0.,
            cyclomatic: 1.,
            n: 1,
            cyclomatic_max: 0.,
            cyclomatic_min: f64::MAX,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("cyclomatic", 4)?;
        st.serialize_field("sum", &self.cyclomatic_sum())?;
        st.serialize_field("average", &self.cyclomatic_average())?;
        st.serialize_field("min", &self.cyclomatic_min())?;
        st.serialize_field("max", &self.cyclomatic_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {}, min: {}, max: {}",
            self.cyclomatic_sum(),
            self.cyclomatic_average(),
            self.cyclomatic_min(),
            self.cyclomatic_max()
        )
    }
}

impl Stats {
    /// Merges a second `Cyclomatic` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        // Calculate minimum and maximum values
        self.cyclomatic_max = self.cyclomatic_max.max(other.cyclomatic_max);
        self.cyclomatic_min = self.cyclomatic_min.min(other.cyclomatic_min);

        self.cyclomatic_sum += other.cyclomatic_sum;
        self.n += other.n;
    }

    /// Returns the `Cyclomatic` metric value
    pub fn cyclomatic(&self) -> f64 {
        self.cyclomatic
    }
    /// Returns the sum
    pub fn cyclomatic_sum(&self) -> f64 {
        self.cyclomatic_sum
    }

    /// Returns the `Cyclomatic` metric average value
    ///
    /// This value is computed dividing the `Cyclomatic` value for the
    /// number of spaces.
    pub fn cyclomatic_average(&self) -> f64 {
        self.cyclomatic_sum() / self.n as f64
    }
    /// Returns the `Cyclomatic` maximum value
    pub fn cyclomatic_max(&self) -> f64 {
        self.cyclomatic_max
    }
    /// Returns the `Cyclomatic` minimum value
    pub fn cyclomatic_min(&self) -> f64 {
        self.cyclomatic_min
    }
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.cyclomatic_sum += self.cyclomatic;
    }
    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        self.cyclomatic_max = self.cyclomatic_max.max(self.cyclomatic);
        self.cyclomatic_min = self.cyclomatic_min.min(self.cyclomatic);
        self.compute_sum();
    }
}

pub trait Cyclomatic
where
    Self: Checker,
{
    fn compute(node: &Node, stats: &mut Stats);
}

impl Cyclomatic for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Python::*;

        match node.kind_id().into() {
            If | Elif | For | While | Except | With | Assert | And | Or => {
                stats.cyclomatic += 1.;
            }
            Else => {
                if node.has_ancestors(
                    |node| matches!(node.kind_id().into(), ForStatement | WhileStatement),
                    |node| node.kind_id() == ElseClause,
                ) {
                    stats.cyclomatic += 1.;
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Mozjs::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Javascript::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Typescript::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Tsx::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Rust::*;

        match node.kind_id().into() {
            If | For | While | Loop | MatchArm | MatchArm2 | TryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Cpp::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | ConditionalExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for ElixirCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Elixir::*;

        match node.kind_id().into() {
            Call => {
                if let Some(identifier) = node.child(0) {
                    if identifier.kind_id() == Identifier
                        && node_text_equals_any(
                            &identifier,
                            &[
                                "if", "unless", "case", "cond", "with", "receive", "try", "for",
                            ],
                        )
                    {
                        stats.cyclomatic += 1.;
                    }
                }
            }
            StabClause | ElseBlock => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for ErlangCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Erlang::*;

        match node.kind_id().into() {
            IfExpr | CaseExpr | ReceiveExpr | TryExpr | TryAfter => {
                stats.cyclomatic += 1.;
            }
            GuardClause | CrClause => {
                stats.cyclomatic += 1.;
            }
            FunctionClause => {
                if let Some(prev) = node.previous_named_sibling() {
                    if Into::<Erlang>::into(prev.kind_id()) == Erlang::FunctionClause {
                        stats.cyclomatic += 1.;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for GleamCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Gleam::*;

        match node.kind_id().into() {
            Case => {
                stats.cyclomatic += 1.;
            }
            CaseClause => {
                if let Some(prev) = node.previous_named_sibling() {
                    if Into::<Gleam>::into(prev.kind_id()) == Gleam::CaseClause {
                        stats.cyclomatic += 1.;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for JavaCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Java::*;

        match node.kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for KotlinCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind() {
            "if_expression" | "when_expression" | "for_statement" | "while_statement"
            | "do_while_statement" | "try_expression" | "catch_block" => {
                stats.cyclomatic += 1.;
            }
            "when_entry" => {
                // Each case in a when expression adds to complexity
                stats.cyclomatic += 1.;
            }
            "binary_expression" => {
                // Handle && and || operators
                if let Some(operator) = node.child_by_field_name("operator") {
                    if matches!(operator.kind(), "&&" | "||") {
                        stats.cyclomatic += 1.;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for LuaCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind() {
            "if_statement" | "while_statement" | "repeat_statement" | "for_statement" => {
                stats.cyclomatic += 1.;
            }
            "elseif_statement" => {
                // Each elseif adds to complexity
                stats.cyclomatic += 1.;
            }
            "binary_expression" => {
                // Lua uses 'and'/'or' for boolean operators
                if let Some(operator) = node.child_by_field_name("operator") {
                    if matches!(operator.kind(), "and" | "or") {
                        stats.cyclomatic += 1.;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for GoCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind() {
            "if_statement"
            | "for_statement"
            | "switch_statement"
            | "select_statement"
            | "type_switch_statement" => {
                stats.cyclomatic += 1.;
            }
            "expression_case" | "communication_case" | "default_case" => {
                // Each case in switch/select adds to complexity
                stats.cyclomatic += 1.;
            }
            "binary_expression" => {
                // Handle && and || operators
                if let Some(operator) = node.child_by_field_name("operator") {
                    if matches!(operator.kind(), "&&" | "||") {
                        stats.cyclomatic += 1.;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for CsharpCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind() {
            "if_statement"
            | "switch_statement"
            | "for_statement"
            | "foreach_statement"
            | "while_statement"
            | "do_statement"
            | "try_statement"
            | "catch_clause"
            | "conditional_expression" => {
                stats.cyclomatic += 1.;
            }
            "switch_section" | "switch_expression_arm" => {
                // Each case in switch adds to complexity
                stats.cyclomatic += 1.;
            }
            "binary_expression" => {
                // Handle && and || operators
                if let Some(operator) = node.child_by_field_name("operator") {
                    if matches!(operator.kind(), "&&" | "||") {
                        stats.cyclomatic += 1.;
                    }
                }
            }
            _ => {}
        }
    }
}

implement_metric_trait!(Cyclomatic, PreprocCode, CcommentCode);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::check_metrics;

    #[test]
    fn python_simple_function() {
        check_metrics::<PythonParser>(
            "def f(a, b): # +2 (+1 unit space)
                if a and b:  # +2 (+1 and)
                   return 1
                if c and d: # +2 (+1 and)
                   return 1",
            "foo.py",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 6.0,
                      "average": 3.0,
                      "min": 1.0,
                      "max": 5.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_1_level_nesting() {
        check_metrics::<PythonParser>(
            "def f(a, b): # +2 (+1 unit space)
                if a:  # +1
                    for i in range(b):  # +1
                        return 1",
            "foo.py",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 4.0,
                      "average": 2.0,
                      "min": 1.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_1_level_nesting() {
        check_metrics::<ParserEngineRust>(
            "fn f() { // +2 (+1 unit space)
                 if true { // +1
                     match true {
                         true => println!(\"test\"), // +1
                         false => println!(\"test\"), // +1
                     }
                 }
             }",
            "foo.rs",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 5.0,
                      "average": 2.5,
                      "min": 1.0,
                      "max": 4.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_switch() {
        check_metrics::<CppParser>(
            "void f() { // +2 (+1 unit space)
                 switch (1) {
                     case 1: // +1
                         printf(\"one\");
                         break;
                     case 2: // +1
                         printf(\"two\");
                         break;
                     case 3: // +1
                         printf(\"three\");
                         break;
                     default:
                         printf(\"all\");
                         break;
                 }
             }",
            "foo.c",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 5.0,
                  "average": 2.5,
                  "min": 1.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn c_real_function() {
        check_metrics::<CppParser>(
            "int sumOfPrimes(int max) { // +2 (+1 unit space)
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }",
            "foo.c",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 5.0,
                  "average": 2.5,
                  "min": 1.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn c_unit_before() {
        check_metrics::<CppParser>(
            "
            int a=42;
            if(a==42) //+2(+1 unit space)
            {

            }
            if(a==34) //+1
            {

            }
            int sumOfPrimes(int max) { // +1
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }",
            "foo.c",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 7.0,
                  "average": 3.5,
                  "min": 3.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    /// Test to handle the case of min and max when merge happen before the final value of one module are set.
    /// In this case the min value should be 3 because the unit space has 2 branches and a complexity of 3
    /// while the function sumOfPrimes has a complexity of 4.
    #[test]
    fn c_unit_after() {
        check_metrics::<CppParser>(
            "
            int sumOfPrimes(int max) { // +1
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }

            int a=42;
            if(a==42) //+2(+1 unit space)
            {

            }
            if(a==34) //+1
            {

            }",
            "foo.c",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 7.0,
                  "average": 3.5,
                  "min": 3.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn java_simple_class() {
        check_metrics::<JavaParser>(
            "
            public class Example { // +2 (+1 unit space)
                int a = 10;
                boolean b = (a > 5) ? true : false; // +1
                boolean c = b && true; // +1

                public void m1() { // +1
                    if (a % 2 == 0) { // +1
                        b = b || c; // +1
                    }
                }
                public void m2() { // +1
                    while (a > 3) { // +1
                        m1();
                        a--;
                    }
                }
            }",
            "foo.java",
            |metric| {
                // nspace = 4 (unit, class and 2 methods)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 9.0,
                      "average": 2.25,
                      "min": 1.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_real_class() {
        check_metrics::<JavaParser>(
            "
            public class Matrix { // +2 (+1 unit space)
                private int[][] m = new int[5][5];

                public void init() { // +1
                    for (int i = 0; i < m.length; i++) { // +1
                        for (int j = 0; j < m[i].length; j++) { // +1
                            m[i][j] = i * j;
                        }
                    }
                }
                public int compute(int i, int j) { // +1
                    try {
                        return m[i][j] / m[j][i];
                    } catch (ArithmeticException e) { // +1
                        return -1;
                    } catch (ArrayIndexOutOfBoundsException e) { // +1
                        return -2;
                    }
                }
                public void print(int result) { // +1
                    switch (result) {
                        case -1: // +1
                            System.out.println(\"Division by zero\");
                            break;
                        case -2: // +1
                            System.out.println(\"Wrong index number\");
                            break;
                        default:
                            System.out.println(\"The result is \" + result);
                    }
                }
            }",
            "foo.java",
            |metric| {
                // nspace = 5 (unit, class and 3 methods)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 11.0,
                      "average": 2.2,
                      "min": 1.0,
                      "max": 3.0
                    }"###
                );
            },
        );
    }

    // As reported here:
    // https://github.com/sebastianbergmann/php-code-coverage/issues/607
    // An anonymous class declaration is not considered when computing the Cyclomatic Complexity metric for Java
    // Only the complexity of the anonymous class content is considered for the computation
    #[test]
    fn java_anonymous_class() {
        check_metrics::<JavaParser>(
            "
            abstract class A { // +2 (+1 unit space)
                public abstract boolean m1(int n); // +1
                public abstract boolean m2(int n); // +1
            }
            public class B { // +1

                public void test() { // +1
                    A a = new A() {
                        public boolean m1(int n) { // +1
                            if (n % 2 == 0) { // +1
                                return true;
                            }
                            return false;
                        }
                        public boolean m2(int n) { // +1
                            if (n % 5 == 0) { // +1
                                return true;
                            }
                            return false;
                        }
                    };
                }
            }",
            "foo.java",
            |metric| {
                // nspace = 8 (unit, 2 classes and 5 methods)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 10.0,
                      "average": 1.25,
                      "min": 1.0,
                      "max": 2.0
                    }"###
                );
            },
        );
    }

    // ==================== Kotlin Tests ====================

    #[test]
    fn kotlin_cyclomatic_simple_function() {
        check_metrics::<KotlinParser>(
            "fun greet() { // +2 (+1 unit space)
                return \"Hello\"
            }",
            "foo.kt",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 2.0,
                      "average": 1.0,
                      "min": 1.0,
                      "max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn kotlin_cyclomatic_if_when() {
        check_metrics::<KotlinParser>(
            "fun check(x: Int): String { // +2 (+1 unit space)
                if (x > 0) { // +1
                    return \"positive\"
                }
                when (x) { // +1
                    0 -> return \"zero\" // +1 (when_entry)
                    else -> return \"negative\" // +1 (when_entry)
                }
            }",
            "foo.kt",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 6.0,
                  "average": 3.0,
                  "min": 1.0,
                  "max": 5.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_cyclomatic_loops_and_boolean() {
        check_metrics::<KotlinParser>(
            "fun process(items: List<Int>): Int { // +2 (+1 unit space)
                var sum = 0
                for (i in items) { // +1
                    if (i > 0 && i < 100) { // +1 (if) +1 (&&)
                        sum += i
                    }
                }
                while (sum > 1000 || sum < 0) { // +1 (while) +1 (||)
                    sum /= 2
                }
                return sum
            }",
            "foo.kt",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 7.0,
                  "average": 3.5,
                  "min": 1.0,
                  "max": 6.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_cyclomatic_try_catch() {
        check_metrics::<KotlinParser>(
            "fun divide(a: Int, b: Int): Int { // +2 (+1 unit space)
                try { // +1
                    if (b == 0) { // +1
                        throw IllegalArgumentException(\"Division by zero\")
                    }
                    return a / b
                } catch (e: IllegalArgumentException) { // +1
                    return -1
                } catch (e: Exception) { // +1
                    return -2
                }
            }",
            "foo.kt",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 6.0,
                  "average": 3.0,
                  "min": 1.0,
                  "max": 5.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_cyclomatic_complex_when() {
        check_metrics::<KotlinParser>(
            "fun evaluate(score: Int): String { // +2 (+1 unit space)
                return when { // +1
                    score >= 90 && score <= 100 -> \"A\" // +1 (when_entry) +1 (&&)
                    score >= 80 -> \"B\" // +1 (when_entry)
                    score >= 70 -> \"C\" // +1 (when_entry)
                    score >= 60 -> \"D\" // +1 (when_entry)
                    else -> \"F\" // +1 (when_entry)
                }
            }",
            "foo.kt",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 9.0,
                  "average": 4.5,
                  "min": 1.0,
                  "max": 8.0
                }
                "#
                );
            },
        );
    }

    // ==================== Lua Tests ====================

    #[test]
    fn lua_cyclomatic_simple_function() {
        check_metrics::<LuaParser>(
            "function greet() -- +2 (+1 unit space)
                return \"Hello\"
            end",
            "foo.lua",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 2.0,
                      "average": 1.0,
                      "min": 1.0,
                      "max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn lua_cyclomatic_if_elseif() {
        check_metrics::<LuaParser>(
            "function check(x) -- +2 (+1 unit space)
                if x > 0 then -- +1
                    return \"positive\"
                elseif x < 0 then -- +1
                    return \"negative\"
                else
                    return \"zero\"
                end
            end",
            "foo.lua",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 4.0,
                  "average": 2.0,
                  "min": 1.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_cyclomatic_loops() {
        check_metrics::<LuaParser>(
            "function sum_array(arr) -- +2 (+1 unit space)
                local sum = 0
                for i = 1, #arr do -- +1
                    sum = sum + arr[i]
                end
                while sum > 100 do -- +1
                    sum = sum / 2
                end
                repeat -- +1
                    sum = sum - 1
                until sum <= 50
                return sum
            end",
            "foo.lua",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 5.0,
                  "average": 2.5,
                  "min": 1.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_cyclomatic_boolean_operators() {
        check_metrics::<LuaParser>(
            "function validate(x, y) -- +2 (+1 unit space)
                if x > 0 and y > 0 then -- +1 (if) +1 (and)
                    return true
                end
                if x < 0 or y < 0 then -- +1 (if) +1 (or)
                    return false
                end
                return nil
            end",
            "foo.lua",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 4.0,
                  "average": 2.0,
                  "min": 1.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_cyclomatic_nested_loops() {
        check_metrics::<LuaParser>(
            "function matrix_sum(matrix) -- +2 (+1 unit space)
                local total = 0
                for i = 1, #matrix do -- +1
                    for j = 1, #matrix[i] do -- +1
                        if matrix[i][j] > 0 and matrix[i][j] < 100 then -- +1 (if) +1 (and)
                            total = total + matrix[i][j]
                        end
                    end
                end
                return total
            end",
            "foo.lua",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 5.0,
                  "average": 2.5,
                  "min": 1.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    // ==================== Go Tests ====================

    #[test]
    fn go_cyclomatic_simple_function() {
        check_metrics::<GoParser>(
            "func greet() string { // +2 (+1 unit space)
                return \"Hello\"
            }",
            "foo.go",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r###"
                    {
                      "sum": 2.0,
                      "average": 1.0,
                      "min": 1.0,
                      "max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn go_cyclomatic_if_and_for() {
        check_metrics::<GoParser>(
            "func sumPositive(nums []int) int { // +2 (+1 unit space)
                sum := 0
                for _, n := range nums { // +1
                    if n > 0 { // +1
                        sum += n
                    }
                }
                return sum
            }",
            "foo.go",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 4.0,
                  "average": 2.0,
                  "min": 1.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_cyclomatic_switch_statement() {
        check_metrics::<GoParser>(
            "func classify(x int) string { // +2 (+1 unit space)
                switch { // +1
                case x > 0: // +1
                    return \"positive\"
                case x < 0: // +1
                    return \"negative\"
                default: // +1
                    return \"zero\"
                }
            }",
            "foo.go",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 5.0,
                  "average": 2.5,
                  "min": 1.0,
                  "max": 4.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_cyclomatic_boolean_operators() {
        check_metrics::<GoParser>(
            "func validate(x, y int) bool { // +2 (+1 unit space)
                if x > 0 && y > 0 { // +1 (if) +1 (&&)
                    return true
                }
                if x < 0 || y < 0 { // +1 (if) +1 (||)
                    return false
                }
                return x == y
            }",
            "foo.go",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 6.0,
                  "average": 3.0,
                  "min": 1.0,
                  "max": 5.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_cyclomatic_select_statement() {
        check_metrics::<GoParser>(
            "func receiver(ch1, ch2 chan int) int { // +2 (+1 unit space)
                select { // +1
                case v := <-ch1: // +1
                    if v > 0 { // +1
                        return v
                    }
                case v := <-ch2: // +1
                    return v * 2
                default: // +1
                    return 0
                }
            }",
            "foo.go",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 7.0,
                  "average": 3.5,
                  "min": 1.0,
                  "max": 6.0
                }
                "#
                );
            },
        );
    }

    // ==================== C# Tests ====================

    #[test]
    fn csharp_cyclomatic_simple_method() {
        check_metrics::<CsharpParser>(
            "public string Greet() { // +2 (+1 unit space)
                return \"Hello\";
            }",
            "foo.cs",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 1.0,
                  "average": 1.0,
                  "min": 1.0,
                  "max": 1.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_cyclomatic_if_and_ternary() {
        check_metrics::<CsharpParser>(
            "public int Check(int x) { // +2 (+1 unit space)
                if (x > 0) { // +1
                    return x;
                }
                return x < 0 ? -x : 0; // +1 (ternary)
            }",
            "foo.cs",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 3.0,
                  "average": 3.0,
                  "min": 3.0,
                  "max": 3.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_cyclomatic_loops() {
        check_metrics::<CsharpParser>(
            "public int SumArray(int[] arr) { // +2 (+1 unit space)
                int sum = 0;
                for (int i = 0; i < arr.Length; i++) { // +1
                    sum += arr[i];
                }
                while (sum > 100) { // +1
                    sum /= 2;
                }
                foreach (var item in arr) { // +1
                    if (item < 0) { // +1
                        sum -= item;
                    }
                }
                return sum;
            }",
            "foo.cs",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 5.0,
                  "average": 5.0,
                  "min": 5.0,
                  "max": 5.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_cyclomatic_switch_statement() {
        check_metrics::<CsharpParser>(
            "public string Grade(int score) { // +2 (+1 unit space)
                switch (score) { // +1
                    case 90: // +1
                    case 91: // +1
                        return \"A\";
                    case 80: // +1
                        return \"B\";
                    case 70: // +1
                        return \"C\";
                    default: // +1
                        return \"F\";
                }
            }",
            "foo.cs",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 7.0,
                  "average": 7.0,
                  "min": 7.0,
                  "max": 7.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_cyclomatic_try_catch() {
        check_metrics::<CsharpParser>(
            "public int Divide(int a, int b) { // +2 (+1 unit space)
                try { // +1
                    if (b == 0) { // +1
                        throw new ArgumentException(\"Division by zero\");
                    }
                    return a / b;
                } catch (ArgumentException e) { // +1
                    return -1;
                } catch (Exception e) { // +1
                    return -2;
                }
            }",
            "foo.cs",
            |metric| {
                // nspace = 2 (func and unit)
                insta::assert_json_snapshot!(
                    metric.cyclomatic,
                    @r#"
                {
                  "sum": 5.0,
                  "average": 5.0,
                  "min": 5.0,
                  "max": 5.0
                }
                "#
                );
            },
        );
    }
}
