use crate::{
    analysis_context::{node_text, with_current_code},
    metrics::halstead::HalsteadType,
    spaces::SpaceKind,
    traits::Search,
    CcommentCode, Cpp, CppCode, CsharpCode, ElixirCode, ErlangCode, GleamCode, GoCode, Java,
    JavaCode, Javascript, JavascriptCode, KotlinCode, LuaCode, MozjsCode, Node, PreprocCode,
    Python, PythonCode, Rust, RustCode, Tsx, TsxCode, Typescript, TypescriptCode,
};

macro_rules! get_operator {
    ($language:ident) => {
        #[inline]
        fn get_operator_id_as_str(id: u16) -> &'static str {
            let typ = id.into();
            match typ {
                $language::LPAREN => "()",
                $language::LBRACK => "[]",
                $language::LBRACE => "{}",
                _ => typ.into(),
            }
        }
    };
}

pub trait Getter {
    #[must_use]
    fn get_func_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        Self::get_func_space_name(node, code)
    }

    #[must_use]
    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        // we're in a function or in a class
        node.child_by_field_name("name")
            .map_or(Some("<anonymous>"), |name| {
                let code = &code[name.start_byte()..name.end_byte()];
                std::str::from_utf8(code).ok()
            })
    }

    #[must_use]
    fn get_space_kind(_node: &Node) -> SpaceKind {
        SpaceKind::Unknown
    }

    #[must_use]
    fn get_op_type(_node: &Node) -> HalsteadType {
        HalsteadType::Unknown
    }

    #[must_use]
    fn get_operator_id_as_str(_id: u16) -> &'static str {
        ""
    }
}

impl Getter for PythonCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "function_definition" => SpaceKind::Function,
            "class_definition" => SpaceKind::Class,
            "module" => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            "import_statement"
            | "import_from_statement"
            | "."
            | ","
            | "as"
            | "*"
            | ">>"
            | "assert_statement"
            | ":="
            | "return_statement"
            | "def"
            | "del_statement"
            | "raise_statement"
            | "pass_statement"
            | "break_statement"
            | "continue_statement"
            | "if_statement"
            | "elif_clause"
            | "else_clause"
            | "async"
            | "for_statement"
            | "in"
            | "while_statement"
            | "try_statement"
            | "except_clause"
            | "finally_clause"
            | "with_statement"
            | "->"
            | "="
            | "global_statement"
            | "exec_statement"
            | "@"
            | "not"
            | "and"
            | "or"
            | "+"
            | "-"
            | "/"
            | "%"
            | "//"
            | "**"
            | "|"
            | "&"
            | "^"
            | "<<"
            | "~"
            | "<"
            | "<="
            | "=="
            | "!="
            | ">="
            | ">"
            | "<>"
            | "is"
            | "+="
            | "-="
            | "*="
            | "/="
            | "@="
            | "//="
            | "%="
            | "**="
            | ">>="
            | "<<="
            | "&="
            | "^="
            | "|="
            | "yield"
            | "await"
            | "print_statement" => HalsteadType::Operator,
            "identifier" | "integer" | "float" | "true" | "false" | "none" => HalsteadType::Operand,
            "string" => {
                let mut operator = HalsteadType::Unknown;
                // check if we've a documentation string or a multiline comment
                if let Some(parent) = node.parent() {
                    if parent.kind() != "expression_statement" || parent.child_count() != 1 {
                        operator = HalsteadType::Operand;
                    }
                }
                operator
            }
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        Into::<Python>::into(id).into()
    }
}

// Singularity custom parsers - delegate to standard parsers for compatibility
impl Getter for MozjsCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        JavascriptCode::get_space_kind(node)
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        JavascriptCode::get_func_space_name(node, code)
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        JavascriptCode::get_op_type(node)
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        JavascriptCode::get_operator_id_as_str(id)
    }
}

