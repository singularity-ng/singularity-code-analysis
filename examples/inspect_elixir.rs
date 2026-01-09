/// Inspects Elixir tree-sitter node types to help build proper language enum
use tree_sitter::Parser;

fn main() {
    let language = tree_sitter_elixir::LANGUAGE.into();
    let mut parser = Parser::new();
    parser.set_language(&language).expect("TODO: Add context for why this shouldn't fail");

    let source_code = r#"
defmodule Example do
  @moduledoc "Test module"

  def add(a, b) do
    a + b
  end

  defp private_func(x) do
    case x do
      1 -> :one
      _ -> :other
    end
  end

  def with_if(val) do
    if val > 0 do
      "positive"
    else
      "negative"
    end
  end
end
"#;

    let tree = parser.parse(source_code, None).expect("TODO: Add context for why this shouldn't fail");
    let root = tree.root_node();

    println!("=== Elixir Tree-Sitter Node Types ===\n");
    print_node(&root, source_code.as_bytes(), 0);

    println!("\n=== All Named Node Kinds ===");
    for i in 0..language.node_kind_count() {
        let Ok(kind_id) = u16::try_from(i) else {
            break;
        };
        if language.node_kind_is_named(kind_id) && let Some(kind) = language.node_kind_for_id(kind_id) {
            println!("{kind_id:3}: {kind}");
        }
    }
}

fn print_node(node: &tree_sitter::Node, source: &[u8], depth: usize) {
    let indent = "  ".repeat(depth);
    let kind = node.kind();
    let id = node.kind_id();

    if node.is_named() {
        let text = if node.child_count() == 0 {
            let bytes = &source[node.start_byte()..node.end_byte()];
            String::from_utf8_lossy(bytes).to_string()
        } else {
            String::new()
        };

        if !text.is_empty() && text.len() < 30 {
            println!("{indent}{kind} [{id}] = \"{text}\"");
        } else {
            println!("{indent}{kind} [{id}]");
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                print_node(&child, source, depth + 1);
            }
        }
    }
}
