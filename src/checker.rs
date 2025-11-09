use std::sync::OnceLock;

use aho_corasick::AhoCorasick;
use regex::bytes::Regex;

use crate::*;

static AHO_CORASICK: OnceLock<AhoCorasick> = OnceLock::new();
static RE: OnceLock<Regex> = OnceLock::new();

#[allow(unused_macros)]
macro_rules! check_if_func {
    ($parser: ident, $node: ident) => {
        $node.count_specific_ancestors::<$parser>(
            |node| {
                matches!(
                    node.kind_id().into(),
                    VariableDeclarator | AssignmentExpression | LabeledStatement | Pair
                )
            },
            |node| {
                matches!(
                    node.kind_id().into(),
                    StatementBlock | ReturnStatement | NewExpression | Arguments
                )
            },
        ) > 0
            || $node.is_child(Identifier as u16)
    };
}

macro_rules! is_js_func {
    ($parser: ident, $node: ident) => {
        matches!(
            $node.kind(),
            "function_declaration" | "method_definition" | "function_expression"
        )
    };
}

macro_rules! is_js_closure {
    ($parser: ident, $node: ident) => {
        matches!(
            $node.kind(),
            "arrow_function" | "generator_function" | "generator_function_declaration"
        )
    };
}

macro_rules! is_js_func_and_closure_checker {
    ($parser: ident, $language: ident) => {
        #[inline(always)]
        fn is_func(node: &Node) -> bool {
            is_js_func!($parser, node)
        }

        #[inline(always)]
        fn is_closure(node: &Node) -> bool {
            is_js_closure!($parser, node)
        }
    };
}

#[inline(always)]
fn get_aho_corasick_match(code: &[u8]) -> bool {
    AHO_CORASICK
        .get_or_init(|| AhoCorasick::new(vec![b"<div rustbindgen"]).unwrap())
        .is_match(code)
}

pub trait Checker {
    fn is_comment(_: &Node) -> bool;
    fn is_useful_comment(_: &Node, _: &[u8]) -> bool;
    fn is_func_space(_: &Node) -> bool;
    fn is_func(_: &Node) -> bool;
    fn is_closure(_: &Node) -> bool;
    fn is_call(_: &Node) -> bool;
    fn is_non_arg(_: &Node) -> bool;
    fn is_string(_: &Node) -> bool;
    fn is_else_if(_: &Node) -> bool;
    fn is_primitive(_id: u16) -> bool;

    fn is_error(node: &Node) -> bool {
        node.has_error()
    }
}

impl Checker for PreprocCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(_: &Node) -> bool {
        false
    }

    fn is_func(_: &Node) -> bool {
        false
    }

    fn is_closure(_: &Node) -> bool {
        false
    }

    fn is_call(_: &Node) -> bool {
        false
    }

    fn is_non_arg(_: &Node) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool {
        matches!(node.kind(), "string_literal" | "raw_string_literal")
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for CcommentCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        get_aho_corasick_match(&code[node.start_byte()..node.end_byte()])
    }

    fn is_func_space(_: &Node) -> bool {
        false
    }

    fn is_func(_: &Node) -> bool {
        false
    }

    fn is_closure(_: &Node) -> bool {
        false
    }

    fn is_call(_: &Node) -> bool {
        false
    }

    fn is_non_arg(_: &Node) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool {
        matches!(node.kind(), "string_literal" | "raw_string_literal")
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for CppCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        get_aho_corasick_match(&code[node.start_byte()..node.end_byte()])
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.kind(),
            "translation_unit"
                | "function_definition"
                | "struct_specifier"
                | "class_specifier"
                | "namespace_definition"
        )
    }

    fn is_func(node: &Node) -> bool {
        node.kind() == "function_definition"
    }

    fn is_closure(node: &Node) -> bool {
        node.kind() == "lambda_expression"
    }

    fn is_call(node: &Node) -> bool {
        node.kind() == "call_expression"
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(node.kind(), "(" | "," | ")")
    }

    fn is_string(node: &Node) -> bool {
        matches!(
            node.kind(),
            "string_literal" | "concatenated_string" | "raw_string_literal"
        )
    }

    fn is_else_if(node: &Node) -> bool {
        if node.kind() != "if_statement" {
            return false;
        }
        if let Some(parent) = node.parent() {
            return parent.kind() == "else_clause";
        }
        false
    }

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        // Since we're using kind strings now, we can't easily check this with just an ID
        // Keep the old enum check for now since this is used in other parts
        id == Cpp::PrimitiveType
    }
}