impl Getter for JavascriptCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "function_expression"
            | "method_definition"
            | "generator_function"
            | "function_declaration"
            | "generator_function_declaration"
            | "arrow_function" => SpaceKind::Function,
            "class" | "class_declaration" => SpaceKind::Class,
            "program" => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind() {
                    "pair" => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    "variable_declarator" => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            "export"
            | "import"
            | "extends"
            | "."
            | "from"
            | "("
            | ","
            | "as"
            | "*"
            | ">>"
            | ">>>"
            | ":"
            | "return"
            | "delete"
            | "throw"
            | "break"
            | "continue"
            | "if"
            | "else"
            | "switch"
            | "case"
            | "default"
            | "async"
            | "for"
            | "in"
            | "of"
            | "while"
            | "try"
            | "catch"
            | "finally"
            | "with"
            | "="
            | "@"
            | "&&"
            | "||"
            | "+"
            | "-"
            | "--"
            | "++"
            | "/"
            | "%"
            | "**"
            | "|"
            | "&"
            | "<<"
            | "~"
            | "<"
            | "<="
            | "=="
            | "!="
            | ">="
            | ">"
            | "+="
            | "!"
            | "!=="
            | "==="
            | "-="
            | "*="
            | "/="
            | "%="
            | "**="
            | ">>="
            | ">>>="
            | "<<="
            | "&="
            | "^"
            | "^="
            | "|="
            | "yield"
            | "["
            | "{"
            | "await"
            | "?"
            | "??"
            | "new"
            | "let"
            | "var"
            | "const"
            | "function"
            | "function_expression"
            | ";" => HalsteadType::Operator,
            "identifier"
            | "member_expression"
            | "property_identifier"
            | "string"
            | "number"
            | "true"
            | "false"
            | "null"
            | "void"
            | "this"
            | "super"
            | "undefined"
            | "set"
            | "get"
            | "typeof"
            | "instanceof" => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Javascript);
}

impl Getter for TypescriptCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "function_expression"
            | "method_definition"
            | "generator_function"
            | "function_declaration"
            | "generator_function_declaration"
            | "arrow_function" => SpaceKind::Function,
            "class" | "class_declaration" => SpaceKind::Class,
            "interface_declaration" => SpaceKind::Interface,
            "program" => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind() {
                    "pair" => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    "variable_declarator" => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            "export"
            | "import"
            | "extends"
            | "."
            | "from"
            | "("
            | ","
            | "as"
            | "*"
            | ">>"
            | ">>>"
            | ":"
            | "return"
            | "delete"
            | "throw"
            | "break"
            | "continue"
            | "if"
            | "else"
            | "switch"
            | "case"
            | "default"
            | "async"
            | "for"
            | "in"
            | "of"
            | "while"
            | "try"
            | "catch"
            | "finally"
            | "with"
            | "="
            | "@"
            | "&&"
            | "||"
            | "+"
            | "-"
            | "--"
            | "++"
            | "/"
            | "%"
            | "**"
            | "|"
            | "&"
            | "<<"
            | "~"
            | "<"
            | "<="
            | "=="
            | "!="
            | ">="
            | ">"
            | "+="
            | "!"
            | "!=="
            | "==="
            | "-="
            | "*="
            | "/="
            | "%="
            | "**="
            | ">>="
            | ">>>="
            | "<<="
            | "&="
            | "^"
            | "^="
            | "|="
            | "yield"
            | "["
            | "{"
            | "await"
            | "?"
            | "??"
            | "new"
            | "let"
            | "var"
            | "const"
            | "function"
            | "function_expression"
            | ";"
            | "predefined_type"
            | "type_identifier" => HalsteadType::Operator,
            "identifier" | "nested_identifier" | "member_expression" | "property_identifier" => {
                // Check if this identifier is part of a type annotation
                if let Some(parent) = node.parent() {
                    match parent.kind() {
                        "type_annotation" | "predefined_type" | "type_identifier"
                        | "generic_type" | "type_arguments" => {
                            return HalsteadType::Unknown;
                        }
                        _ => {}
                    }
                }
                HalsteadType::Operand
            }
            "string" | "number" | "true" | "false" | "null" | "void" | "this" | "super"
            | "undefined" | "set" | "get" | "typeof" | "instanceof" => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Typescript);
}

