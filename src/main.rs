use tree_sitter::Parser;
use tree_sitter_fuse_rs::external::*;

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_javascript() };

    parser.set_language(language).unwrap();
}
