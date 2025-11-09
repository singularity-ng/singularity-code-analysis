use std::{collections::HashMap, fmt};

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use crate::{checker::Checker, getter::Getter, *};

/// The `Halstead` metric suite.
#[derive(Default, Clone, Debug)]
pub struct Stats {
    u_operators: u64,
    operators: u64,
    u_operands: u64,
    operands: u64,
}

/// Specifies the type of nodes accepted by the `Halstead` metric.
pub enum HalsteadType {
    /// The node is an `Halstead` operator
    Operator,
    /// The node is an `Halstead` operand
    Operand,
    /// The node is unknown to the `Halstead` metric
    Unknown,
}

#[derive(Debug, Default, Clone)]
pub struct HalsteadMaps<'a> {
    pub(crate) operators: HashMap<u16, u64>,
    pub(crate) operands: HashMap<&'a [u8], u64>,
}

impl<'a> HalsteadMaps<'a> {
    pub(crate) fn new() -> Self {
        HalsteadMaps {
            operators: HashMap::default(),
            operands: HashMap::default(),
        }
    }

    pub(crate) fn merge(&mut self, other: &HalsteadMaps<'a>) {
        for (k, v) in other.operators.iter() {
            *self.operators.entry(*k).or_insert(0) += v;
        }
        for (k, v) in other.operands.iter() {
            *self.operands.entry(*k).or_insert(0) += v;
        }
    }

    pub(crate) fn finalize(&self, stats: &mut Stats) {
        stats.u_operators = self.operators.len() as u64;
        stats.operators = self.operators.values().sum::<u64>();
        stats.u_operands = self.operands.len() as u64;
        stats.operands = self.operands.values().sum::<u64>();
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("halstead", 14)?;
        st.serialize_field("n1", &self.u_operators())?;
        st.serialize_field("N1", &self.operators())?;
        st.serialize_field("n2", &self.u_operands())?;
        st.serialize_field("N2", &self.operands())?;
        st.serialize_field("length", &self.length())?;
        st.serialize_field("estimated_program_length", &self.estimated_program_length())?;
        st.serialize_field("purity_ratio", &self.purity_ratio())?;
        st.serialize_field("vocabulary", &self.vocabulary())?;
        st.serialize_field("volume", &self.volume())?;
        st.serialize_field("difficulty", &self.difficulty())?;
        st.serialize_field("level", &self.level())?;
        st.serialize_field("effort", &self.effort())?;
        st.serialize_field("time", &self.time())?;
        st.serialize_field("bugs", &self.bugs())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "n1: {}, \
             N1: {}, \
             n2: {}, \
             N2: {}, \
             length: {}, \
             estimated program length: {}, \
             purity ratio: {}, \
             size: {}, \
             volume: {}, \
             difficulty: {}, \
             level: {}, \
             effort: {}, \
             time: {}, \
             bugs: {}",
            self.u_operators(),
            self.operators(),
            self.u_operands(),
            self.operands(),
            self.length(),
            self.estimated_program_length(),
            self.purity_ratio(),
            self.vocabulary(),
            self.volume(),
            self.difficulty(),
            self.level(),
            self.effort(),
            self.time(),
            self.bugs(),
        )
    }
}

impl Stats {
    pub(crate) fn merge(&self, _other: &Stats) {}

    /// Returns `η1`, the number of distinct operators
    #[inline(always)]
    pub fn u_operators(&self) -> f64 {
        self.u_operators as f64
    }

    /// Returns `N1`, the number of total operators
    #[inline(always)]
    pub fn operators(&self) -> f64 {
        self.operators as f64
    }

    /// Returns `η2`, the number of distinct operands
    #[inline(always)]
    pub fn u_operands(&self) -> f64 {
        self.u_operands as f64
    }

    /// Returns `N2`, the number of total operands
    #[inline(always)]
    pub fn operands(&self) -> f64 {
        self.operands as f64
    }

