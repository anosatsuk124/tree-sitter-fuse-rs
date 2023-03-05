use tree_sitter::Language;

extern "C" {
    pub fn tree_sitter_javascript() -> Language;
    pub fn tree_sitter_fuse_rust() -> Language;
}
