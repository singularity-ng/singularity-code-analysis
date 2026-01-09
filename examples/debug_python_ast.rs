// Debug tool to inspect Python AST structure for boolean operators
use tree_sitter::Parser;

fn main() {
    let code = "if a and b:\n    pass";
    println!("Code:\n{code}\n");

    let language = tree_sitter_python::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language).expect("TODO: Add context for why this shouldn't fail");

    let tree = parser.parse(code, None).expect("TODO: Add context for why this shouldn't fail");
    let root = tree.root_node();

    println!("AST Structure:");
    print_tree(&root, 0);
}

fn print_tree(node: &tree_sitter::Node, depth: usize) {
    let indent = "  ".repeat(depth);
    let kind_id = node.kind_id();
    let child_desc = if node.child_count() > 0 {
        format!("{} children", node.child_count())
    } else {
        "leaf".to_string()
    };
    println!("{indent}[{kind_id:3}] {} ({child_desc})", node.kind(),);

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            print_tree(&child, depth + 1);
        }
    }
}