impl Getter for TsxCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "function_expression"
            | "method_definition"
            | "generator_function"
            | "function_declaration"
            | "generator_function_declaration"
            | "arrow_function" => SpaceKind::Function,
            "class" | "class_declaration" => SpaceKind::Class,
            "interface_declaration" => SpaceKind::Interface,
            "program" => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind() {
                    "pair" => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    "variable_declarator" => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            "export"
            | "import"
            | "extends"
            | "."
            | "from"
            | "("
            | ","
            | "as"
            | "*"
            | ">>"
            | ">>>"
            | ":"
            | "return"
            | "delete"
            | "throw"
            | "break"
            | "continue"
            | "if"
            | "else"
            | "switch"
            | "case"
            | "default"
            | "async"
            | "for"
            | "in"
            | "of"
            | "while"
            | "try"
            | "catch"
            | "finally"
            | "with"
            | "="
            | "@"
            | "&&"
            | "||"
            | "+"
            | "-"
            | "--"
            | "++"
            | "/"
            | "%"
            | "**"
            | "|"
            | "&"
            | "<<"
            | "~"
            | "<"
            | "<="
            | "=="
            | "!="
            | ">="
            | ">"
            | "+="
            | "!"
            | "!=="
            | "==="
            | "-="
            | "*="
            | "/="
            | "%="
            | "**="
            | ">>="
            | ">>>="
            | "<<="
            | "&="
            | "^"
            | "^="
            | "|="
            | "yield"
            | "["
            | "{"
            | "await"
            | "?"
            | "??"
            | "new"
            | "let"
            | "var"
            | "const"
            | "function"
            | "function_expression"
            | ";"
            | "predefined_type"
            | "type_identifier" => HalsteadType::Operator,
            "identifier" | "nested_identifier" | "member_expression" | "property_identifier" => {
                // Check if this identifier is part of a type annotation
                if let Some(parent) = node.parent() {
                    match parent.kind() {
                        "type_annotation" | "predefined_type" | "type_identifier"
                        | "generic_type" | "type_arguments" => {
                            return HalsteadType::Unknown;
                        }
                        _ => {}
                    }
                }
                HalsteadType::Operand
            }
            "string" | "number" | "true" | "false" | "null" | "void" | "this" | "super"
            | "undefined" | "set" | "get" | "typeof" | "instanceof" => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Tsx);
}

impl Getter for RustCode {
    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        // we're in a function or in a class or an impl
        // for an impl: we've  'impl ... type {...'
        node.child_by_field_name("name")
            .or_else(|| node.child_by_field_name("type"))
            .map_or(Some("<anonymous>"), |name| {
                let code = &code[name.start_byte()..name.end_byte()];
                std::str::from_utf8(code).ok()
            })
    }

    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "function_item" | "closure_expression" => SpaceKind::Function,
            "trait_item" => SpaceKind::Trait,
            "impl_item" => SpaceKind::Impl,
            "source_file" => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            // `||` is treated as an operator only if it's part of a binary expression.
            // This prevents misclassification inside macros where closures without arguments (e.g., `let closure = || { /* ... */ };`)
            // are not recognized as `ClosureExpression` and their `||` node is identified as `||` instead of `ClosureParameters`.
            //
            // Similarly, exclude `/` when it corresponds to the third slash in `///` (`OuterDocCommentMarker`)
            "||" | "/" => match node.parent() {
                Some(parent) if parent.kind() == "binary_expression" => HalsteadType::Operator,
                _ => HalsteadType::Unknown,
            },
            // Ensure `!` is counted as an operator unless it belongs to an `InnerDocCommentMarker` `//!`
            "!" => match node.parent() {
                Some(parent) if parent.kind() != "inner_doc_comment_marker" => {
                    HalsteadType::Operator
                }
                _ => HalsteadType::Unknown,
            },
            "(" | "{" | "[" | "=>" | "+" | "*" | "async" | "await" | "continue" | "for" | "if"
            | "let" | "loop" | "match" | "return" | "unsafe" | "while" | "=" | "," | "->" | "?"
            | "<" | ">" | "&" | "mutable_specifier" | ".." | "..=" | "-" | "&&" | "|" | "^"
            | "==" | "!=" | "<=" | ">=" | "<<" | ">>" | "%" | "+=" | "-=" | "*=" | "/=" | "%="
            | "&=" | "|=" | "^=" | "<<=" | ">>=" | "move" | "." | "primitive_type" | "fn" | ";" => {
                HalsteadType::Operator
            }
            "identifier" | "string_literal" | "raw_string_literal" | "integer_literal"
            | "float_literal" | "boolean_literal" | "self" | "char_literal" | "_" => {
                HalsteadType::Operand
            }
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Rust);
}