impl Checker for PythonCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        // comment containing coding info are useful
        node.start_row() <= 1
            && RE
                .get_or_init(|| {
                    Regex::new(r"^[ \t\f]*#.*?coding[:=][ \t]*([-_.a-zA-Z0-9]+)").unwrap()
                })
                .is_match(&code[node.start_byte()..node.end_byte()])
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.kind(),
            "module" | "function_definition" | "class_definition"
        )
    }

    fn is_func(node: &Node) -> bool {
        node.kind() == "function_definition"
    }

    fn is_closure(node: &Node) -> bool {
        node.kind() == "lambda"
    }

    fn is_call(node: &Node) -> bool {
        node.kind() == "call"
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(node.kind(), "(" | "," | ")")
    }

    fn is_string(node: &Node) -> bool {
        node.kind() == "string" || node.kind() == "concatenated_string"
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for JavaCode {
    fn is_comment(node: &Node) -> bool {
        matches!(node.kind(), "line_comment" | "block_comment")
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.kind(),
            "program" | "class_declaration" | "interface_declaration"
        )
    }

    fn is_func(node: &Node) -> bool {
        matches!(node.kind(), "method_declaration" | "constructor_declaration")
    }

    fn is_closure(node: &Node) -> bool {
        node.kind() == "lambda_expression"
    }

    fn is_call(node: &Node) -> bool {
        node.kind() == "method_invocation"
    }

    fn is_non_arg(_: &Node) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool {
        node.kind() == "string_literal"
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for MozjsCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.kind(),
            "program"
                | "function_expression"
                | "class"
                | "generator_function"
                | "function_declaration"
                | "method_definition"
                | "generator_function_declaration"
                | "class_declaration"
                | "arrow_function"
        )
    }

    is_js_func_and_closure_checker!(MozjsParser, Mozjs);

    fn is_call(node: &Node) -> bool {
        node.kind() == "call_expression"
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(node.kind(), "(" | "," | ")")
    }

    fn is_string(node: &Node) -> bool {
        node.kind() == "string" || node.kind() == "template_string"
    }

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.kind() != "if_statement" {
            return false;
        }
        if let Some(parent) = node.parent() {
            return parent.kind() == "else_clause";
        }
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for JavascriptCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.kind(),
            "program"
                | "function_expression"
                | "class"
                | "generator_function"
                | "function_declaration"
                | "method_definition"
                | "generator_function_declaration"
                | "class_declaration"
                | "arrow_function"
        )
    }

    is_js_func_and_closure_checker!(JavascriptParser, Javascript);

    fn is_call(node: &Node) -> bool {
        node.kind() == "call_expression"
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(node.kind(), "(" | "," | ")")
    }

    fn is_string(node: &Node) -> bool {
        node.kind() == "string" || node.kind() == "template_string"
    }

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.kind() != "if_statement" {
            return false;
        }
        if let Some(parent) = node.parent() {
            return node.kind() == "if_statement"
                && parent.kind() == "if_statement";
        }
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for TypescriptCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.kind(),
            "program"
                | "function_expression"
                | "class"
                | "generator_function"
                | "function_declaration"
                | "method_definition"
                | "generator_function_declaration"
                | "class_declaration"
                | "interface_declaration"
                | "arrow_function"
        )
    }

    is_js_func_and_closure_checker!(TypescriptParser, Typescript);

    fn is_call(node: &Node) -> bool {
        node.kind() == "call_expression"
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(node.kind(), "(" | "," | ")")
    }

    fn is_string(node: &Node) -> bool {
        node.kind() == "string" || node.kind() == "template_string"
    }

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.kind() != "if_statement" {
            return false;
        }
        if let Some(parent) = node.parent() {
            return parent.kind() == "else_clause";
        }
        false
    }

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Typescript::PredefinedType
    }
}

impl Checker for TsxCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.kind(),
            "program"
                | "function_expression"
                | "class"
                | "generator_function"
                | "function_declaration"
                | "method_definition"
                | "generator_function_declaration"
                | "class_declaration"
                | "interface_declaration"
                | "arrow_function"
        )
    }

    is_js_func_and_closure_checker!(TsxParser, Tsx);

    fn is_call(node: &Node) -> bool {
        node.kind() == "call_expression"
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(node.kind(), "(" | "," | ")")
    }

    fn is_string(node: &Node) -> bool {
        node.kind() == "string" || node.kind() == "template_string"
    }

    fn is_else_if(node: &Node) -> bool {
        if node.kind() != "if_statement" {
            return false;
        }
        if let Some(parent) = node.parent() {
            return node.kind() == "if_statement" && parent.kind() == "if_statement";
        }
        false
    }

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Tsx::PredefinedType
    }
}

