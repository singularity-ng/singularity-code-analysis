use std::{
    collections::{HashMap, VecDeque},
    fs, mem,
};

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use convert_case::{Case, Casing};
use indoc::formatdoc;
use itertools::Itertools;
use serde::Deserialize;
use tree_sitter::Language;
use tree_sitter_language::LanguageFn;

#[derive(Parser)]
#[command(author, version, about = "Regenerate tree-sitter enums and macros")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate language enums
    Languages {
        #[arg(long = "out", value_name = "DIR", default_value = "src/languages")]
        out: Utf8PathBuf,
        #[arg(long = "only", value_name = "LANG", value_delimiter = ',')]
        only: Vec<String>,
    },
    /// Generate C macro helpers
    Macros {
        #[arg(long = "out", value_name = "DIR", default_value = "src/c_langs_macros")]
        out: Utf8PathBuf,
    },
}

struct LanguageConfig {
    key: &'static str,
    enum_name: &'static str,
    language: fn() -> Language,
}

static LANGUAGES: &[LanguageConfig] = &[
    LanguageConfig {
        key: "rust",
        enum_name: "Rust",
        language: || lang_from_fn(tree_sitter_rust::LANGUAGE),
    },
    LanguageConfig {
        key: "python",
        enum_name: "Python",
        language: || lang_from_fn(tree_sitter_python::LANGUAGE),
    },
    LanguageConfig {
        key: "java",
        enum_name: "Java",
        language: || lang_from_fn(tree_sitter_java::LANGUAGE),
    },
    LanguageConfig {
        key: "javascript",
        enum_name: "Javascript",
        language: || lang_from_fn(tree_sitter_javascript::LANGUAGE),
    },
    LanguageConfig {
        key: "typescript",
        enum_name: "Typescript",
        language: || lang_from_fn(tree_sitter_typescript::LANGUAGE_TYPESCRIPT),
    },
    LanguageConfig {
        key: "tsx",
        enum_name: "Tsx",
        language: || lang_from_fn(tree_sitter_typescript::LANGUAGE_TSX),
    },
    LanguageConfig {
        key: "mozjs",
        enum_name: "Mozjs",
        language: || tree_sitter_mozjs::language(),
    },
    LanguageConfig {
        key: "cpp",
        enum_name: "Cpp",
        language: || lang_from_fn(tree_sitter_cpp::LANGUAGE),
    },
    LanguageConfig {
        key: "csharp",
        enum_name: "Csharp",
        language: || lang_from_fn(tree_sitter_c_sharp::LANGUAGE),
    },
    LanguageConfig {
        key: "elixir",
        enum_name: "Elixir",
        language: || lang_from_fn(tree_sitter_elixir::LANGUAGE),
    },
    LanguageConfig {
        key: "erlang",
        enum_name: "Erlang",
        language: || lang_from_fn(tree_sitter_erlang::LANGUAGE),
    },
    LanguageConfig {
        key: "gleam",
        enum_name: "Gleam",
        language: || lang_from_fn(tree_sitter_gleam::LANGUAGE),
    },
    LanguageConfig {
        key: "lua",
        enum_name: "Lua",
        language: || lang_from_fn(tree_sitter_lua::LANGUAGE),
    },
    LanguageConfig {
        key: "go",
        enum_name: "Go",
        language: || lang_from_fn(tree_sitter_go::LANGUAGE),
    },
    LanguageConfig {
        key: "kotlin",
        enum_name: "Kotlin",
        language: || lang_from_fn(tree_sitter_kotlin_ng::LANGUAGE),
    },
];

fn lang_from_fn(language_fn: LanguageFn) -> Language {
    let raw_fn = language_fn.into_raw();
    let raw_ptr = unsafe { raw_fn() };
    unsafe { mem::transmute(raw_ptr) }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Languages { out, only } => {
            generate_languages(out, only)?;
        }
        Command::Macros { out } => {
            generate_macros(out)?;
        }
    }
    Ok(())
}

fn generate_languages(out_dir: Utf8PathBuf, only: Vec<String>) -> Result<()> {
    fs::create_dir_all(&out_dir)?;
    for cfg in LANGUAGES {
        if !should_emit(cfg, &only) {
            continue;
        }
        generate_language(cfg, &out_dir)?;
    }
    Ok(())
}

fn should_emit(cfg: &LanguageConfig, only: &[String]) -> bool {
    if only.is_empty() {
        return true;
    }
    only.iter()
        .any(|item| item.eq_ignore_ascii_case(cfg.key) || item.eq_ignore_ascii_case(cfg.enum_name))
}

