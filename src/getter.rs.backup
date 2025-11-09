use crate::{
    analysis_context::{node_text, with_current_code},
    metrics::halstead::HalsteadType,
    spaces::SpaceKind,
    traits::Search,
    CcommentCode, Cpp, CppCode, CsharpCode, Elixir, ElixirCode, ErlangCode, GleamCode, GoCode,
    Java, JavaCode, Javascript, JavascriptCode, KotlinCode, LuaCode, Mozjs, MozjsCode, Node,
    PreprocCode, Python, PythonCode, Rust, RustCode, Tsx, TsxCode, Typescript, TypescriptCode,
};

macro_rules! get_operator {
    ($language:ident) => {
        #[inline(always)]
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
    fn get_func_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        Self::get_func_space_name(node, code)
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        // we're in a function or in a class
        node.child_by_field_name("name")
            .map_or(Some("<anonymous>"), |name| {
                let code = &code[name.start_byte()..name.end_byte()];
                std::str::from_utf8(code).ok()
            })
    }

    fn get_space_kind(_node: &Node) -> SpaceKind {
        SpaceKind::Unknown
    }

    fn get_op_type(_node: &Node) -> HalsteadType {
        HalsteadType::Unknown
    }

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
            "import_statement" | "import_from_statement" | "." | "," | "as" | "*" | ">>" 
            | "assert_statement" | ":=" | "return_statement" | "def" | "del_statement"
            | "raise_statement" | "pass_statement" | "break_statement" | "continue_statement"
            | "if_statement" | "elif_clause" | "else_clause" | "async" | "for_statement"
            | "in" | "while_statement" | "try_statement" | "except_clause" | "finally_clause"
            | "with_statement" | "->" | "=" | "global_statement" | "exec_statement" | "@"
            | "not" | "and" | "or" | "+" | "-" | "/" | "%" | "//" | "**" | "|" | "&"
            | "^" | "<<" | "~" | "<" | "<=" | "==" | "!=" | ">=" | ">" | "<>" | "is"
            | "+=" | "-=" | "*=" | "/=" | "@=" | "//=" | "%=" | "**=" | ">>=" | "<<="
            | "&=" | "^=" | "|=" | "yield" | "await" | "print_statement" => {
                HalsteadType::Operator
            }
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
        use Javascript::{
            ArrowFunction, Class, ClassDeclaration, FunctionDeclaration, FunctionExpression,
            GeneratorFunction, GeneratorFunctionDeclaration, MethodDefinition, Program,
        };

        match node.kind_id().into() {
            FunctionExpression
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => SpaceKind::Function,
            Class | ClassDeclaration => SpaceKind::Class,
            Program => SpaceKind::Unit,
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
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
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
        use Javascript::{
            As, Async, Await, Break, Case, Catch, Const, Continue, Default, Delete, Else, Export,
            Extends, False, Finally, For, From, Function, FunctionExpression, Get, Identifier,
            Identifier2, If, Import, Import2, In, Instanceof, Let, MemberExpression,
            MemberExpression2, New, Null, Number, Of, PropertyIdentifier, Return, Set, String,
            String2, Super, Switch, This, Throw, True, Try, Typeof, Undefined, Var, Void, While,
            With, Yield, AMP, AMPAMP, AMPEQ, AT, BANG, BANGEQ, BANGEQEQ, CARET, CARETEQ, COLON,
            COMMA, DASH, DASHDASH, DASHEQ, DOT, EQ, EQEQ, EQEQEQ, GT, GTEQ, GTGT, GTGTEQ, GTGTGT,
            GTGTGTEQ, LBRACE, LBRACK, LPAREN, LT, LTEQ, LTLT, LTLTEQ, PERCENT, PERCENTEQ, PIPE,
            PIPEEQ, PIPEPIPE, PLUS, PLUSEQ, PLUSPLUS, QMARK, QMARKQMARK, SEMI, SLASH, SLASHEQ,
            STAR, STAREQ, STARSTAR, STARSTAREQ, TILDE,
        };

        match node.kind_id().into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const
            | Function | FunctionExpression | SEMI => HalsteadType::Operator,
            Identifier | Identifier2 | MemberExpression | MemberExpression2
            | PropertyIdentifier | String | String2 | Number | True | False | Null | Void
            | This | Super | Undefined | Set | Get | Typeof | Instanceof => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Javascript);
}

