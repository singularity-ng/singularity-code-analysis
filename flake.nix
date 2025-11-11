{
  description = "Singularity Analysis Engine development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain - latest stable
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        # Build inputs for the project
        buildInputs = with pkgs; [
          # Rust and cargo tools
          rustToolchain
          cargo-edit      # cargo add/rm/upgrade
          cargo-watch     # cargo watch for auto-recompilation
          cargo-audit     # security audit
          cargo-outdated  # check for outdated dependencies
          cargo-tarpaulin # code coverage
          cargo-nextest   # better test runner
          cargo-machete   # find unused dependencies
          cargo-deny      # lint dependencies
          cargo-release   # release automation
          sccache         # compilation cache

          # Elixir and Erlang
          beam.packages.erlang_27.elixir_1_17  # Latest stable Elixir (1.17 is the latest stable, 1.19 doesn't exist yet)
          beam.packages.erlang_27.erlang
          beam.packages.erlang_27.rebar3
          beam.packages.erlang_27.hex

          # Elixir tools from Nix (pre-built, no compilation needed)
          beam.packages.erlang_27.elixir-ls  # Language server

          # Build dependencies for NIFs
          pkg-config
          openssl

          # Development tools
          git
          gnumake
          gcc
          libiconv

          # Optional: useful for development
          jq
          ripgrep
          fd
          bat
          eza
          tokei  # code statistics
        ];

        # Set up environment variables
        shellHook = ''
          echo "🚀 Singularity Analysis Engine Development Environment"
          echo ""
          echo "📦 Versions:"
          echo "  • Rust: $(rustc --version | cut -d' ' -f2)"
          echo "  • Cargo: $(cargo --version | cut -d' ' -f2)"
          echo "  • Elixir: $(elixir --version | grep Elixir | cut -d' ' -f2)"
          echo "  • Erlang/OTP: $(erl -eval 'erlang:display(erlang:system_info(otp_release)), halt().' -noshell | tr -d '\"')"
          echo ""
          echo "🛠️  Available Cargo tools:"
          echo "  • cargo-edit (add/rm/upgrade)"
          echo "  • cargo-watch (auto-recompile)"
          echo "  • cargo-audit (security)"
          echo "  • cargo-outdated"
          echo "  • cargo-tarpaulin (coverage)"
          echo "  • cargo-nextest (testing)"
          echo "  • cargo-machete (unused deps)"
          echo "  • cargo-deny (lint deps)"
          echo "  • cargo-release"
          echo ""
          echo "🧪 Elixir tools:"
          echo "  • elixir-ls (language server)"
          echo "  • hex (package manager)"
          echo "  • rebar3 (Erlang build tool)"
          echo ""
          echo "💡 Tips:"
          echo "  • Run 'mix local.hex --force' if hex needs setup"
          echo "  • Run 'mix local.rebar --force' if rebar needs setup"
          echo "  • Install Elixir deps: 'mix deps.get'"
          echo "  • For credo: 'mix archive.install hex credo'"
          echo "  • For dialyxir: add to mix.exs deps and 'mix deps.get'"
          echo ""

          # Set up Rust environment
          export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
          export RUST_BACKTRACE=1

          # Set up build cache
          export SCCACHE_DIR="$PWD/.sccache"
          export RUSTC_WRAPPER="${pkgs.sccache}/bin/sccache"

          # Elixir/Erlang environment
          export ERL_AFLAGS="-kernel shell_history enabled"
          export HEX_HOME="$PWD/.hex"
          export MIX_HOME="$PWD/.mix"

          # Create local directories if they don't exist
          mkdir -p .sccache .hex .mix

          # Install local hex and rebar if not present
          if [ ! -f "$MIX_HOME/escripts/hex" ]; then
            echo "📥 Installing local hex..."
            mix local.hex --force --if-missing
          fi

          if [ ! -f "$MIX_HOME/escripts/rebar" ] && [ ! -f "$MIX_HOME/escripts/rebar3" ]; then
            echo "📥 Installing local rebar..."
            mix local.rebar --force --if-missing
          fi
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          inherit shellHook;

          # Additional environment variables
          RUST_LOG = "debug";
          CARGO_HOME = "$PWD/.cargo";
        };

        # Also provide a minimal shell without the startup message
        devShells.minimal = pkgs.mkShell {
          inherit buildInputs;
          shellHook = ''
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
            export RUST_BACKTRACE=1
            export SCCACHE_DIR="$PWD/.sccache"
            export RUSTC_WRAPPER="${pkgs.sccache}/bin/sccache"
            export ERL_AFLAGS="-kernel shell_history enabled"
            export HEX_HOME="$PWD/.hex"
            export MIX_HOME="$PWD/.mix"
            export RUST_LOG="debug"
            export CARGO_HOME="$PWD/.cargo"
            mkdir -p .sccache .hex .mix .cargo
          '';
        };
      });
}