    /// Returns the program length
    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.operands() + self.operators()
    }

    /// Returns the calculated estimated program length
    #[inline(always)]
    pub fn estimated_program_length(&self) -> f64 {
        self.u_operators() * self.u_operators().log2()
            + self.u_operands() * self.u_operands().log2()
    }

    /// Returns the purity ratio
    #[inline(always)]
    pub fn purity_ratio(&self) -> f64 {
        self.estimated_program_length() / self.length()
    }

    /// Returns the program vocabulary
    #[inline(always)]
    pub fn vocabulary(&self) -> f64 {
        self.u_operands() + self.u_operators()
    }

    /// Returns the program volume.
    ///
    /// Unit of measurement: bits
    #[inline(always)]
    pub fn volume(&self) -> f64 {
        // Assumes a uniform binary encoding for the vocabulary is used.
        self.length() * self.vocabulary().log2()
    }

    /// Returns the estimated difficulty required to program
    #[inline(always)]
    pub fn difficulty(&self) -> f64 {
        self.u_operators() / 2. * self.operands() / self.u_operands()
    }

    /// Returns the estimated level of difficulty required to program
    #[inline(always)]
    pub fn level(&self) -> f64 {
        1. / self.difficulty()
    }

    /// Returns the estimated effort required to program
    #[inline(always)]
    pub fn effort(&self) -> f64 {
        self.difficulty() * self.volume()
    }

    /// Returns the estimated time required to program.
    ///
    /// Unit of measurement: seconds
    #[inline(always)]
    pub fn time(&self) -> f64 {
        // The floating point `18.` aims to describe the processing rate of the
        // human brain. It is called Stoud number, S, and its
        // unit of measurement is moments/seconds.
        // A moment is the time required by the human brain to carry out the
        // most elementary decision.
        // 5 <= S <= 20. Halstead uses 18.
        // The value of S has been empirically developed from psychological
        // reasoning, and its recommended value for
        // programming applications is 18.
        //
        // Source: https://www.geeksforgeeks.org/software-engineering-halsteads-software-metrics/
        self.effort() / 18.
    }

    /// Returns the estimated number of delivered bugs.
    ///
    /// This metric represents the average amount of work a programmer can do
    /// without introducing an error.
    #[inline(always)]
    pub fn bugs(&self) -> f64 {
        // The floating point `3000.` represents the number of elementary
        // mental discriminations.
        // A mental discrimination, in psychology, is the ability to perceive
        // and respond to differences among stimuli.
        //
        // The value above is obtained starting from a constant that
        // is different for every language and assumes that natural language is
        // the language of the brain.
        // For programming languages, the English language constant
        // has been considered.
        //
        // After every 3000 mental discriminations a result is produced.
        // This result, whether correct or incorrect, is more than likely
        // either used as an input for the next operation or is output to the
        // environment.
        // If incorrect the error should become apparent.
        // Thus, an opportunity for error occurs every 3000
        // mental discriminations.
        //
        // Source: https://docs.lib.purdue.edu/cgi/viewcontent.cgi?article=1145&context=cstech
        self.effort().powf(2. / 3.) / 3000.
    }
}

pub trait Halstead
where
    Self: Checker,
{
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>);
}

#[inline(always)]
fn get_id<'a>(node: &Node<'a>, code: &'a [u8]) -> &'a [u8] {
    &code[node.start_byte()..node.end_byte()]
}

#[inline(always)]
fn compute_halstead<'a, T: Getter>(
    node: &Node<'a>,
    code: &'a [u8],
    halstead_maps: &mut HalsteadMaps<'a>,
) {
    match T::get_op_type(node) {
        HalsteadType::Operator => {
            *halstead_maps.operators.entry(node.kind_id()).or_insert(0) += 1;
        }
        HalsteadType::Operand => {
            *halstead_maps
                .operands
                .entry(get_id(node, code))
                .or_insert(0) += 1;
        }
        _ => {}
    }
}