impl Getter for CppCode {
    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        match node.kind() {
            "function_definition" => {
                if let Some(op_cast) =
                    node.first_child_kind(|child| child.kind() == "operator_cast")
                {
                    let code = &code[op_cast.start_byte()..op_cast.end_byte()];
                    return std::str::from_utf8(code).ok();
                }
                // we're in a function_definition so need to get the declarator
                if let Some(declarator) = node.child_by_field_name("declarator") {
                    let declarator_node = declarator;
                    if let Some(fd) = declarator_node
                        .first_occurrence_kind(|child| child.kind() == "function_declarator")
                    {
                        if let Some(first) = fd.child(0) {
                            match first.kind() {
                                "type_identifier"
                                | "identifier"
                                | "field_identifier"
                                | "destructor_name"
                                | "operator_name"
                                | "qualified_identifier"
                                | "template_function"
                                | "template_method" => {
                                    let code = &code[first.start_byte()..first.end_byte()];
                                    return std::str::from_utf8(code).ok();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {
                if let Some(name) = node.child_by_field_name("name") {
                    let code = &code[name.start_byte()..name.end_byte()];
                    return std::str::from_utf8(code).ok();
                }
            }
        }
        None
    }

    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "function_definition" => SpaceKind::Function,
            "struct_specifier" => SpaceKind::Struct,
            "class_specifier" => SpaceKind::Class,
            "namespace_definition" => SpaceKind::Namespace,
            "translation_unit" => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Cpp::{
            Break, Case, Catch, Continue, Default, Delete, Do, Else, False, FieldIdentifier, For,
            Goto, Identifier, If, NamespaceDefinition, NamespaceIdentifier, New, Null,
            NumberLiteral, PrimitiveType, RawStringLiteral, Return, Sizeof, StringLiteral, Switch,
            Throw, True, Try, Try2, TypeIdentifier, TypeSpecifier, While, AMP, AMPAMP, AMPEQ, BANG,
            BANGEQ, CARET, CARETEQ, COLON, COLONCOLON, COMMA, DASH, DASHDASH, DASHGT, DOT,
            DOTDOTDOT, EQ, EQEQ, GT, GT2, GTEQ, GTGT, GTGTEQ, LBRACE, LBRACK, LPAREN, LPAREN2, LT,
            LTEQ, LTLT, LTLTEQ, PERCENT, PERCENTEQ, PIPE, PIPEEQ, PIPEPIPE, PLUS, PLUSEQ, PLUSPLUS,
            QMARK, SEMI, SLASH, SLASHEQ, STAR, STAREQ, TILDE,
        };

        match node.kind_id().into() {
            DOT | LPAREN | LPAREN2 | COMMA | STAR | GTGT | COLON | SEMI | Return | Break
            | Continue | If | Else | Switch | Case | Default | For | While | Goto | Do | Delete
            | New | Try | Try2 | Catch | Throw | EQ | AMPAMP | PIPEPIPE | DASH | DASHDASH
            | DASHGT | PLUS | PLUSPLUS | SLASH | PERCENT | PIPE | AMP | LTLT | TILDE | LT
            | LTEQ | EQEQ | BANGEQ | GTEQ | GT | GT2 | PLUSEQ | BANG | STAREQ | SLASHEQ
            | PERCENTEQ | GTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ | LBRACK | LBRACE
            | QMARK | COLONCOLON | PrimitiveType | TypeSpecifier | Sizeof => HalsteadType::Operator,
            Identifier | TypeIdentifier | FieldIdentifier | RawStringLiteral | StringLiteral
            | NumberLiteral | True | False | Null | DOTDOTDOT => HalsteadType::Operand,
            NamespaceIdentifier => match node.parent() {
                Some(parent) if matches!(parent.kind_id().into(), NamespaceDefinition) => {
                    HalsteadType::Operand
                }
                _ => HalsteadType::Unknown,
            },
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Cpp);
}

// Singularity custom parsers - delegate to standard C/C++ parser for compatibility
impl Getter for PreprocCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        CppCode::get_space_kind(node)
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        CppCode::get_func_space_name(node, code)
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        CppCode::get_op_type(node)
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        CppCode::get_operator_id_as_str(id)
    }
}

impl Getter for CcommentCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        CppCode::get_space_kind(node)
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        CppCode::get_func_space_name(node, code)
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        CppCode::get_op_type(node)
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        CppCode::get_operator_id_as_str(id)
    }
}

impl Getter for JavaCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "class_declaration" => SpaceKind::Class,
            "method_declaration" | "constructor_declaration" | "lambda_expression" => {
                SpaceKind::Function
            }
            "interface_declaration" => SpaceKind::Interface,
            "program" => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            // Operator: control flow
            "if" | "else" | "switch" | "case" | "try" | "catch" | "throw" | "throws"
            | "for" | "while" | "continue" | "break" | "do" | "finally"
            // Operator: keywords
            | "new" | "return" | "default" | "abstract" | "assert" | "instanceof"
            | "extends" | "final" | "implements" | "transient" | "synchronized"
            | "super" | "this" | "void_type"
            // Operator: brackets and comma and terminators (separators)
            | ";" | "," | "::" | "{" | "[" | "("
            // Operator: operators
            | "=" | "<" | ">" | "!" | "~" | "?" | ":"
            | "==" | "<=" | ">=" | "!=" | "&&" | "||" | "++" | "--"
            | "+" | "-" | "*" | "/" | "&" | "|" | "^" | "%" | "<<" | ">>" | ">>>"
            | "+=" | "-=" | "*=" | "/=" | "&=" | "|=" | "^=" | "%=" | "<<=" | ">>=" | ">>>="
            // primitive types
            | "int" | "float"
            => HalsteadType::Operator,
            // Operands: variables, constants, literals
            "identifier" | "null_literal" | "class_literal" | "string_literal"
            | "character_literal" | "hex_integer_literal" | "octal_integer_literal"
            | "binary_integer_literal" | "decimal_integer_literal"
            | "hex_floating_point_literal" | "decimal_floating_point_literal"
            => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let kind_str: &str = Java::from(id).into();
        match kind_str {
            "(" => "()",
            "[" => "[]",
            "{" => "{}",
            "void_type" => "void",
            _ => kind_str,
        }
    }
}

