use singularity_code_analysis::{get_function_spaces, LANG};
use std::path::Path;

#[test]
fn test_elixir_function_detection() {
    let elixir_code = r#"
defmodule Test do
  def hello(name) do
    "Hello, #{name}!"
  end

  defp private_func do
    :ok
  end
end
"#;

    let path = Path::new("test.ex");
    println!("Testing Elixir function detection:");
    let func_space =
        get_function_spaces(&LANG::Elixir, elixir_code.as_bytes().to_vec(), path, None)
            .expect("No function spaces found");

    println!("Success! Found {} spaces", func_space.spaces.len());
    for space in &func_space.spaces {
        println!(
            "  - {}: {}",
            space.kind,
            space.name.as_ref().unwrap_or(&"unnamed".to_string())
        );
    }
    assert!(!func_space.spaces.is_empty());
}
