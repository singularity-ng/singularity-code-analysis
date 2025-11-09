mod c_macros;
pub use c_macros::*;

mod c_specials;
pub use c_specials::*;

#[cfg(test)]
mod tests {

    use std::{collections::HashSet, path::PathBuf};

    use crate::{c_macro, *};

    fn parse(samples: &[&str], debug: bool, expect_success: bool) {
        let path = PathBuf::from("foo.c");
        let macros = [
            "MOZ_ALWAYS_INLINE",
            "MOZ_NEVER_INLINE",
            "MOZ_NONHEAP_CLASS",
            "QM_TRY_INSPECT",
            "MOZ_TO_RESULT_INVOKE",
        ];
        let macro_set: HashSet<String> = macros.iter().map(|m| (*m).to_string()).collect();
        for (n, sample) in samples.iter().enumerate() {
            let mut code = sample.as_bytes().to_vec();
            let replaced = c_macro::replace(&code, &macro_set);
            if let Some(fake) = replaced.clone() {
                code = fake
                    .into_iter()
                    .map(|b| if b == b'$' { b' ' } else { b })
                    .collect();
            }
            let parser = CppParser::new(code.clone(), &path, None);
            let root = parser.get_root();
            if debug || root.has_error() {
                let display = String::from_utf8_lossy(sample.as_bytes());
                eprintln!("Sample (CPP) {n}: {display}");
                dump_node(&code, &root, -1, None, None).unwrap();
            }
            if expect_success {
                assert!(
                    replaced.is_some(),
                    "expected macro substitution for {sample}"
                );
                assert!(!root.has_error());
            }
        }
    }

    #[test]
    fn test_fn_macros() {
        let samples = vec![
            "MOZ_ALWAYS_INLINE void f() { }",
            "MOZ_NEVER_INLINE void f() { }",
        ];
        parse(&samples, false, true);
    }

    #[test]
    fn test_fn_macros_cpp() {
        let samples = vec!["class MOZ_NONHEAP_CLASS Factory : public IClassFactory {};"];
        parse(&samples, false, true);
    }

    #[test]
    #[ignore = "tree-sitter-cpp parser limitation with Mozilla macros - GitHub issue #1142"]
    fn test_fn_id_strings() {
        let samples = vec!["nsPrintfCString(\"%\" PRIi32, lifetime.mTag);"];
        parse(&samples, false, false);
    }

    #[test]
    fn test_fn_qm_try_inspect_cpp() {
        let samples = vec!["QM_TRY_INSPECT(const int32_t& storageVersion, MOZ_TO_RESULT_INVOKE(aConnection, GetSchemaVersion));"];
        parse(&samples, false, false);
    }
}