impl Getter for KotlinCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "source_file" => SpaceKind::Unit,
            "class_declaration" => SpaceKind::Class,
            "function_declaration" | "lambda_literal" | "anonymous_function" => SpaceKind::Function,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            // Keywords and control flow
            "if" | "else" | "when" | "for" | "while" | "do" | "return" | "break" | "continue"
            | "throw" | "try" | "catch" | "finally" | "class" | "fun" | "val" | "var"
            | "in" | "is" | "as" | "object" | "companion" | "init" | "this" | "super"
            // Operators
            | "=" | "+" | "-" | "*" | "/" | "%" | "++" | "--" | "==" | "!=" | "<" | ">"
            | "<=" | ">=" | "&&" | "||" | "!" | "&" | "|" | "^" | "<<" | ">>" | ">>>"
            | "+=" | "-=" | "*=" | "/=" | "%=" | ".." | "?:" | "?." | "!!" | "::"
            // Delimiters
            | "(" | "[" | "{" | "," | ";" | "." | "->" | "=>"
            => HalsteadType::Operator,
            // Operands
            "identifier" | "string_literal" | "multiline_string_literal" | "integer_literal"
            | "real_literal" | "boolean_literal" | "character_literal" | "null_literal"
            => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let language: tree_sitter::Language = tree_sitter_kotlin_ng::LANGUAGE.into();
        match language.node_kind_for_id(id) {
            Some("(") => "()",
            Some("[") => "[]",
            Some("{") => "{}",
            Some(kind) => kind,
            None => "unknown",
        }
    }
}

