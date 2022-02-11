use tree_sitter::Language;

extern "C" {
    fn tree_sitter_javascript() -> Language;
}

/// Returns the tree-sitter [Language][] for this grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language() -> Language {
    unsafe { tree_sitter_javascript() }
}

/// The source of the JavaScript tree-sitter grammar description.
pub const GRAMMAR: &'static str = include_str!("../languages/tree-sitter-javascript/grammar.js");

/// The syntax highlighting query for this language.
pub const HIGHLIGHT_QUERY: &'static str = include_str!("../languages/tree-sitter-javascript/queries/highlights.scm");

/// The syntax highlighting query for languages injected into this one.
pub const INJECTION_QUERY: &'static str = include_str!("../languages/tree-sitter-javascript/queries/injections.scm");

/// The syntax highlighting query for JSX.
pub const JSX_HIGHLIGHT_QUERY: &'static str = include_str!("../languages/tree-sitter-javascript/queries/highlights-jsx.scm");

/// The local-variable syntax highlighting query for this language.
pub const LOCALS_QUERY: &'static str = include_str!("../languages/tree-sitter-javascript/queries/locals.scm");

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &'static str = include_str!("../languages/tree-sitter-javascript/src/node-types.json");

/// The symbol tagging query for this language.
pub const TAGGING_QUERY: &'static str = include_str!("../languages/tree-sitter-javascript/queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language())
            .expect("Error loading JavaScript grammar");
    }
}