fn generate_language(cfg: &LanguageConfig, out_dir: &Utf8PathBuf) -> Result<()> {
    let path = out_dir.join(format!("language_{}.rs", cfg.key));
    let mut existing = load_existing_variants(&path, cfg.enum_name);
    let kinds = collect_kinds((cfg.language)());
    let kinds = apply_existing_variants(kinds, &mut existing);
    let content = render_language(cfg, &kinds);
    fs::write(&path, content).with_context(|| format!("Failed to write {}", path))?;
    Ok(())
}

#[derive(Clone)]
struct KindInfo {
    variant: String,
    literal: String,
    id: u16,
}

fn collect_kinds(language: Language) -> Vec<KindInfo> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut kinds = Vec::new();
    let count = language.node_kind_count();
    for id in 0..count {
        let id = id as u16;
        if let Some(name) = language.node_kind_for_id(id) {
            let variant = make_variant_name(name, &mut counts);
            kinds.push(KindInfo {
                variant,
                literal: name.to_owned(),
                id,
            });
        }
    }
    if !kinds.iter().any(|k| k.literal == "ERROR") {
        let variant = make_variant_name("ERROR", &mut counts);
        kinds.push(KindInfo {
            variant,
            literal: "ERROR".to_owned(),
            id: 65535,
        });
    }
    kinds
}

fn apply_existing_variants(
    mut kinds: Vec<KindInfo>,
    existing: &mut HashMap<String, VecDeque<String>>,
) -> Vec<KindInfo> {
    for kind in &mut kinds {
        if let Some(names) = existing.get_mut(&kind.literal) {
            if let Some(existing_name) = names.pop_front() {
                kind.variant = existing_name;
                continue;
            }
        }
    }
    kinds
}

fn load_existing_variants(
    path: &Utf8PathBuf,
    enum_name: &str,
) -> HashMap<String, VecDeque<String>> {
    let mut map = HashMap::new();
    let content = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(_) => return map,
    };
    let prefix = format!("{enum_name}::");
    for line in content.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with(&prefix) {
            continue;
        }
        let remainder = &trimmed[prefix.len()..];
        let (variant_part, literal_part) = match remainder.split_once("=>") {
            Some(parts) => parts,
            None => continue,
        };
        let variant = variant_part.trim().trim_end_matches(',');
        if variant.is_empty() {
            continue;
        }
        if let Some(literal) = parse_literal(literal_part) {
            map.entry(literal)
                .or_insert_with(VecDeque::new)
                .push_back(variant.to_string());
        }
    }
    map
}

fn parse_literal(segment: &str) -> Option<String> {
    let token = segment.trim().trim_end_matches(',');
    if !token.starts_with('"') {
        return None;
    }
    serde_json::from_str(token).ok()
}

fn render_language(cfg: &LanguageConfig, kinds: &[KindInfo]) -> String {
    let enum_variants = kinds
        .iter()
        .map(|kind| format!("    {} = {},", kind.variant, kind.id))
        .join("\n");

    let match_from = kinds
        .iter()
        .map(|kind| {
            format!(
                "            {}::{} => {},",
                cfg.enum_name,
                kind.variant,
                quote(&kind.literal)
            )
        })
        .join("\n");

    formatdoc!(
        "// Code generated; DO NOT EDIT.\n\n#![allow(clippy::match_same_arms, clippy::match_wildcard_for_single_variants, clippy::too_many_lines)]\n\nuse num_derive::FromPrimitive;\n\n#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]\npub enum {enum_name} {{\n{enum_variants}\n}}\n\nimpl From<{enum_name}> for &'static str {{\n    #[inline]\n    fn from(tok: {enum_name}) -> Self {{\n        match tok {{\n{match_from}\n        }}\n    }}\n}}\n\nimpl From<u16> for {enum_name} {{\n    #[inline]\n    fn from(x: u16) -> Self {{\n        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)\n    }}\n}}\n\nimpl PartialEq<u16> for {enum_name} {{\n    #[inline]\n    fn eq(&self, x: &u16) -> bool {{\n        *self == Into::<Self>::into(*x)\n    }}\n}}\n\nimpl PartialEq<{enum_name}> for u16 {{\n    #[inline]\n    fn eq(&self, x: &{enum_name}) -> bool {{\n        *x == *self\n    }}\n}}\n",
        enum_name = cfg.enum_name,
        enum_variants = enum_variants,
        match_from = match_from,
    )
}

