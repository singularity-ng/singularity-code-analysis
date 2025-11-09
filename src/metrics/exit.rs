use std::fmt;

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::{checker::Checker, macros::implement_metric_trait, *};

/// The `NExit` metric.
///
/// This metric counts the number of possible exit points
/// from a function/method.
#[derive(Debug, Clone)]
pub struct Stats {
    exit: usize,
    exit_sum: usize,
    total_space_functions: usize,
    exit_min: usize,
    exit_max: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            exit: 0,
            exit_sum: 0,
            total_space_functions: 1,
            exit_min: usize::MAX,
            exit_max: 0,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("nexits", 4)?;
        st.serialize_field("sum", &self.exit_sum())?;
        st.serialize_field("average", &self.exit_average())?;
        st.serialize_field("min", &self.exit_min())?;
        st.serialize_field("max", &self.exit_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {} min: {}, max: {}",
            self.exit_sum(),
            self.exit_average(),
            self.exit_min(),
            self.exit_max()
        )
    }
}

impl Stats {
    /// Merges a second `NExit` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.exit_max = self.exit_max.max(other.exit_max);
        self.exit_min = self.exit_min.min(other.exit_min);
        self.exit_sum += other.exit_sum;
    }

    /// Returns the `NExit` metric value
    pub fn exit(&self) -> f64 {
        self.exit as f64
    }
    /// Returns the `NExit` metric sum value
    pub fn exit_sum(&self) -> f64 {
        self.exit_sum as f64
    }
    /// Returns the `NExit` metric  minimum value
    pub fn exit_min(&self) -> f64 {
        self.exit_min as f64
    }
    /// Returns the `NExit` metric maximum value
    pub fn exit_max(&self) -> f64 {
        self.exit_max as f64
    }

    /// Returns the `NExit` metric average value
    ///
    /// This value is computed dividing the `NExit` value
    /// for the total number of functions/closures in a space.
    ///
    /// If there are no functions in a code, its value is `NAN`.
    pub fn exit_average(&self) -> f64 {
        self.exit_sum() / self.total_space_functions as f64
    }
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.exit_sum += self.exit;
    }
    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        self.exit_max = self.exit_max.max(self.exit);
        self.exit_min = self.exit_min.min(self.exit);
        self.compute_sum();
    }
    pub(crate) fn finalize(&mut self, total_space_functions: usize) {
        self.total_space_functions = total_space_functions;
    }
}

pub trait Exit
where
    Self: Checker,
{
    fn compute(node: &Node, stats: &mut Stats);
}

impl Exit for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.kind_id().into(), Python::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.kind_id().into(), Mozjs::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.kind_id().into(), Javascript::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.kind_id().into(), Typescript::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.kind_id().into(), Tsx::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(
            node.kind_id().into(),
            Rust::ReturnExpression | Rust::TryExpression
        ) || Self::is_func(node) && node.child_by_field_name("return_type").is_some()
        {
            stats.exit += 1;
        }
    }
}

impl Exit for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.kind_id().into(), Cpp::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for JavaCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.kind_id().into(), Java::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

