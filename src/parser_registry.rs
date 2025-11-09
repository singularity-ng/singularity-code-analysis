use std::{collections::HashMap, path::Path, sync::Arc};

use crate::traits::{LanguageInfo, ParserTrait};
use crate::{
    abc::Abc, alterator::Alterator, checker::Checker, cognitive::Cognitive, cyclomatic::Cyclomatic,
    exit::Exit, getter::Getter, halstead::Halstead, langs::*, loc::Loc, mi::Mi, nargs::NArgs,
    nom::Nom, npa::Npa, npm::Npm, preproc::PreprocResults, wmc::Wmc,
};

/// A registry for managing parsers for different programming languages.
/// Provides dynamic registration and lookup of parsers by language type.
pub struct ParserRegistry {
    parsers: HashMap<LANG, Box<dyn ParserFactory>>,
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserRegistry {
    /// Create a new empty parser registry.
    pub fn new() -> Self {
        Self {
            parsers: HashMap::new(),
        }
    }

    /// Create a new parser registry with all built-in parsers registered.
    #[allow(dead_code)]
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register_builtin_parsers();
        registry
    }

    /// Register a parser factory for a specific language.
    pub fn register<T>(&mut self, language: LANG, factory: Box<dyn ParserFactory>)
    where
        T: 'static
            + Send
            + Sync
            + LanguageInfo
            + Alterator
            + Checker
            + Getter
            + Abc
            + Cognitive
            + Cyclomatic
            + Exit
            + Halstead
            + Loc
            + Mi
            + NArgs
            + Nom
            + Npa
            + Npm
            + Wmc,
    {
        self.parsers.insert(language, factory);
    }

    /// Get a parser factory for the specified language.
    pub fn get_factory(&self, language: &LANG) -> Option<&dyn ParserFactory> {
        self.parsers.get(language).map(|boxed| boxed.as_ref())
    }