// BEAM languages - Elixir, Erlang, Gleam (full implementations)
impl Getter for ElixirCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "source" => SpaceKind::Unit,
            "anonymous_function" => SpaceKind::Function,
            "do_block" => {
                if let Some(parent) = node.parent() {
                    if parent.kind() == "call" {
                        if let Some(head) = parent.child(0) {
                            if head.kind() == "identifier" {
                                // Determine whether this do-block belongs to a module or a function
                                return with_keyword(&head, |kw| {
                                    if matches!(kw, "defmodule" | "defprotocol" | "defimpl") {
                                        SpaceKind::Unit
                                    } else if matches!(
                                        kw,
                                        "def" | "defp" | "defmacro" | "defmacrop"
                                    ) {
                                        SpaceKind::Function
                                    } else {
                                        SpaceKind::Unknown
                                    }
                                })
                                .unwrap_or(SpaceKind::Unknown);
                            }
                        }
                    }
                }
                SpaceKind::Unknown
            }
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        match node.kind() {
            "do_block" => {
                let parent = node.parent()?;
                if parent.kind() != "call" {
                    return None;
                }

                let keyword_node = parent.child(0)?;
                if keyword_node.kind() != "identifier" {
                    return None;
                }

                let keyword = node_text(&keyword_node, code)?;
                let arguments = (0..parent.child_count())
                    .filter_map(|idx| parent.child(idx))
                    .find(|child| child.is_named() && child.kind() == "arguments")?;

                match keyword {
                    "defmodule" | "defprotocol" | "defimpl" => {
                        extract_first_named_text(&arguments, code)
                    }
                    "def" | "defp" | "defmacro" | "defmacrop" => {
                        extract_function_head_name(&arguments, code)
                    }
                    _ => default_space_name(node, code),
                }
            }
            "anonymous_function" => Some("anonymous_function"),
            _ => default_space_name(node, code),
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            "binary_operator"
            | "unary_operator"
            | "operator_identifier"
            | "dot"
            | "call"
            | "arguments" => HalsteadType::Operator,
            "identifier" | "alias" | "atom" | "quoted_atom" | "integer" | "string" | "charlist"
            | "sigil" | "list" | "tuple" | "map" | "struct" | "keywords" | "anonymous_function" => {
                HalsteadType::Operand
            }
            "+" | "-" | "*" | "/" | "%" | "++" | "--" | "::" | "->" | "<-" | "<>" | "||" | "&&"
            | "===" | "==" | "!==" | "!=" | "<" | "<=" | ">" | ">=" | "in" | "when" | "and"
            | "or" | "not" | "xor" | "<<<" | ">>>" | "^^^" | "~~~" | "&&&" | "|||" | "." | "if"
            | "unless" | "case" | "fn" | "do" | "after" | "rescue" | "catch" | "else" => {
                HalsteadType::Operator
            }
            "nil" | "true" | "false" => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let language: tree_sitter::Language = tree_sitter_elixir::LANGUAGE.into();
        match language.node_kind_for_id(id) {
            Some("(") => "()",
            Some("[") => "[]",
            Some("{") => "{}",
            Some(kind) => kind,
            None => "unknown",
        }
    }
}

fn extract_first_named_text<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
    for idx in 0..node.child_count() {
        if let Some(child) = node.child(idx) {
            if !child.is_named() {
                continue;
            }
            if matches!(
                child.kind(),
                "alias" | "atom" | "identifier" | "quoted_atom"
            ) {
                return node_text(&child, code);
            }
        }
    }
    None
}