implement_metric_trait!(
    Exit,
    KotlinCode,
    PreprocCode,
    CcommentCode,
    ElixirCode,
    ErlangCode,
    GleamCode,
    LuaCode,
    GoCode,
    CsharpCode
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::check_metrics;

    #[test]
    fn python_no_exit() {
        check_metrics::<PythonParser>("a = 42", "foo.py", |metric| {
            // 0 functions
            insta::assert_json_snapshot!(
                metric.nexits,
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
    fn rust_no_exit() {
        check_metrics::<ParserEngineRust>("let a = 42;", "foo.rs", |metric| {
            // 0 functions
            insta::assert_json_snapshot!(
                metric.nexits,
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
    fn rust_question_mark() {
        check_metrics::<ParserEngineRust>("let _ = a? + b? + c?;", "foo.rs", |metric| {
            // 0 functions
            insta::assert_json_snapshot!(
                metric.nexits,
                @r###"
                    {
                      "sum": 3.0,
                      "average": null,
                      "min": 3.0,
                      "max": 3.0
                    }"###
            );
        });
    }

    #[test]
    fn c_no_exit() {
        check_metrics::<CppParser>("int a = 42;", "foo.c", |metric| {
            // 0 functions
            insta::assert_json_snapshot!(
                metric.nexits,
                @r#"
            {
              "sum": 0.0,
              "average": null,
              "min": 0.0,
              "max": 0.0
            }
            "#
            );
        });
    }

    #[test]
    fn javascript_no_exit() {
        check_metrics::<JavascriptParser>("var a = 42;", "foo.js", |metric| {
            // 0 functions
            insta::assert_json_snapshot!(
                metric.nexits,
                @r#"
            {
              "sum": 0.0,
              "average": null,
              "min": 0.0,
              "max": 0.0
            }
            "#
            );
        });
    }

    #[test]
    fn python_simple_function() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                 if a:
                     return a",
            "foo.py",
            |metric| {
                println!("{:?}", metric.nexits);
                // 1 function
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn python_more_functions() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                 if a:
                     return a
            def f(a, b):
                 if b:
                     return b",
            "foo.py",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 2.0,
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
    fn python_nested_functions() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                 def foo(a):
                     if a:
                         return 1
                 bar = lambda a: lambda b: b or True or True
                 return bar(foo(a))(a)",
            "foo.py",
            |metric| {
                // 2 functions + 2 lambdas = 4
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 2.0,
                  "average": 0.3333333333333333,
                  "min": 0.0,
                  "max": 1.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn java_no_exit() {
        check_metrics::<JavaParser>("int a = 42;", "foo.java", |metric| {
            // 0 functions
            insta::assert_json_snapshot!(
                metric.nexits,
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
    fn java_simple_function() {
        check_metrics::<JavaParser>(
            "class A {
              public int sum(int x, int y) {
                return x + y;
              }
            }",
            "foo.java",
            |metric| {
                // 1 exit / 1 space
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn java_split_function() {
        check_metrics::<JavaParser>(
            "class A {
              public int multiply(int x, int y) {
                if(x == 0 || y == 0){
                    return 0;
                }
                return x * y;
              }
            }",
            "foo.java",
            |metric| {
                // 2 exit / space 1
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn cpp_exit_single_return() {
        check_metrics::<CppParser>(
            "int add(int a, int b) {
                 return a + b;
             }",
            "foo.cpp",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn cpp_exit_multiple_returns() {
        check_metrics::<CppParser>(
            "int abs(int x) {
                 if (x < 0) {
                     return -x;
                 }
                 return x;
             }",
            "foo.cpp",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn cpp_exit_early_returns() {
        check_metrics::<CppParser>(
            "int process(int x) {
                 if (x < 0) return -1;
                 if (x == 0) return 0;
                 if (x > 100) return 100;
                 return x;
             }",
            "foo.cpp",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn cpp_exit_nested_functions() {
        check_metrics::<CppParser>(
            "int outer(int a) {
                 auto lambda = [](int x) { return x * 2; };
                 return lambda(a);
             }",
            "foo.cpp",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn cpp_exit_switch_statement() {
        check_metrics::<CppParser>(
            "int classify(int x) {
                 switch(x) {
                     case 1: return 10;
                     case 2: return 20;
                     default: return 0;
                 }
             }",
            "foo.cpp",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_exit_single_return() {
        check_metrics::<KotlinParser>(
            "fun add(a: Int, b: Int): Int { return a + b }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_exit_multiple_returns() {
        check_metrics::<KotlinParser>(
            "fun abs(x: Int): Int {
                 if (x < 0) {
                     return -x
                 }
                 return x
             }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_exit_early_returns() {
        check_metrics::<KotlinParser>(
            "fun process(x: Int): Int {
                 if (x < 0) return -1
                 if (x == 0) return 0
                 if (x > 100) return 100
                 return x
             }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_exit_when_expression() {
        check_metrics::<KotlinParser>(
            "fun classify(x: Int): Int {
                 return when(x) {
                     1 -> 10
                     2 -> 20
                     else -> 0
                 }
             }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_exit_exception() {
        check_metrics::<KotlinParser>(
            "fun validate(x: Int): Int {
                 if (x < 0) throw IllegalArgumentException(\"Negative\")
                 return x
             }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_exit_single_return() {
        check_metrics::<LuaParser>(
            "function add(a, b) return a + b end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_exit_multiple_returns() {
        check_metrics::<LuaParser>(
            "function abs(x)
                 if x < 0 then
                     return -x
                 end
                 return x
             end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_exit_early_returns() {
        check_metrics::<LuaParser>(
            "function process(x)
                 if x < 0 then return -1 end
                 if x == 0 then return 0 end
                 if x > 100 then return 100 end
                 return x
             end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_exit_nested_functions() {
        check_metrics::<LuaParser>(
            "function outer(a)
                 function inner(x)
                     return x * 2
                 end
                 return inner(a)
             end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_exit_implicit_return() {
        check_metrics::<LuaParser>(
            "function implicit()
                 local x = 42
             end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r###"
                    {
                      "sum": 0.0,
                      "average": 0.0,
                      "min": 0.0,
                      "max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn go_exit_single_return() {
        check_metrics::<GoParser>(
            "func add(a int, b int) int { return a + b }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_exit_multiple_returns() {
        check_metrics::<GoParser>(
            "func abs(x int) int {
                 if x < 0 {
                     return -x
                 }
                 return x
             }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_exit_early_returns() {
        check_metrics::<GoParser>(
            "func process(x int) int {
                 if x < 0 { return -1 }
                 if x == 0 { return 0 }
                 if x > 100 { return 100 }
                 return x
             }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_exit_multiple_return_values() {
        check_metrics::<GoParser>(
            "func divide(a int, b int) (int, error) {
                 if b == 0 {
                     return 0, errors.New(\"division by zero\")
                 }
                 return a / b, nil
             }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_exit_named_returns() {
        check_metrics::<GoParser>(
            "func calculate(x int) (result int) {
                 result = x * 2
                 return
             }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_exit_single_return() {
        check_metrics::<CsharpParser>(
            "int Add(int a, int b) { return a + b; }",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn csharp_exit_multiple_returns() {
        check_metrics::<CsharpParser>(
            "int Abs(int x) {
                 if (x < 0) {
                     return -x;
                 }
                 return x;
             }",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn csharp_exit_early_returns() {
        check_metrics::<CsharpParser>(
            "int Process(int x) {
                 if (x < 0) return -1;
                 if (x == 0) return 0;
                 if (x > 100) return 100;
                 return x;
             }",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn csharp_exit_exception() {
        check_metrics::<CsharpParser>(
            "int Validate(int x) {
                 if (x < 0) throw new ArgumentException(\"Negative\");
                 return x;
             }",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn csharp_exit_expression_body() {
        check_metrics::<CsharpParser>(
            "int Square(int x) => x * x;",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r###"
                    {
                      "sum": 0.0,
                      "average": null,
                      "min": 0.0,
                      "max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn typescript_exit_single_return() {
        check_metrics::<TypescriptParser>(
            "function add(a: number, b: number): number { return a + b; }",
            "foo.ts",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn typescript_exit_multiple_returns() {
        check_metrics::<TypescriptParser>(
            "function abs(x: number): number {
                 if (x < 0) {
                     return -x;
                 }
                 return x;
             }",
            "foo.ts",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn typescript_exit_early_returns() {
        check_metrics::<TypescriptParser>(
            "function process(x: number): number {
                 if (x < 0) return -1;
                 if (x === 0) return 0;
                 if (x > 100) return 100;
                 return x;
             }",
            "foo.ts",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn typescript_exit_arrow_function() {
        check_metrics::<TypescriptParser>(
            "const square = (x: number): number => { return x * x; };",
            "foo.ts",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
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
    fn typescript_exit_implicit_return() {
        check_metrics::<TypescriptParser>(
            "const double = (x: number): number => x * 2;",
            "foo.ts",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.nexits,
                    @r#"
                {
                  "sum": 0.0,
                  "average": 0.0,
                  "min": 0.0,
                  "max": 0.0
                }
                "#
                );
            },
        );
    }
}
