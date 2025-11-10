/// Inspects Python tree-sitter node types to help understand the AST structure
use tree_sitter::Parser;

fn main() {
    let language = tree_sitter_python::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source_code = r"
def f(a, b):
    if a and b:
        return 1
    if c and d:
        return 1
";

    let tree = parser.parse(source_code, None).unwrap();
    let root = tree.root_node();

    println!("=== Python Tree-Sitter Node Types ===\n");
    print_node(&root, source_code.as_bytes(), 0);

    println!("\n=== All Named Node Kinds ===");
    for i in 0..language.node_kind_count() {
        let Ok(kind_id) = u16::try_from(i) else {
            break;
        };
        if language.node_kind_is_named(kind_id) {
            if let Some(kind) = language.node_kind_for_id(kind_id) {
                println!("{kind_id:3}: {kind}");
            }
        }
    }
}

fn print_node(node: &tree_sitter::Node, source: &[u8], depth: usize) {
    let indent = "  ".repeat(depth);
    let kind = node.kind();
    let start = node.start_position();
    let end = node.end_position();

    let text = if node.child_count() == 0 {
        String::from_utf8_lossy(&source[node.start_byte()..node.end_byte()])
            .chars()
            .take(50)
            .collect::<String>()
    } else {
        String::new()
    };

    if text.is_empty() {
        println!(
            "{}[{}] {} ({}, {}) -> ({}, {})",
            indent,
            node.id(),
            kind,
            start.row,
            start.column,
            end.row,
            end.column
        );
    } else {
        println!(
            "{}[{}] {} ({}, {}) -> ({}, {}): \"{}\"",
            indent,
            node.id(),
            kind,
            start.row,
            start.column,
            end.row,
            end.column,
            text
        );
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            print_node(&child, source, depth + 1);
        }
    }
}