impl Checker for RustCode {
    fn is_comment(node: &Node) -> bool {
        matches!(node.kind(), "line_comment" | "block_comment")
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        if let Some(parent) = node.parent() {
            if parent.kind() == "token_tree" {
                // A comment could be a macro token
                return true;
            }
        }
        let code = &code[node.start_byte()..node.end_byte()];
        code.starts_with(b"/// cbindgen:")
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.kind(),
            "source_file"
                | "function_item"
                | "impl_item"
                | "trait_item"
                | "closure_expression"
        )
    }

    fn is_func(node: &Node) -> bool {
        node.kind() == "function_item"
    }

    fn is_closure(node: &Node) -> bool {
        node.kind() == "closure_expression"
    }

    fn is_call(node: &Node) -> bool {
        node.kind() == "call_expression"
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(
            node.kind(),
            "(" | "," | ")" | "|" | "attribute_item"
        )
    }

    fn is_string(node: &Node) -> bool {
        matches!(node.kind(), "string_literal" | "raw_string_literal")
    }

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.kind() != "if_expression" {
            return false;
        }
        if let Some(parent) = node.parent() {
            return parent.kind() == "else_clause";
        }
        false
    }

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Rust::PrimitiveType
    }
}

impl Checker for KotlinCode {
    fn is_comment(_: &Node) -> bool {
        false
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(_: &Node) -> bool {
        false
    }

    fn is_func(_: &Node) -> bool {
        false
    }

    fn is_closure(_: &Node) -> bool {
        false
    }

    fn is_call(_: &Node) -> bool {
        false
    }

    fn is_non_arg(_: &Node) -> bool {
        false
    }

    fn is_string(_: &Node) -> bool {
        false
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

// BEAM languages - Full implementations

// Elixir implementation - based on tree-sitter-elixir 0.3.4
impl Checker for ElixirCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        // Module docs (@moduledoc) are useful
        false
    }

    fn is_func_space(node: &Node) -> bool {
        // Elixir function spaces: source file, do blocks (which contain functions)
        matches!(
            node.kind(),
            "source" | "do_block" | "anonymous_function"
        )
    }

    fn is_func(node: &Node) -> bool {
        // In Elixir, functions are identified by `def` and `defp` calls
        // These appear as Call nodes with identifier "def" or "defp"
        if node.kind() != "call" {
            return false;
        }
        // Check if first child is identifier matching a function-defining keyword
        node.child(0)
            .filter(|child| child.kind() == "identifier")
            .map(|child| node_text_equals_any(&child, &["def", "defp", "defmacro", "defmacrop"]))
            .unwrap_or(false)
    }

    fn is_closure(node: &Node) -> bool {
        node.kind() == "anonymous_function"
    }

    fn is_call(node: &Node) -> bool {
        node.kind() == "call"
    }

    fn is_non_arg(_node: &Node) -> bool {
        // Elixir uses parentheses, commas, etc. in arguments - but tree-sitter abstracts these
        // Look for node types that are structural but not actual arguments
        false
    }

    fn is_string(node: &Node) -> bool {
        matches!(
            node.kind(),
            "string" | "charlist" | "quoted_content"
        )
    }

    fn is_else_if(_node: &Node) -> bool {
        // Elixir doesn't have else-if syntax (uses cond instead)
        false
    }

    fn is_primitive(_id: u16) -> bool {
        // Elixir primitives: atoms, integers, floats, booleans, nil
        matches!(
            _id.into(),
            Elixir::Atom | Elixir::Integer | Elixir::Float | Elixir::Boolean | Elixir::Nil
        )
    }
}

// Erlang implementation - based on tree-sitter-erlang 0.15.0
impl Checker for ErlangCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        // No specific useful comments in Erlang (unlike Python's encoding comments)
        false
    }

    fn is_func_space(node: &Node) -> bool {
        // Erlang function spaces: source file, function declarations, anonymous functions
        matches!(
            node.kind(),
            "source_file"
                | "fun_decl"
                | "function_clause"
                | "anonymous_fun"
                | "clause_body"
        )
    }

    fn is_func(node: &Node) -> bool {
        // Erlang function declarations
        matches!(
            node.kind(),
            "fun_decl" | "function_clause"
        )
    }

    fn is_closure(node: &Node) -> bool {
        // Erlang anonymous functions (fun ... end)
        node.kind() == "anonymous_fun"
    }

    fn is_call(node: &Node) -> bool {
        // Erlang function calls
        node.kind() == "call"
    }

    fn is_non_arg(_node: &Node) -> bool {
        // Structural elements that aren't actual arguments
        false
    }

    fn is_string(node: &Node) -> bool {
        // Erlang doesn't have a dedicated string node (strings are lists of chars)
        // But there might be char nodes
        node.kind() == "char"
    }

    fn is_else_if(_node: &Node) -> bool {
        // Erlang uses pattern matching in case expressions, no else-if
        false
    }

    fn is_primitive(_id: u16) -> bool {
        // Erlang primitives: atoms, integers, floats, vars
        matches!(_id.into(), Erlang::Atom | Erlang::Integer | Erlang::Float)
    }
}