impl Halstead for PythonCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for MozjsCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for JavascriptCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for TypescriptCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for TsxCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for RustCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for CppCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for LuaCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for ElixirCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for ErlangCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for GleamCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for JavaCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for KotlinCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for PreprocCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for CcommentCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for GoCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for CsharpCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::check_metrics;

    #[test]
    fn python_operators_and_operands() {
        check_metrics::<PythonParser>(
            "def foo():
                 def bar():
                     def toto():
                        a = 1 + 1
                     b = 2 + a
                 c = 3 + 3",
            "foo.py",
            |metric| {
                // unique operators: def, =, +
                // operators: def, def, def, =, =, =, +, +, +
                // unique operands: foo, bar, toto, a, b, c, 1, 2, 3
                // operands: foo, bar, toto, a, b, c, 1, 1, 2, a, 3, 3
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 3.0,
                      "N1": 9.0,
                      "n2": 9.0,
                      "N2": 12.0,
                      "length": 21.0,
                      "estimated_program_length": 33.284212515144276,
                      "purity_ratio": 1.584962500721156,
                      "vocabulary": 12.0,
                      "volume": 75.28421251514428,
                      "difficulty": 2.0,
                      "level": 0.5,
                      "effort": 150.56842503028855,
                      "time": 8.364912501682698,
                      "bugs": 0.0094341190071077
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_operators_and_operands() {
        // Define operators and operands for C/C++ grammar according to this specification:
        // https://www.verifysoft.com/en_halstead_metrics.html
        // The only difference with the specification above is that
        // primitive types are treated as operators, since the definition of a
        // primitive type can be seen as the creation of a slot of a certain size.
        // i.e. The `int a;` definition creates a n-bytes slot.
        check_metrics::<CppParser>(
            "main()
            {
              int a, b, c, avg;
              scanf(\"%d %d %d\", &a, &b, &c);
              avg = (a + b + c) / 3;
              printf(\"avg = %d\", avg);
            }",
            "foo.c",
            |metric| {
                // unique operators: (), {}, int, &, =, +, /, ,, ;
                // unique operands: main, a, b, c, avg, scanf, "%d %d %d", 3, printf, "avg = %d"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 9.0,
                  "N1": 24.0,
                  "n2": 8.0,
                  "N2": 16.0,
                  "length": 40.0,
                  "estimated_program_length": 52.529325012980806,
                  "purity_ratio": 1.3132331253245202,
                  "vocabulary": 17.0,
                  "volume": 163.49851365001356,
                  "difficulty": 9.0,
                  "level": 0.1111111111111111,
                  "effort": 1471.486622850122,
                  "time": 81.74925682500678,
                  "bugs": 0.04312372727862395
                }
                "#
                );
            },
        );
    }

    #[test]
    fn rust_operators_and_operands() {
        check_metrics::<ParserEngineRust>(
            "fn main() {
              let a = 5; let b = 5; let c = 5;
              let avg = (a + b + c) / 3;
              println!(\"{}\", avg);
            }",
            "foo.rs",
            |metric| {
                // unique operators: fn, (), {}, let, =, +, /, ;, !, ,
                // unique operands: main, a, b, c, avg, 5, 3, println, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 10.0,
                  "N1": 23.0,
                  "n2": 9.0,
                  "N2": 15.0,
                  "length": 38.0,
                  "estimated_program_length": 61.74860596185444,
                  "purity_ratio": 1.624963314785643,
                  "vocabulary": 19.0,
                  "volume": 161.42124551085624,
                  "difficulty": 8.333333333333334,
                  "level": 0.12,
                  "effort": 1345.177045923802,
                  "time": 74.7320581068779,
                  "bugs": 0.040619232256751396
                }
                "#
                );
            },
        );
    }

    #[test]
    fn javascript_operators_and_operands() {
        check_metrics::<JavascriptParser>(
            "function main() {
              var a, b, c, avg;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.js",
            |metric| {
                // unique operators: function, (), {}, var, =, +, /, ,, ., ;
                // unique operands: main, a, b, c, avg, 3, 5, console.log, console, log, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 10.0,
                  "N1": 24.0,
                  "n2": 11.0,
                  "N2": 21.0,
                  "length": 45.0,
                  "estimated_program_length": 71.27302875388389,
                  "purity_ratio": 1.583845083419642,
                  "vocabulary": 21.0,
                  "volume": 197.65428402504423,
                  "difficulty": 9.545454545454545,
                  "level": 0.10476190476190476,
                  "effort": 1886.699983875422,
                  "time": 104.81666577085679,
                  "bugs": 0.05089564733125986
                }
                "#
                );
            },
        );
    }

    #[test]
    fn mozjs_operators_and_operands() {
        check_metrics::<MozjsParser>(
            "function main() {
              var a, b, c, avg;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.js",
            |metric| {
                // unique operators: function, (), {}, var, =, +, /, ,, ., ;
                // unique operands: main, a, b, c, avg, 3, 5, console.log, console, log, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 10.0,
                  "N1": 24.0,
                  "n2": 11.0,
                  "N2": 21.0,
                  "length": 45.0,
                  "estimated_program_length": 71.27302875388389,
                  "purity_ratio": 1.583845083419642,
                  "vocabulary": 21.0,
                  "volume": 197.65428402504423,
                  "difficulty": 9.545454545454545,
                  "level": 0.10476190476190476,
                  "effort": 1886.699983875422,
                  "time": 104.81666577085679,
                  "bugs": 0.05089564733125986
                }
                "#
                );
            },
        );
    }

    #[test]
    fn typescript_operators_and_operands() {
        check_metrics::<TypescriptParser>(
            "function main() {
              var a, b, c, avg;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.ts",
            |metric| {
                // unique operators: function, (), {}, var, =, +, /, ,, ., ;
                // unique operands: main, a, b, c, avg, 3, 5, console.log, console, log, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 10.0,
                      "N1": 24.0,
                      "n2": 11.0,
                      "N2": 21.0,
                      "length": 45.0,
                      "estimated_program_length": 71.27302875388389,
                      "purity_ratio": 1.583845083419642,
                      "vocabulary": 21.0,
                      "volume": 197.65428402504423,
                      "difficulty": 9.545454545454545,
                      "level": 0.10476190476190476,
                      "effort": 1886.699983875422,
                      "time": 104.81666577085679,
                      "bugs": 0.05089564733125986
                    }"###
                );
            },
        );
    }

    #[test]
    fn tsx_operators_and_operands() {
        check_metrics::<TsxParser>(
            "function main() {
              var a, b, c, avg;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.ts",
            |metric| {
                // unique operators: function, (), {}, var, =, +, /, ,, ., ;
                // unique operands: main, a, b, c, avg, 3, 5, console.log, console, log, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 10.0,
                      "N1": 24.0,
                      "n2": 11.0,
                      "N2": 21.0,
                      "length": 45.0,
                      "estimated_program_length": 71.27302875388389,
                      "purity_ratio": 1.583845083419642,
                      "vocabulary": 21.0,
                      "volume": 197.65428402504423,
                      "difficulty": 9.545454545454545,
                      "level": 0.10476190476190476,
                      "effort": 1886.699983875422,
                      "time": 104.81666577085679,
                      "bugs": 0.05089564733125986
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_wrong_operators() {
        check_metrics::<PythonParser>("()[]{}", "foo.py", |metric| {
            insta::assert_json_snapshot!(
                metric.halstead,
                @r###"
                    {
                      "n1": 0.0,
                      "N1": 0.0,
                      "n2": 0.0,
                      "N2": 0.0,
                      "length": 0.0,
                      "estimated_program_length": null,
                      "purity_ratio": null,
                      "vocabulary": 0.0,
                      "volume": null,
                      "difficulty": null,
                      "level": null,
                      "effort": null,
                      "time": null,
                      "bugs": null
                    }"###
            );
        });
    }

    #[test]
    fn python_check_metrics() {
        check_metrics::<PythonParser>(
            "def f():
                 pass",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 2.0,
                      "N1": 2.0,
                      "n2": 1.0,
                      "N2": 1.0,
                      "length": 3.0,
                      "estimated_program_length": 2.0,
                      "purity_ratio": 0.6666666666666666,
                      "vocabulary": 3.0,
                      "volume": 4.754887502163468,
                      "difficulty": 1.0,
                      "level": 1.0,
                      "effort": 4.754887502163468,
                      "time": 0.26416041678685936,
                      "bugs": 0.0009425525573729414
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_operators_and_operands() {
        check_metrics::<JavaParser>(
            "public class Main {
            public static void main(string args[]) {
                  int a, b, c, avg;
                  a = 5; b = 5; c = 5;
                  avg = (a + b + c) / 3;
                  MessageFormat.format(\"{0}\", avg);
                }
            }",
            "foo.java",
            |metric| {
                // { void ; ( String [ ] ) , int = + / format . }
                // Main main args a b c avg 5 3 MessageFormat format "{0}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 10.0,
                  "N1": 25.0,
                  "n2": 12.0,
                  "N2": 22.0,
                  "length": 47.0,
                  "estimated_program_length": 76.2388309575275,
                  "purity_ratio": 1.6221027863303723,
                  "vocabulary": 22.0,
                  "volume": 209.59328607595296,
                  "difficulty": 9.166666666666666,
                  "level": 0.1090909090909091,
                  "effort": 1921.2717890295687,
                  "time": 106.73732161275382,
                  "bugs": 0.05151550353617788
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_halstead_simple() {
        check_metrics::<KotlinParser>("val x = 1 + 2", "foo.kt", |metric| {
            insta::assert_json_snapshot!(
                metric.halstead,
                @r#"
            {
              "n1": 3.0,
              "N1": 3.0,
              "n2": 1.0,
              "N2": 1.0,
              "length": 4.0,
              "estimated_program_length": 4.754887502163468,
              "purity_ratio": 1.188721875540867,
              "vocabulary": 4.0,
              "volume": 8.0,
              "difficulty": 1.5,
              "level": 0.6666666666666666,
              "effort": 12.0,
              "time": 0.6666666666666666,
              "bugs": 0.0017471609294725976
            }
            "#
            );
        });
    }

    #[test]
    fn kotlin_halstead_moderate() {
        check_metrics::<KotlinParser>(
            "fun add(a: Int, b: Int): Int { return a + b }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 6.0,
                  "N1": 6.0,
                  "n2": 4.0,
                  "N2": 8.0,
                  "length": 14.0,
                  "estimated_program_length": 23.509775004326936,
                  "purity_ratio": 1.6792696431662097,
                  "vocabulary": 10.0,
                  "volume": 46.50699332842307,
                  "difficulty": 6.0,
                  "level": 0.16666666666666666,
                  "effort": 279.0419599705384,
                  "time": 15.502331109474357,
                  "bugs": 0.014233938588537278
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_halstead_complex() {
        check_metrics::<KotlinParser>(
            "fun calculate(a: Int, b: Int, c: Int): Int {
                 val x = a * b + c
                 val y = x / 2 - a
                 return if (y > 0) y else -y
             }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 14.0,
                  "N1": 19.0,
                  "n2": 7.0,
                  "N2": 18.0,
                  "length": 37.0,
                  "estimated_program_length": 72.95445336320968,
                  "purity_ratio": 1.971741982789451,
                  "vocabulary": 21.0,
                  "volume": 162.51574464281416,
                  "difficulty": 18.0,
                  "level": 0.05555555555555555,
                  "effort": 2925.2834035706546,
                  "time": 162.51574464281416,
                  "bugs": 0.06818005963549205
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_halstead_boolean_heavy() {
        check_metrics::<KotlinParser>(
            "fun validate(x: Int, y: Int): Boolean { return x > 0 && y > 0 || x < 0 && y < 0 }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 9.0,
                  "N1": 12.0,
                  "n2": 5.0,
                  "N2": 10.0,
                  "length": 22.0,
                  "estimated_program_length": 40.13896548741762,
                  "purity_ratio": 1.8244984312462555,
                  "vocabulary": 14.0,
                  "volume": 83.76180828526729,
                  "difficulty": 9.0,
                  "level": 0.1111111111111111,
                  "effort": 753.8562745674056,
                  "time": 41.880904142633646,
                  "bugs": 0.027610299305771368
                }
                "#
                );
            },
        );
    }

    #[test]
    fn kotlin_halstead_mixed_operators() {
        check_metrics::<KotlinParser>(
            "fun process(a: Int): String { return \"Result: ${a * 2 + (a / 3)}\" }",
            "foo.kt",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 7.0,
                  "N1": 8.0,
                  "n2": 5.0,
                  "N2": 7.0,
                  "length": 15.0,
                  "estimated_program_length": 31.26112492884004,
                  "purity_ratio": 2.0840749952560027,
                  "vocabulary": 12.0,
                  "volume": 53.77443751081734,
                  "difficulty": 4.9,
                  "level": 0.2040816326530612,
                  "effort": 263.494743803005,
                  "time": 14.638596877944723,
                  "bugs": 0.013700193974307346
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_halstead_simple() {
        check_metrics::<LuaParser>("local x = 1 + 2", "foo.lua", |metric| {
            insta::assert_json_snapshot!(
                metric.halstead,
                @r#"
            {
              "n1": 0.0,
              "N1": 0.0,
              "n2": 0.0,
              "N2": 0.0,
              "length": 0.0,
              "estimated_program_length": null,
              "purity_ratio": null,
              "vocabulary": 0.0,
              "volume": null,
              "difficulty": null,
              "level": null,
              "effort": null,
              "time": null,
              "bugs": null
            }
            "#
            );
        });
    }

    #[test]
    fn lua_halstead_moderate() {
        check_metrics::<LuaParser>(
            "function add(a, b) return a + b end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 6.0,
                  "N1": 6.0,
                  "n2": 3.0,
                  "N2": 5.0,
                  "length": 11.0,
                  "estimated_program_length": 20.264662506490403,
                  "purity_ratio": 1.842242046044582,
                  "vocabulary": 9.0,
                  "volume": 34.86917501586544,
                  "difficulty": 5.0,
                  "level": 0.2,
                  "effort": 174.3458750793272,
                  "time": 9.68588194885151,
                  "bugs": 0.010402870600353142
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_halstead_complex() {
        check_metrics::<LuaParser>(
            "function calculate(a, b, c)
                 local x = a * b + c
                 local y = x / 2 - a
                 if y > 0 then return y else return -y end
             end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 15.0,
                  "N1": 21.0,
                  "n2": 8.0,
                  "N2": 16.0,
                  "length": 37.0,
                  "estimated_program_length": 82.60335893412778,
                  "purity_ratio": 2.232523214435886,
                  "vocabulary": 23.0,
                  "volume": 167.37179237410948,
                  "difficulty": 15.0,
                  "level": 0.06666666666666667,
                  "effort": 2510.576885611642,
                  "time": 139.4764936450912,
                  "bugs": 0.06157358344691786
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_halstead_boolean_heavy() {
        check_metrics::<LuaParser>(
            "function validate(x, y) return x > 0 and y > 0 or x < 0 and y < 0 end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 9.0,
                  "N1": 12.0,
                  "n2": 4.0,
                  "N2": 11.0,
                  "length": 23.0,
                  "estimated_program_length": 36.529325012980806,
                  "purity_ratio": 1.5882315223035133,
                  "vocabulary": 13.0,
                  "volume": 85.11011351724513,
                  "difficulty": 12.375,
                  "level": 0.08080808080808081,
                  "effort": 1053.2376547759084,
                  "time": 58.51320304310602,
                  "bugs": 0.0345061360406794
                }
                "#
                );
            },
        );
    }

    #[test]
    fn lua_halstead_mixed_operators() {
        check_metrics::<LuaParser>(
            "function process(a) return \"Result: \" .. (a * 2 + a / 3) end",
            "foo.lua",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 8.0,
                  "N1": 9.0,
                  "n2": 5.0,
                  "N2": 7.0,
                  "length": 16.0,
                  "estimated_program_length": 35.60964047443681,
                  "purity_ratio": 2.2256025296523005,
                  "vocabulary": 13.0,
                  "volume": 59.207035490257475,
                  "difficulty": 5.6,
                  "level": 0.17857142857142858,
                  "effort": 331.55939874544185,
                  "time": 18.41996659696899,
                  "bugs": 0.015968090091229327
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_halstead_simple() {
        check_metrics::<GoParser>("var x = 1 + 2", "foo.go", |metric| {
            insta::assert_json_snapshot!(
                metric.halstead,
                @r#"
            {
              "n1": 3.0,
              "N1": 3.0,
              "n2": 3.0,
              "N2": 3.0,
              "length": 6.0,
              "estimated_program_length": 9.509775004326936,
              "purity_ratio": 1.584962500721156,
              "vocabulary": 6.0,
              "volume": 15.509775004326936,
              "difficulty": 1.5,
              "level": 0.6666666666666666,
              "effort": 23.264662506490403,
              "time": 1.292481250360578,
              "bugs": 0.0027165012951989257
            }
            "#
            );
        });
    }

    #[test]
    fn go_halstead_moderate() {
        check_metrics::<GoParser>(
            "func add(a int, b int) int { return a + b }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 6.0,
                  "N1": 6.0,
                  "n2": 3.0,
                  "N2": 5.0,
                  "length": 11.0,
                  "estimated_program_length": 20.264662506490403,
                  "purity_ratio": 1.842242046044582,
                  "vocabulary": 9.0,
                  "volume": 34.86917501586544,
                  "difficulty": 5.0,
                  "level": 0.2,
                  "effort": 174.3458750793272,
                  "time": 9.68588194885151,
                  "bugs": 0.010402870600353142
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_halstead_complex() {
        check_metrics::<GoParser>(
            "func calculate(a int, b int, c int) int {
                 x := a * b + c
                 y := x / 2 - a
                 if y > 0 {
                     return y
                 }
                 return -y
             }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 12.0,
                  "N1": 17.0,
                  "n2": 8.0,
                  "N2": 16.0,
                  "length": 33.0,
                  "estimated_program_length": 67.01955000865388,
                  "purity_ratio": 2.0308954548076934,
                  "vocabulary": 20.0,
                  "volume": 142.62362713128297,
                  "difficulty": 12.0,
                  "level": 0.08333333333333333,
                  "effort": 1711.4835255753956,
                  "time": 95.08241808752197,
                  "bugs": 0.04769365003766825
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_halstead_boolean_heavy() {
        check_metrics::<GoParser>(
            "func validate(x int, y int) bool { return x > 0 && y > 0 || x < 0 && y < 0 }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 9.0,
                  "N1": 12.0,
                  "n2": 4.0,
                  "N2": 11.0,
                  "length": 23.0,
                  "estimated_program_length": 36.529325012980806,
                  "purity_ratio": 1.5882315223035133,
                  "vocabulary": 13.0,
                  "volume": 85.11011351724513,
                  "difficulty": 12.375,
                  "level": 0.08080808080808081,
                  "effort": 1053.2376547759084,
                  "time": 58.51320304310602,
                  "bugs": 0.0345061360406794
                }
                "#
                );
            },
        );
    }

    #[test]
    fn go_halstead_mixed_operators() {
        check_metrics::<GoParser>(
            "func process(a int) string { return fmt.Sprintf(\"Result: %d\", a * 2 + a / 3) }",
            "foo.go",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 9.0,
                  "N1": 10.0,
                  "n2": 6.0,
                  "N2": 8.0,
                  "length": 18.0,
                  "estimated_program_length": 44.039100017307746,
                  "purity_ratio": 2.446616667628208,
                  "vocabulary": 15.0,
                  "volume": 70.32403072095333,
                  "difficulty": 6.0,
                  "level": 0.16666666666666666,
                  "effort": 421.94418432572,
                  "time": 23.44134357365111,
                  "bugs": 0.018752049849923146
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_halstead_simple() {
        check_metrics::<CsharpParser>("var x = 1 + 2;", "foo.cs", |metric| {
            insta::assert_json_snapshot!(
                metric.halstead,
                @r#"
            {
              "n1": 4.0,
              "N1": 4.0,
              "n2": 3.0,
              "N2": 3.0,
              "length": 7.0,
              "estimated_program_length": 12.754887502163468,
              "purity_ratio": 1.8221267860233525,
              "vocabulary": 7.0,
              "volume": 19.651484454403228,
              "difficulty": 2.0,
              "level": 0.5,
              "effort": 39.302968908806456,
              "time": 2.1834982727114696,
              "bugs": 0.0038532659414573967
            }
            "#
            );
        });
    }

    #[test]
    fn csharp_halstead_moderate() {
        check_metrics::<CsharpParser>(
            "int Add(int a, int b) { return a + b; }",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 6.0,
                  "N1": 6.0,
                  "n2": 3.0,
                  "N2": 5.0,
                  "length": 11.0,
                  "estimated_program_length": 20.264662506490403,
                  "purity_ratio": 1.842242046044582,
                  "vocabulary": 9.0,
                  "volume": 34.86917501586544,
                  "difficulty": 5.0,
                  "level": 0.2,
                  "effort": 174.3458750793272,
                  "time": 9.68588194885151,
                  "bugs": 0.010402870600353142
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_halstead_complex() {
        check_metrics::<CsharpParser>(
            "int Calculate(int a, int b, int c) {
                 var x = a * b + c;
                 var y = x / 2 - a;
                 return y > 0 ? y : -y;
             }",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 14.0,
                  "N1": 20.0,
                  "n2": 8.0,
                  "N2": 16.0,
                  "length": 36.0,
                  "estimated_program_length": 77.30296890880646,
                  "purity_ratio": 2.1473046919112906,
                  "vocabulary": 22.0,
                  "volume": 160.5395382709427,
                  "difficulty": 14.0,
                  "level": 0.07142857142857142,
                  "effort": 2247.5535357931976,
                  "time": 124.86408532184431,
                  "bugs": 0.057194215680663914
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_halstead_boolean_heavy() {
        check_metrics::<CsharpParser>(
            "bool Validate(int x, int y) { return x > 0 && y > 0 || x < 0 && y < 0; }",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 9.0,
                  "N1": 12.0,
                  "n2": 4.0,
                  "N2": 11.0,
                  "length": 23.0,
                  "estimated_program_length": 36.529325012980806,
                  "purity_ratio": 1.5882315223035133,
                  "vocabulary": 13.0,
                  "volume": 85.11011351724513,
                  "difficulty": 12.375,
                  "level": 0.08080808080808081,
                  "effort": 1053.2376547759084,
                  "time": 58.51320304310602,
                  "bugs": 0.0345061360406794
                }
                "#
                );
            },
        );
    }

    #[test]
    fn csharp_halstead_mixed_operators() {
        check_metrics::<CsharpParser>(
            "string Process(int a) { return $\"Result: {a * 2 + a / 3}\"; }",
            "foo.cs",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 7.0,
                  "N1": 7.0,
                  "n2": 5.0,
                  "N2": 7.0,
                  "length": 14.0,
                  "estimated_program_length": 31.26112492884004,
                  "purity_ratio": 2.2329374949171457,
                  "vocabulary": 12.0,
                  "volume": 50.18947501009619,
                  "difficulty": 4.9,
                  "level": 0.2040816326530612,
                  "effort": 245.92842754947134,
                  "time": 13.662690419415075,
                  "bugs": 0.01308432231664305
                }
                "#
                );
            },
        );
    }

    #[test]
    fn elixir_halstead_simple() {
        check_metrics::<ElixirParser>("x = 1 + 2", "foo.ex", |metric| {
            insta::assert_json_snapshot!(
                metric.halstead,
                @r#"
            {
              "n1": 2.0,
              "N1": 3.0,
              "n2": 3.0,
              "N2": 3.0,
              "length": 6.0,
              "estimated_program_length": 6.754887502163468,
              "purity_ratio": 1.1258145836939113,
              "vocabulary": 5.0,
              "volume": 13.931568569324174,
              "difficulty": 1.0,
              "level": 1.0,
              "effort": 13.931568569324174,
              "time": 0.7739760316291208,
              "bugs": 0.0019299471801733172
            }
            "#
            );
        });
    }

    #[test]
    fn elixir_halstead_moderate() {
        check_metrics::<ElixirParser>(
            "def add(a, b) do a + b end",
            "foo.ex",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 5.0,
                  "N1": 7.0,
                  "n2": 4.0,
                  "N2": 6.0,
                  "length": 13.0,
                  "estimated_program_length": 19.60964047443681,
                  "purity_ratio": 1.5084338826489854,
                  "vocabulary": 9.0,
                  "volume": 41.209025018750054,
                  "difficulty": 3.75,
                  "level": 0.26666666666666666,
                  "effort": 154.5338438203127,
                  "time": 8.585213545572929,
                  "bugs": 0.009599040339100903
                }
                "#
                );
            },
        );
    }

    #[test]
    fn elixir_halstead_complex() {
        check_metrics::<ElixirParser>(
            "def calculate(a, b, c) do
                 x = a * b + c
                 y = div(x, 2) - a
                 if y > 0, do: y, else: -y
             end",
            "foo.ex",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 9.0,
                  "N1": 21.0,
                  "n2": 12.0,
                  "N2": 20.0,
                  "length": 41.0,
                  "estimated_program_length": 71.54887502163469,
                  "purity_ratio": 1.7450945127227973,
                  "vocabulary": 21.0,
                  "volume": 180.0850143339292,
                  "difficulty": 7.5,
                  "level": 0.13333333333333333,
                  "effort": 1350.6376075044689,
                  "time": 75.03542263913715,
                  "bugs": 0.040729083484724496
                }
                "#
                );
            },
        );
    }

    #[test]
    fn elixir_halstead_boolean_heavy() {
        check_metrics::<ElixirParser>(
            "def validate(x, y), do: x > 0 and y > 0 or x < 0 and y < 0",
            "foo.ex",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 7.0,
                  "N1": 18.0,
                  "n2": 6.0,
                  "N2": 13.0,
                  "length": 31.0,
                  "estimated_program_length": 35.161259458730164,
                  "purity_ratio": 1.1342341760880699,
                  "vocabulary": 13.0,
                  "volume": 114.71363126237385,
                  "difficulty": 7.583333333333333,
                  "level": 0.13186813186813187,
                  "effort": 869.9117037396684,
                  "time": 48.32842798553713,
                  "bugs": 0.030375879491505737
                }
                "#
                );
            },
        );
    }

    #[test]
    fn elixir_halstead_mixed_operators() {
        check_metrics::<ElixirParser>(
            "def process(a), do: \"Result: #{a * 2 + div(a, 3)}\"",
            "foo.ex",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 5.0,
                  "N1": 10.0,
                  "n2": 8.0,
                  "N2": 10.0,
                  "length": 20.0,
                  "estimated_program_length": 35.60964047443681,
                  "purity_ratio": 1.7804820237218404,
                  "vocabulary": 13.0,
                  "volume": 74.00879436282185,
                  "difficulty": 3.125,
                  "level": 0.32,
                  "effort": 231.27748238381827,
                  "time": 12.848749021323236,
                  "bugs": 0.012559363956854735
                }
                "#
                );
            },
        );
    }

    #[test]
    fn gleam_halstead_simple() {
        check_metrics::<GleamParser>("let x = 1 + 2", "foo.gleam", |metric| {
            insta::assert_json_snapshot!(
                metric.halstead,
                @r#"
            {
              "n1": 4.0,
              "N1": 4.0,
              "n2": 3.0,
              "N2": 3.0,
              "length": 7.0,
              "estimated_program_length": 12.754887502163468,
              "purity_ratio": 1.8221267860233525,
              "vocabulary": 7.0,
              "volume": 19.651484454403228,
              "difficulty": 2.0,
              "level": 0.5,
              "effort": 39.302968908806456,
              "time": 2.1834982727114696,
              "bugs": 0.0038532659414573967
            }
            "#
            );
        });
    }

    #[test]
    fn gleam_halstead_moderate() {
        check_metrics::<GleamParser>(
            "pub fn add(a: Int, b: Int) -> Int { a + b }",
            "foo.gleam",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r#"
                {
                  "n1": 3.0,
                  "N1": 3.0,
                  "n2": 3.0,
                  "N2": 5.0,
                  "length": 8.0,
                  "estimated_program_length": 9.509775004326936,
                  "purity_ratio": 1.188721875540867,
                  "vocabulary": 6.0,
                  "volume": 20.67970000576925,
                  "difficulty": 2.5,
                  "level": 0.4,
                  "effort": 51.69925001442312,
                  "time": 2.87218055635684,
                  "bugs": 0.004625956812489424
                }
                "#
                );
            },
        );
    }
}
