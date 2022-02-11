use tree_sitter::Language;

extern "C" {
    fn tree_sitter_haskell() -> Language;
}

/// Get the tree-sitter [Language][] for this grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language() -> Language {
    unsafe { tree_sitter_haskell() }
}

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &'static str = include_str!("../languages/tree-sitter-haskell/src/node-types.json");

// Uncomment these to include any queries that this grammar contains

pub const HIGHLIGHT_QUERY: &'static str = include_str!("../languages/tree-sitter-haskell/queries/highlights.scm");
// pub const INJECTIONS_QUERY: &'static str = include_str!("../languages/tree-sitter-haskell/queries/injections.scm");
 pub const LOCALS_QUERY: &'static str = include_str!("../languages/tree-sitter-haskell/queries/locals.scm");
// pub const TAGS_QUERY: &'static str = include_str!("../languages/tree-sitter-haskell/queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language())
            .expect("Error loading haskell language");
    }
}