impl Getter for TypescriptCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        use Typescript::{
            ArrowFunction, Class, ClassDeclaration, FunctionDeclaration, FunctionExpression,
            GeneratorFunction, GeneratorFunctionDeclaration, InterfaceDeclaration,
            MethodDefinition, Program,
        };

        match node.kind_id().into() {
            FunctionExpression
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => SpaceKind::Function,
            Class | ClassDeclaration => SpaceKind::Class,
            InterfaceDeclaration => SpaceKind::Interface,
            Program => SpaceKind::Unit,
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
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
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
        use Typescript::{
            As, Async, Await, Break, Case, Catch, Const, Continue, Default, Delete, Else, Export,
            Extends, False, Finally, For, From, Function, FunctionExpression, Get, Identifier, If,
            Import, Import2, In, Instanceof, Let, MemberExpression, NestedIdentifier, New, Null,
            Number, Of, PropertyIdentifier, Return, Set, String, Super, Switch, This, Throw, True,
            Try, Typeof, Undefined, Var, Void, While, With, Yield, AMP, AMPAMP, AMPEQ, AT, BANG,
            BANGEQ, BANGEQEQ, CARET, CARETEQ, COLON, COMMA, DASH, DASHDASH, DASHEQ, DOT, EQ, EQEQ,
            EQEQEQ, GT, GTEQ, GTGT, GTGTEQ, GTGTGT, GTGTGTEQ, LBRACE, LBRACK, LPAREN, LT, LTEQ,
            LTLT, LTLTEQ, PERCENT, PERCENTEQ, PIPE, PIPEEQ, PIPEPIPE, PLUS, PLUSEQ, PLUSPLUS,
            QMARK, QMARKQMARK, SEMI, SLASH, SLASHEQ, STAR, STAREQ, STARSTAR, STARSTAREQ, TILDE,
        };

        match node.kind_id().into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const
            | Function | FunctionExpression | SEMI => HalsteadType::Operator,
            Identifier | NestedIdentifier | MemberExpression | PropertyIdentifier | String
            | Number | True | False | Null | Void | This | Super | Undefined | Set | Get
            | Typeof | Instanceof => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Typescript);
}