fn make_variant_name(raw: &str, counts: &mut HashMap<String, usize>) -> String {
    let base = if let Some(mapped) = special_symbol(raw) {
        mapped.to_string()
    } else {
        sanitize_identifier(raw)
    };
    let entry = counts.entry(base.clone()).or_insert(0);
    if *entry == 0 {
        *entry = 1;
        base
    } else {
        *entry += 1;
        format!("{base}{}", *entry)
    }
}

fn sanitize_identifier(raw: &str) -> String {
    let cleaned = raw
        .chars()
        .map(|ch| if ch.is_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .to_case(Case::UpperCamel);
    let mut name = if cleaned.is_empty() {
        "Unnamed".to_string()
    } else {
        cleaned
    };
    if name.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        name = format!("_{name}");
    }
    if is_reserved_variant(&name) {
        name.push_str("Kind");
    }
    name
}

fn is_reserved_variant(name: &str) -> bool {
    matches!(name, "Self")
}

fn special_symbol(raw: &str) -> Option<&'static str> {
    match raw {
        ";" => Some("SEMI"),
        ":" => Some("COLON"),
        "," => Some("COMMA"),
        "." => Some("DOT"),
        "(" => Some("LPAREN"),
        ")" => Some("RPAREN"),
        "[" => Some("LBRACK"),
        "]" => Some("RBRACK"),
        "{" => Some("LBRACE"),
        "}" => Some("RBRACE"),
        "+" => Some("PLUS"),
        "-" => Some("DASH"),
        "*" => Some("STAR"),
        "/" => Some("SLASH"),
        "%" => Some("PERCENT"),
        "&" => Some("AMP"),
        "|" => Some("PIPE"),
        "^" => Some("CARET"),
        "!" => Some("BANG"),
        "?" => Some("QMARK"),
        "~" => Some("TILDE"),
        "@" => Some("AT"),
        "#" => Some("HASH"),
        "$" => Some("DOLLAR"),
        "&&" => Some("AMPAMP"),
        "||" => Some("PIPEPIPE"),
        "<<" => Some("LTLT"),
        ">>" => Some("GTGT"),
        "<=" => Some("LTEQ"),
        ">=" => Some("GTEQ"),
        "==" => Some("EQEQ"),
        "!=" => Some("BANGEQ"),
        "+=" => Some("PLUSEQ"),
        "-=" => Some("DASHEQ"),
        "*=" => Some("STAREQ"),
        "/=" => Some("SLASHEQ"),
        "%=" => Some("PERCENTEQ"),
        "&=" => Some("AMPEQ"),
        "|=" => Some("PIPEEQ"),
        "^=" => Some("CARETEQ"),
        "<<=" => Some("LTLTEQ"),
        ">>=" => Some("GTGTEQ"),
        "=>" => Some("EQGT"),
        "->" => Some("DASHGT"),
        "<-" => Some("LARROW"),
        "**" => Some("STARSTAR"),
        "//" => Some("SLASHSLASH"),
        "**=" => Some("STARSTAREQ"),
        "//=" => Some("SLASHSLASHEQ"),
        "..." => Some("Ellipsis"),
        "_" => Some("UNDERSCORE"),
        _ => None,
    }
}

fn quote(value: &str) -> String {
    serde_json::to_string(value).expect("TODO: Add context for why this shouldn't fail")
}

fn generate_macros(out_dir: Utf8PathBuf) -> Result<()> {
    fs::create_dir_all(&out_dir)?;
    let data = macros_data();
    let macros_rs = formatdoc!(
        "// Code generated; DO NOT EDIT.\n\nconst PREDEFINED_MACROS: &[&str] = &[\n{macros}\n];\n\npub fn is_predefined_macros(mac: &str) -> bool {{\n    PREDEFINED_MACROS.contains(&mac)\n}}\n",
        macros = data
            .predefined
            .iter()
            .map(|value| format!("    {},", quote(value)))
            .join("\n"),
    );
    let specials_rs = formatdoc!(
        "// Code generated; DO NOT EDIT.\n\nconst SPECIALS: &[&str] = &[\n{specials}\n];\n\npub fn is_specials(mac: &str) -> bool {{\n    SPECIALS.contains(&mac)\n}}\n",
        specials = data
            .specials
            .iter()
            .map(|value| format!("    {},", quote(value)))
            .join("\n"),
    );
    fs::write(out_dir.join("c_macros.rs"), macros_rs)?;
    fs::write(out_dir.join("c_specials.rs"), specials_rs)?;
    Ok(())
}

#[derive(Deserialize)]
struct MacroData {
    predefined: Vec<String>,
    specials: Vec<String>,
}

fn macros_data() -> MacroData {
    serde_json::from_str(include_str!("../data/c_macros.json")).expect("invalid macros data")
}
