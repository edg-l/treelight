use tree_sitter::Language;

extern "C" {
    fn tree_sitter_typescript() -> Language;
    fn tree_sitter_tsx() -> Language;
}

/// Returns the tree-sitter [Language][] for this Typescript.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_typescript() -> Language {
    unsafe { tree_sitter_typescript() }
}

/// Returns the tree-sitter [Language][] for TSX.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_tsx() -> Language {
    unsafe { tree_sitter_tsx() }
}

/// The syntax highlighting query for this language.
pub const HIGHLIGHT_QUERY: &str =
    include_str!("../languages/tree-sitter-typescript/queries/highlights.scm");

/// The local-variable syntax highlighting query for this language.
pub const LOCALS_QUERY: &str =
    include_str!("../languages/tree-sitter-typescript/queries/locals.scm");

/// The symbol tagging query for this language.
pub const TAGGING_QUERY: &str =
    include_str!("../languages/tree-sitter-typescript/queries/tags.scm");

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const TYPESCRIPT_NODE_TYPES: &str =
    include_str!("../languages/tree-sitter-typescript/typescript/src/node-types.json");
pub const TSX_NODE_TYPES: &str =
    include_str!("../languages/tree-sitter-typescript/tsx/src/node-types.json");

#[cfg(test)]
mod tests {
    #[test]
    fn can_load_grammar_typescript() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language_typescript())
            .expect("Error loading Java grammar");
    }

    #[test]
    fn can_load_grammar_tsx() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language_tsx())
            .expect("Error loading Java grammar");
    }
}