impl Getter for TsxCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        use Tsx::{
            ArrowFunction, Class, ClassDeclaration, FunctionDeclaration, FunctionExpression,
            GeneratorFunction, GeneratorFunctionDeclaration, InterfaceDeclaration,
            MethodDefinition, Program,
        };

        match node.kind_id().into() {
            FunctionExpression
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => SpaceKind::Function,
            Class | ClassDeclaration => SpaceKind::Class,
            InterfaceDeclaration => SpaceKind::Interface,
            Program => SpaceKind::Unit,
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
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
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
        use Tsx::{
            As, Async, Await, Break, Case, Catch, Const, Continue, Default, Delete, Else, Export,
            Extends, False, Finally, For, From, Function, FunctionExpression, Get, Identifier, If,
            Import, Import2, In, Instanceof, Let, MemberExpression, NestedIdentifier, New, Null,
            Number, Of, PropertyIdentifier, Return, Set, String, String2, Super, Switch, This,
            Throw, True, Try, Typeof, Undefined, Var, Void, While, With, Yield, AMP, AMPAMP, AMPEQ,
            AT, BANG, BANGEQ, BANGEQEQ, CARET, CARETEQ, COLON, COMMA, DASH, DASHDASH, DASHEQ, DOT,
            EQ, EQEQ, EQEQEQ, GT, GTEQ, GTGT, GTGTEQ, GTGTGT, GTGTGTEQ, LBRACE, LBRACK, LPAREN, LT,
            LTEQ, LTLT, LTLTEQ, PERCENT, PERCENTEQ, PIPE, PIPEEQ, PIPEPIPE, PLUS, PLUSEQ, PLUSPLUS,
            QMARK, QMARKQMARK, SEMI, SLASH, SLASHEQ, STAR, STAREQ, STARSTAR, STARSTAREQ, TILDE,
        };

        match node.kind_id().into() {
            Export | Import | Import2 | Extends | DOT | From | LPAREN | COMMA | As | STAR
            | GTGT | GTGTGT | COLON | Return | Delete | Throw | Break | Continue | If | Else
            | Switch | Case | Default | Async | For | In | Of | While | Try | Catch | Finally
            | With | EQ | AT | AMPAMP | PIPEPIPE | PLUS | DASH | DASHDASH | PLUSPLUS | SLASH
            | PERCENT | STARSTAR | PIPE | AMP | LTLT | TILDE | LT | LTEQ | EQEQ | BANGEQ | GTEQ
            | GT | PLUSEQ | BANG | BANGEQEQ | EQEQEQ | DASHEQ | STAREQ | SLASHEQ | PERCENTEQ
            | STARSTAREQ | GTGTEQ | GTGTGTEQ | LTLTEQ | AMPEQ | CARET | CARETEQ | PIPEEQ
            | Yield | LBRACK | LBRACE | Await | QMARK | QMARKQMARK | New | Let | Var | Const
            | Function | FunctionExpression | SEMI => HalsteadType::Operator,
            Identifier | NestedIdentifier | MemberExpression | PropertyIdentifier | String
            | String2 | Number | True | False | Null | Void | This | Super | Undefined | Set
            | Get | Typeof | Instanceof => HalsteadType::Operand,
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
        use Rust::{ClosureExpression, FunctionItem, ImplItem, SourceFile, TraitItem};

        match node.kind_id().into() {
            FunctionItem | ClosureExpression => SpaceKind::Function,
            TraitItem => SpaceKind::Trait,
            ImplItem => SpaceKind::Impl,
            SourceFile => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Rust::{
            Async, Await, BinaryExpression, BooleanLiteral, CharLiteral, Continue, FloatLiteral,
            Fn, For, Identifier, If, InnerDocCommentMarker, IntegerLiteral, Let, Loop, Match, Move,
            MutableSpecifier, PrimitiveType, RawStringLiteral, Return, StringLiteral, Unsafe,
            While, Zelf, AMP, AMPAMP, AMPEQ, BANG, BANGEQ, CARET, CARETEQ, COMMA, DASH, DASHEQ,
            DASHGT, DOT, DOTDOT, DOTDOTEQ, EQ, EQEQ, EQGT, GT, GTEQ, GTGT, GTGTEQ, LBRACE, LBRACK,
            LPAREN, LT, LTEQ, LTLT, LTLTEQ, PERCENT, PERCENTEQ, PIPE, PIPEEQ, PIPEPIPE, PLUS,
            PLUSEQ, QMARK, SEMI, SLASH, SLASHEQ, STAR, STAREQ, UNDERSCORE,
        };

        match node.kind_id().into() {
            // `||` is treated as an operator only if it's part of a binary expression.
            // This prevents misclassification inside macros where closures without arguments (e.g., `let closure = || { /* ... */ };`)
            // are not recognized as `ClosureExpression` and their `||` node is identified as `PIPEPIPE` instead of `ClosureParameters`.
            //
            // Similarly, exclude `/` when it corresponds to the third slash in `///` (`OuterDocCommentMarker`)
            PIPEPIPE | SLASH => match node.parent() {
                Some(parent) if matches!(parent.kind_id().into(), BinaryExpression) => {
                    HalsteadType::Operator
                }
                _ => HalsteadType::Unknown,
            },
            // Ensure `!` is counted as an operator unless it belongs to an `InnerDocCommentMarker` `//!`
            BANG => match node.parent() {
                Some(parent) if !matches!(parent.kind_id().into(), InnerDocCommentMarker) => {
                    HalsteadType::Operator
                }
                _ => HalsteadType::Unknown,
            },
            LPAREN | LBRACE | LBRACK | EQGT | PLUS | STAR | Async | Await | Continue | For | If
            | Let | Loop | Match | Return | Unsafe | While | EQ | COMMA | DASHGT | QMARK | LT
            | GT | AMP | MutableSpecifier | DOTDOT | DOTDOTEQ | DASH | AMPAMP | PIPE | CARET
            | EQEQ | BANGEQ | LTEQ | GTEQ | LTLT | GTGT | PERCENT | PLUSEQ | DASHEQ | STAREQ
            | SLASHEQ | PERCENTEQ | AMPEQ | PIPEEQ | CARETEQ | LTLTEQ | GTGTEQ | Move | DOT
            | PrimitiveType | Fn | SEMI => HalsteadType::Operator,
            Identifier | StringLiteral | RawStringLiteral | IntegerLiteral | FloatLiteral
            | BooleanLiteral | Zelf | CharLiteral | UNDERSCORE => HalsteadType::Operand,
            _ => HalsteadType::Unknown,
        }
    }

    get_operator!(Rust);
}

impl Getter for CppCode {
    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        match node.kind() {
            "function_definition" => {
                if let Some(op_cast) = node.first_child_kind(|child| child.kind() == "operator_cast") {
                    let code = &code[op_cast.start_byte()..op_cast.end_byte()];
                    return std::str::from_utf8(code).ok();
                }
                // we're in a function_definition so need to get the declarator
                if let Some(declarator) = node.child_by_field_name("declarator") {
                    let declarator_node = declarator;
                    if let Some(fd) = declarator_node.first_occurrence_kind(|child| {
                        child.kind() == "function_declarator"
                    }) {
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
        use Java::{
            ClassDeclaration, ConstructorDeclaration, InterfaceDeclaration, LambdaExpression,
            MethodDeclaration, Program,
        };

        match node.kind_id().into() {
            ClassDeclaration => SpaceKind::Class,
            MethodDeclaration | ConstructorDeclaration | LambdaExpression => SpaceKind::Function,
            InterfaceDeclaration => SpaceKind::Interface,
            Program => SpaceKind::Unit,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Java::{
            Abstract, Assert, BinaryIntegerLiteral, Break, Case, Catch, CharacterLiteral,
            ClassLiteral, Continue, DecimalFloatingPointLiteral, DecimalIntegerLiteral, Default,
            Do, Else, Extends, Final, Finally, Float, For, HexFloatingPointLiteral,
            HexIntegerLiteral, Identifier, If, Implements, Instanceof, Int, New, NullLiteral,
            OctalIntegerLiteral, Return, StringLiteral, Super, Switch, Synchronized, This, Throw,
            Throws, Throws2, Transient, Try, VoidType, While, AMP, AMPAMP, AMPEQ, BANG, BANGEQ,
            CARET, CARETEQ, COLON, COLONCOLON, COMMA, DASH, DASHDASH, DASHEQ, EQ, EQEQ, GT, GTEQ,
            GTGT, GTGTEQ, GTGTGT, GTGTGTEQ, LBRACE, LBRACK, LPAREN, LT, LTEQ, LTLT, LTLTEQ,
            PERCENT, PERCENTEQ, PIPE, PIPEEQ, PIPEPIPE, PLUS, PLUSEQ, PLUSPLUS, QMARK, SEMI, SLASH,
            SLASHEQ, STAR, STAREQ, TILDE,
        };
        // Some guides that informed grammar choice for Halstead
        // keywords, operators, literals: https://docs.oracle.com/javase/specs/jls/se18/html/jls-3.html#jls-3.12
        // https://www.geeksforgeeks.org/software-engineering-halsteads-software-metrics/?msclkid=5e181114abef11ecbb03527e95a34828
        match node.kind_id().into() {
            // Operator: control flow
            | If | Else | Switch | Case | Try | Catch | Throw | Throws | Throws2 | For | While | Continue | Break | Do | Finally
            // Operator: keywords
            | New | Return | Default | Abstract | Assert | Instanceof | Extends | Final | Implements | Transient | Synchronized | Super | This | VoidType
            // Operator: brackets and comma and terminators (separators)
            | SEMI | COMMA | COLONCOLON | LBRACE | LBRACK | LPAREN // | RBRACE | RBRACK | RPAREN | DOTDOTDOT | DOT
            // Operator: operators
            | EQ | LT | GT | BANG | TILDE | QMARK | COLON // no grammar for lambda operator ->
            | EQEQ | LTEQ | GTEQ | BANGEQ | AMPAMP | PIPEPIPE | PLUSPLUS | DASHDASH
            | PLUS | DASH | STAR | SLASH | AMP | PIPE | CARET | PERCENT| LTLT | GTGT | GTGTGT
            | PLUSEQ | DASHEQ | STAREQ | SLASHEQ | AMPEQ | PIPEEQ | CARETEQ | PERCENTEQ | LTLTEQ | GTGTEQ | GTGTGTEQ
            // primitive types
            | Int | Float
            => {
                HalsteadType::Operator
            },
            // Operands: variables, constants, literals
            Identifier | NullLiteral | ClassLiteral | StringLiteral | CharacterLiteral | HexIntegerLiteral | OctalIntegerLiteral | BinaryIntegerLiteral | DecimalIntegerLiteral | HexFloatingPointLiteral | DecimalFloatingPointLiteral  => {
                HalsteadType::Operand
            },
            _ => {
                HalsteadType::Unknown
            },
        }
    }

    fn get_operator_id_as_str(id: u16) -> &'static str {
        let typ = id.into();
        match typ {
            Java::LPAREN => "()",
            Java::LBRACK => "[]",
            Java::LBRACE => "{}",
            Java::VoidType => "void",
            _ => typ.into(),
        }
    }
}

impl Getter for KotlinCode {}

// BEAM languages - Elixir, Erlang, Gleam (full implementations)
impl Getter for ElixirCode {
    fn get_space_kind(node: &Node) -> SpaceKind {
        match node.kind_id().into() {
            Elixir::Source => SpaceKind::Unit,
            Elixir::AnonymousFunction => SpaceKind::Function,
            Elixir::DoBlock => {
                if let Some(parent) = node.parent() {
                    if parent.kind_id() == Elixir::Call {
                        if let Some(head) = parent.child(0) {
                            if head.kind_id() == Elixir::Identifier {
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
        match node.kind_id().into() {
            Elixir::DoBlock => {
                let parent = node.parent()?;
                if parent.kind_id() != Elixir::Call {
                    return None;
                }

                let keyword_node = parent.child(0)?;
                if keyword_node.kind_id() != Elixir::Identifier {
                    return None;
                }

                let keyword = node_text(&keyword_node, code)?;
                let arguments = (0..parent.child_count())
                    .filter_map(|idx| parent.child(idx))
                    .find(|child| {
                        child.is_named()
                            && Into::<Elixir>::into(child.kind_id()) == Elixir::Arguments
                    })?;

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
            Elixir::AnonymousFunction => Some("anonymous_function"),
            _ => default_space_name(node, code),
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use Elixir::{
            Alias, AnonymousFunction, Arguments, Atom, BinaryOperator, Call, Charlist, Dot,
            Identifier, Integer, Keywords, List, Map, OperatorIdentifier, QuotedAtom, Sigil,
            String, Struct, Tuple, UnaryOperator,
        };

        match node.kind_id().into() {
            BinaryOperator | UnaryOperator | OperatorIdentifier | Dot => HalsteadType::Operator,
            Call | Arguments => HalsteadType::Operator,
            Identifier | Alias | Atom | QuotedAtom | Integer | String | Charlist | Sigil | List
            | Tuple | Map | Struct | Keywords => HalsteadType::Operand,
            AnonymousFunction => HalsteadType::Operand,
            _ => match node.kind() {
                "+" | "-" | "*" | "/" | "%" | "++" | "--" | "::" | "->" | "<-" | "<>" | "||"
                | "&&" | "===" | "==" | "!==" | "!=" | "<" | "<=" | ">" | ">=" | "in" | "when"
                | "and" | "or" | "not" | "xor" | "<<<" | ">>>" | "^^^" | "~~~" | "&&&" | "|||"
                | "." => HalsteadType::Operator,
                "if" | "unless" | "case" | "fn" | "do" | "after" | "rescue" | "catch" | "else" => {
                    HalsteadType::Operator
                }
                "nil" | "true" | "false" => HalsteadType::Operand,
                _ => HalsteadType::Unknown,
            },
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
                child.kind_id().into(),
                Elixir::Alias | Elixir::Atom | Elixir::Identifier | Elixir::QuotedAtom
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
            match child.kind_id().into() {
                Elixir::Call => {
                    for call_idx in 0..child.child_count() {
                        if let Some(name_node) = child.child(call_idx) {
                            if !name_node.is_named() {
                                continue;
                            }
                            if matches!(
                                name_node.kind_id().into(),
                                Elixir::Identifier | Elixir::Atom | Elixir::QuotedAtom
                            ) {
                                return node_text(&name_node, code);
                            }
                            break;
                        }
                    }
                }
                Elixir::Identifier | Elixir::Atom | Elixir::QuotedAtom => {
                    return node_text(&child, code)
                }
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
        use crate::Erlang::*;

        match node.kind_id().into() {
            SourceFile => SpaceKind::Unit,
            FunDecl | FunctionClause => SpaceKind::Function,
            AnonymousFun => SpaceKind::Function,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        use crate::Erlang::*;

        match node.kind_id().into() {
            FunDecl | FunctionClause => {
                // Erlang functions are typically named with atoms
                node.child_by_field_name("name")
                    .and_then(|name| node_text(&name, code))
            }
            AnonymousFun => Some("anonymous_function"),
            _ => default_space_name(node, code),
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use crate::Erlang::{
            Atom, BinaryOpExpr, CatchExpr, List, MapExpr, MatchExpr, Tuple, UnaryOpExpr, Var,
        };

        match node.kind_id().into() {
            BinaryOpExpr | UnaryOpExpr | MatchExpr | CatchExpr => HalsteadType::Operator,
            Atom | Var | List | Tuple | MapExpr => HalsteadType::Operand,
            _ => match node.kind() {
                "+" | "-" | "*" | "/" | "%" | "div" | "rem" | "band" | "bor" | "bxor" | "bsl"
                | "bsr" | "and" | "or" | "not" | "xor" | "orelse" | "andalso" | "==" | "/="
                | "=:= " | "=/=" | "<" | "<=" | ">" | ">=" | "++" | "--" | "!" | "catch" | "of"
                | "after" => HalsteadType::Operator,
                "(" | ")" | "[" | "]" | "{" | "}" => HalsteadType::Unknown,
                _ => HalsteadType::Unknown,
            },
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
        use crate::Gleam::*;

        match node.kind_id().into() {
            SourceFile => SpaceKind::Unit,
            Function => SpaceKind::Function,
            AnonymousFunction => SpaceKind::Function,
            _ => SpaceKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        use crate::Gleam::*;

        match node.kind_id().into() {
            Function => node
                .child_by_field_name("name")
                .and_then(|name| node_text(&name, code)),
            AnonymousFunction => Some("anonymous_function"),
            _ => default_space_name(node, code),
        }
    }

    fn get_op_type(node: &Node) -> HalsteadType {
        use crate::Gleam::*;

        match node.kind_id().into() {
            BinaryExpression | BooleanNegation | IntegerNegation | PipelineEcho => {
                HalsteadType::Operator
            }
            Case | Let => HalsteadType::Operator,
            Identifier | Integer | Float | String | Comment => HalsteadType::Operand,
            _ => match node.kind() {
                "+" | "-" | "*" | "/" | "%" | "++" | "--" | "<" | "<=" | ">" | ">=" | "=="
                | "!=" | "&&" | "||" | "<-" | "->" | "if" | "else" => HalsteadType::Operator,
                _ => HalsteadType::Unknown,
            },
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

// Lua (minimal implementation)
impl Getter for LuaCode {}

// Compatibility implementations for unimplemented languages
impl Getter for GoCode {}
impl Getter for CsharpCode {}