fn extract_function_head_name<'a>(arguments: &Node, code: &'a [u8]) -> Option<&'a str> {
    for idx in 0..arguments.child_count() {
        if let Some(child) = arguments.child(idx) {
            if !child.is_named() {
                continue;
            }
            match child.kind() {
                "call" => {
                    for call_idx in 0..child.child_count() {
                        if let Some(name_node) = child.child(call_idx) {
                            if !name_node.is_named() {
                                continue;
                            }
                            if matches!(name_node.kind(), "identifier" | "atom" | "quoted_atom") {
                                return node_text(&name_node, code);
                            }
                            break;
                        }
                    }
                }
                "identifier" | "atom" | "quoted_atom" => return node_text(&child, code),
                _ => {}
            }
        }
    }
    None
}

fn with_keyword<F>(identifier: &Node, f: F) -> Option<SpaceKind>
where
    F: FnOnce(&str) -> SpaceKind,
{
    with_current_code(|code| {
        let text = &code[identifier.start_byte()..identifier.end_byte()];
        std::str::from_utf8(text).ok().map(f)
    })
    .flatten()
}

fn default_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
    node.child_by_field_name("name")
        .and_then(|name| node_text(&name, code))
        .or(Some("<anonymous>"))
}
impl Getter for ErlangCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "source_file" => SpaceKind::Unit,
            "fun_decl" | "function_clause" | "anonymous_fun" => SpaceKind::Function,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        match node.kind() {
            "fun_decl" | "function_clause" => {
                // Erlang functions are typically named with atoms
                node.child_by_field_name("name")
                    .and_then(|name| node_text(&name, code))
            }
            "anonymous_fun" => Some("anonymous_function"),
            _ => default_space_name(node, code),
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            "binary_op_expr" | "unary_op_expr" | "match_expr" | "catch_expr" | "+" | "-" | "*"
            | "/" | "%" | "div" | "rem" | "band" | "bor" | "bxor" | "bsl" | "bsr" | "and"
            | "or" | "not" | "xor" | "orelse" | "andalso" | "==" | "/=" | "=:= " | "=/=" | "<"
            | "<=" | ">" | ">=" | "++" | "--" | "!" | "catch" | "of" | "after" => {
                HalsteadType::Operator
            }
            "atom" | "var" | "list" | "tuple" | "map_expr" => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let language: tree_sitter::Language = tree_sitter_erlang::LANGUAGE.into();
        match language.node_kind_for_id(id) {
            Some("(") => "()",
            Some("[") => "[]",
            Some("{") => "{}",
            Some(kind) => kind,
            None => "unknown",
        }
    }
}

impl Getter for GleamCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "source_file" => SpaceKind::Unit,
            "function" | "anonymous_function" => SpaceKind::Function,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        match node.kind() {
            "function" => node
                .child_by_field_name("name")
                .and_then(|name| node_text(&name, code)),
            "anonymous_function" => Some("anonymous_function"),
            _ => default_space_name(node, code),
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            "binary_expression" | "boolean_negation" | "integer_negation" | "pipeline_echo"
            | "case" | "let" | "+" | "-" | "*" | "/" | "%" | "++" | "--" | "<" | "<=" | ">"
            | ">=" | "==" | "!=" | "&&" | "||" | "<-" | "->" | "if" | "else" => {
                HalsteadType::Operator
            }
            "identifier" | "integer" | "float" | "string" | "comment" => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let language: tree_sitter::Language = tree_sitter_gleam::LANGUAGE.into();
        match language.node_kind_for_id(id) {
            Some("(") => "()",
            Some("[") => "[]",
            Some("{") => "{}",
            Some(kind) => kind,
            None => "unknown",
        }
    }
}