// Gleam implementation - based on tree-sitter-gleam 1.0.0
impl Checker for GleamCode {
    fn is_comment(node: &Node) -> bool {
        matches!(
            node.kind(),
            "comment" | "module_comment" | "statement_comment"
        )
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        // Module comments might be useful
        false
    }

    fn is_func_space(node: &Node) -> bool {
        // Gleam function spaces: source file, functions, anonymous functions, blocks
        matches!(
            node.kind(),
            "source_file"
                | "function"
                | "anonymous_function"
                | "function_body"
                | "block"
        )
    }

    fn is_func(node: &Node) -> bool {
        // Gleam function declarations (pub fn or fn)
        node.kind() == "function"
    }

    fn is_closure(node: &Node) -> bool {
        // Gleam anonymous functions
        node.kind() == "anonymous_function"
    }

    fn is_call(node: &Node) -> bool {
        // Gleam function calls
        node.kind() == "function_call"
    }

    fn is_non_arg(_node: &Node) -> bool {
        // Structural elements that aren't arguments
        false
    }

    fn is_string(node: &Node) -> bool {
        matches!(node.kind(), "string" | "quoted_content")
    }

    fn is_else_if(_node: &Node) -> bool {
        // Gleam doesn't have else-if (uses case expressions)
        false
    }

    fn is_primitive(_id: u16) -> bool {
        // Gleam primitives: integers, floats
        matches!(_id.into(), Gleam::Integer | Gleam::Float)
    }
}

// Lua implementation - based on tree-sitter-lua 0.2.0
impl Checker for LuaCode {
    fn is_comment(node: &Node) -> bool {
        node.kind() == "comment"
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        // Lua function spaces: program (top-level), function declarations, function definitions
        matches!(
            node.kind(),
            "program" | "function_declaration" | "function_definition" | "function"
        )
    }

    fn is_func(node: &Node) -> bool {
        // Lua function declarations and definitions
        matches!(
            node.kind(),
            "function_declaration" | "function_definition"
        )
    }

    fn is_closure(node: &Node) -> bool {
        // Lua anonymous functions
        node.kind() == "function"
    }

    fn is_call(node: &Node) -> bool {
        // Lua function calls
        node.kind() == "function_call"
    }

    fn is_non_arg(_node: &Node) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool {
        node.kind() == "string"
    }

    fn is_else_if(_node: &Node) -> bool {
        // Lua doesn't have else-if as a separate construct (uses elseif keyword within if)
        false
    }

    fn is_primitive(_id: u16) -> bool {
        // Lua primitives: numbers, strings, booleans, nil
        matches!(
            _id.into(),
            Lua::Number | Lua::String | Lua::True | Lua::False | Lua::Nil
        )
    }
}

// Go language - delegate to Java as fallback
impl Checker for GoCode {
    fn is_comment(node: &Node) -> bool {
        JavaCode::is_comment(node)
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        JavaCode::is_useful_comment(node, code)
    }

    fn is_func_space(node: &Node) -> bool {
        JavaCode::is_func_space(node)
    }

    fn is_func(node: &Node) -> bool {
        JavaCode::is_func(node)
    }

    fn is_closure(node: &Node) -> bool {
        JavaCode::is_closure(node)
    }

    fn is_call(node: &Node) -> bool {
        JavaCode::is_call(node)
    }

    fn is_non_arg(node: &Node) -> bool {
        JavaCode::is_non_arg(node)
    }

    fn is_string(node: &Node) -> bool {
        JavaCode::is_string(node)
    }

    fn is_else_if(node: &Node) -> bool {
        JavaCode::is_else_if(node)
    }

    fn is_primitive(id: u16) -> bool {
        JavaCode::is_primitive(id)
    }
}

// C# language - delegate to Java as fallback
impl Checker for CsharpCode {
    fn is_comment(node: &Node) -> bool {
        JavaCode::is_comment(node)
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        JavaCode::is_useful_comment(node, code)
    }

    fn is_func_space(node: &Node) -> bool {
        JavaCode::is_func_space(node)
    }

    fn is_func(node: &Node) -> bool {
        JavaCode::is_func(node)
    }

    fn is_closure(node: &Node) -> bool {
        JavaCode::is_closure(node)
    }

    fn is_call(node: &Node) -> bool {
        JavaCode::is_call(node)
    }

    fn is_non_arg(node: &Node) -> bool {
        JavaCode::is_non_arg(node)
    }

    fn is_string(node: &Node) -> bool {
        JavaCode::is_string(node)
    }

    fn is_else_if(node: &Node) -> bool {
        JavaCode::is_else_if(node)
    }

    fn is_primitive(id: u16) -> bool {
        JavaCode::is_primitive(id)
    }
}