    /// Create a parser for the given code and language.
    pub fn create_parser(
        &self,
        language: &LANG,
        code: Vec<u8>,
        path: &Path,
        pr: Option<Arc<PreprocResults>>,
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>> {
        self.parsers
            .get(language)
            .ok_or_else(|| Box::<dyn std::error::Error>::from("Parser not found for language"))?
            .create_parser(code, path, pr)
    }

    /// Detect language from file extension.
    pub fn detect_language_from_path(&self, path: &Path) -> Option<LANG> {
        let extension = path.extension()?.to_str()?;

        // Check all registered parsers for matching extensions
        for (lang, factory) in &self.parsers {
            if factory.get_extensions().contains(&extension) {
                return Some(*lang);
            }
        }

        None
    }

    /// Get all supported languages.
    pub fn supported_languages(&self) -> Vec<LANG> {
        self.parsers.keys().cloned().collect()
    }

    /// Register all built-in parsers.
    fn register_builtin_parsers(&mut self) {
        // Register all built-in language parsers
        self.register_parser::<JavascriptCode>(LANG::Javascript);
        self.register_parser::<JavaCode>(LANG::Java);
        self.register_parser::<RustCode>(LANG::Rust);
        self.register_parser::<CppCode>(LANG::Cpp);
        self.register_parser::<PythonCode>(LANG::Python);
        self.register_parser::<TsxCode>(LANG::Tsx);
        self.register_parser::<TypescriptCode>(LANG::Typescript);
        self.register_parser::<ElixirCode>(LANG::Elixir);
        self.register_parser::<ErlangCode>(LANG::Erlang);
        self.register_parser::<GleamCode>(LANG::Gleam);
        self.register_parser::<LuaCode>(LANG::Lua);
        self.register_parser::<GoCode>(LANG::Go);
        self.register_parser::<CsharpCode>(LANG::Csharp);
    }

    /// Helper method to register a built-in parser.
    fn register_parser<T>(&mut self, language: LANG)
    where
        T: 'static
            + Send
            + Sync
            + LanguageInfo
            + Alterator
            + Checker
            + Getter
            + Abc
            + Cognitive
            + Cyclomatic
            + Exit
            + Halstead
            + Loc
            + Mi
            + NArgs
            + Nom
            + Npa
            + Npm
            + Wmc,
    {
        let factory = Box::new(BuiltinParserFactory::<T>::new());
        self.parsers.insert(language, factory);
    }
}

/// Trait for parser factories that can create parsers for specific languages.
pub trait ParserFactory: Send + Sync {
    /// Create a parser instance for the given code and path.
    fn create_parser(
        &self,
        code: Vec<u8>,
        path: &Path,
        pr: Option<Arc<PreprocResults>>,
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>>;

    /// Get the file extensions supported by this parser.
    fn get_extensions(&self) -> Vec<&str>;

    /// Get the language type for this parser.
    fn get_language(&self) -> LANG;
}

/// Built-in parser factory implementation.
struct BuiltinParserFactory<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> BuiltinParserFactory<T> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<
        T: 'static
            + LanguageInfo
            + Alterator
            + Checker
            + Getter
            + Abc
            + Cognitive
            + Cyclomatic
            + Exit
            + Halstead
            + Loc
            + Mi
            + NArgs
            + Nom
            + Npa
            + Npm
            + Wmc
            + Send
            + Sync,
    > ParserFactory for BuiltinParserFactory<T>
{
    fn create_parser(
        &self,
        code: Vec<u8>,
        path: &Path,
        pr: Option<Arc<PreprocResults>>,
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>> {
        Ok(Box::new(crate::parser::Parser::<T>::new(code, path, pr)))
    }

    fn get_extensions(&self) -> Vec<&str> {
        // Get extensions from the language info
        // This is a simplified implementation - in practice you'd need to
        // extract this from the language definitions
        match T::get_lang() {
            LANG::Javascript => vec!["js", "mjs", "jsx"],
            LANG::Java => vec!["java"],
            LANG::Rust => vec!["rs"],
            LANG::Cpp => vec![
                "cpp", "cxx", "cc", "hxx", "hpp", "c", "h", "hh", "inc", "mm", "m",
            ],
            LANG::Python => vec!["py"],
            LANG::Tsx => vec!["tsx"],
            LANG::Typescript => vec!["ts", "jsw", "jsmw"],
            LANG::Elixir => vec!["ex", "exs"],
            LANG::Erlang => vec!["erl", "hrl"],
            LANG::Gleam => vec!["gleam"],
            LANG::Lua => vec!["lua"],
            LANG::Go => vec!["go"],
            LANG::Csharp => vec!["cs", "csx"],
            LANG::Kotlin => vec!["kt", "kts"],
            // C not yet fully implemented
        }
    }

    fn get_language(&self) -> LANG {
        T::get_lang()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_registry_creation() {
        let registry = ParserRegistry::new();
        assert!(registry.supported_languages().is_empty());
    }

    #[test]
    fn test_builtin_registry() {
        let registry = ParserRegistry::with_builtins();
        let languages = registry.supported_languages();
        assert!(!languages.is_empty());
        assert!(languages.contains(&LANG::Rust));
        assert!(languages.contains(&LANG::Elixir));
    }

    #[test]
    fn test_language_detection() {
        let registry = ParserRegistry::with_builtins();

        let rust_path = PathBuf::from("test.rs");
        assert_eq!(
            registry.detect_language_from_path(&rust_path),
            Some(LANG::Rust)
        );

        let elixir_path = PathBuf::from("test.ex");
        assert_eq!(
            registry.detect_language_from_path(&elixir_path),
            Some(LANG::Elixir)
        );

        let unknown_path = PathBuf::from("test.unknown");
        assert_eq!(registry.detect_language_from_path(&unknown_path), None);
    }

    #[test]
    fn test_parser_creation() {
        let registry = ParserRegistry::with_builtins();

        let code = b"fn main() {}".to_vec();
        let path = PathBuf::from("test.rs");

        let parser_result = registry.create_parser(&LANG::Rust, code, &path, None);
        assert!(parser_result.is_ok());

        let parser_any = parser_result.unwrap();
        // We can't easily test the downcast here without more complex setup
        // but we can verify it returns something
        assert!(parser_any.is::<crate::parser::Parser<crate::RustCode>>());
    }
}