// Lua implementation
impl Getter for LuaCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "program" => SpaceKind::Unit,
            "function_declaration" | "function_definition" | "function" => SpaceKind::Function,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            // Keywords and control flow
            "if" | "then" | "else" | "elseif" | "end" | "for" | "while" | "do" | "repeat"
            | "until" | "return" | "break" | "goto" | "in" | "local" | "function"
            | "and" | "or" | "not"
            // Operators
            | "=" | "+" | "-" | "*" | "/" | "%" | "^" | "#" | "==" | "~=" | "<" | ">"
            | "<=" | ">=" | ".." | "." | ":"
            // Delimiters
            | "(" | "[" | "{" | "," | ";"
            => HalsteadType::Operator,
            // Operands
            "identifier" | "string" | "number" | "nil" | "true" | "false"
            => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let language: tree_sitter::Language = tree_sitter_lua::LANGUAGE.into();
        match language.node_kind_for_id(id) {
            Some("(") => "()",
            Some("[") => "[]",
            Some("{") => "{}",
            Some(kind) => kind,
            None => "unknown",
        }
    }
}

// Go implementation
impl Getter for GoCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "source_file" => SpaceKind::Unit,
            "function_declaration" | "method_declaration" | "func_literal" => SpaceKind::Function,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            // Keywords and control flow
            "if" | "else" | "for" | "switch" | "case" | "default" | "return" | "break"
            | "continue" | "goto" | "fallthrough" | "select" | "defer" | "go" | "type"
            | "struct" | "interface" | "map" | "chan" | "func" | "var" | "const" | "package"
            | "import" | "range"
            // Operators
            | "=" | "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>" | "&^"
            | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | "&^="
            | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" | "!" | "<-" | "++" | "--"
            | ":=" | "..."
            // Delimiters
            | "(" | "[" | "{" | "," | ";" | "." | ":"
            => HalsteadType::Operator,
            // Operands
            "identifier" | "interpreted_string_literal" | "raw_string_literal"
            | "int_literal" | "float_literal" | "imaginary_literal" | "rune_literal"
            | "nil" | "true" | "false" | "iota"
            => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let language: tree_sitter::Language = tree_sitter_go::LANGUAGE.into();
        match language.node_kind_for_id(id) {
            Some("(") => "()",
            Some("[") => "[]",
            Some("{") => "{}",
            Some(kind) => kind,
            None => "unknown",
        }
    }
}

// C# implementation
impl Getter for CsharpCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind() {
            "compilation_unit" => SpaceKind::Unit,
            "class_declaration" | "struct_declaration" | "record_declaration" => SpaceKind::Class,
            "interface_declaration" => SpaceKind::Interface,
            "method_declaration"
            | "constructor_declaration"
            | "lambda_expression"
            | "anonymous_method_expression" => SpaceKind::Function,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        match node.kind() {
            // Keywords and control flow
            "if" | "else" | "switch" | "case" | "default" | "for" | "foreach" | "while"
            | "do" | "return" | "break" | "continue" | "goto" | "throw" | "try" | "catch"
            | "finally" | "yield" | "await" | "async" | "lock" | "using" | "new" | "typeof"
            | "sizeof" | "nameof" | "is" | "as" | "var" | "class" | "struct" | "interface"
            | "enum" | "delegate" | "this" | "base" | "null" | "in" | "out" | "ref" | "params"
            // Operators
            | "=" | "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>" | ">>>"
            | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>=" | ">>>="
            | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||" | "!" | "~" | "++" | "--"
            | "??" | "?" | "?." | "=>" | "::"
            // Delimiters
            | "(" | "[" | "{" | "," | ";" | "." | ":" | "->"
            => HalsteadType::Operator,
            // Operands
            "identifier" | "string_literal" | "interpolated_string_expression"
            | "integer_literal" | "real_literal" | "character_literal" | "boolean_literal"
            | "null_literal" | "verbatim_string_literal"
            => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let language: tree_sitter::Language = tree_sitter_c_sharp::LANGUAGE.into();
        match language.node_kind_for_id(id) {
            Some("(") => "()",
            Some("[") => "[]",
            Some("{") => "{}",
            Some(kind) => kind,
            None => "unknown",
        }
    }
}
