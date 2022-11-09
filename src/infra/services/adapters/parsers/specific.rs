pub struct SpecificLanguageParserService {}

impl SpecificLanguageParserService {
    pub fn new() -> Self {
        Self {}
    }

    fn detect_language(&self, language: &Language) -> TsLanguage {
        match language {
            // TODO(dauliac): fix parsers with other tree_sitter parsers
            Languages::Cpp(_) => unsafe { tree_sitter_rust() },
            Languages::Rust(_) => unsafe { tree_sitter_rust() },
            Languages::C(_) => unsafe { tree_sitter_rust() },
            Languages::Java(_) => unsafe { tree_sitter_rust() },
        }
    }
    fn iter_on_nodes() {}
}
