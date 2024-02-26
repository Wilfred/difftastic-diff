use std::collections::HashSet;

/// Configuration for a tree-sitter parser.
pub struct TreeSitterConfig {
    /// The tree-sitter language parser.
    pub(crate) language: tree_sitter::Language,

    /// Tree-sitter nodes that we treat as indivisible atoms.
    ///
    /// This is particularly useful for strings, as some grammars use
    /// several nodes for a single string literal. We don't want to
    /// say e.g. the closing string delimiter moved, as it's confusing
    /// and not well-balanced syntax.
    ///
    /// This is also useful for when tree-sitter nodes don't include
    /// all the children in the source. This is known limitation of
    /// tree-sitter, and occurs more often for complex string syntax.
    /// <https://github.com/tree-sitter/tree-sitter/issues/1156>
    atom_nodes: HashSet<&'static str>,

    /// We want to consider delimiter tokens as part of lists, not
    /// standalone atoms. Tree-sitter includes delimiter tokens, so
    /// mark which token pairs we consider to be delimiters.
    delimiter_tokens: Vec<(&'static str, &'static str)>,

    /// Tree-sitter query used for syntax highlighting this
    /// language.
    highlight_query: tree_sitter::Query,
    // /// Sub-languages in use, if any.
    // sub_languages: Vec<TreeSitterSubLanguage>,
}
