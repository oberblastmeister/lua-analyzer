use crate::{ast::SourceFile, lexer::tokenize};

pub fn check_parser(text: &str) {
    let _ = SourceFile::parse(text);
}

pub fn check_lexer(text: &str) {
    let _ = tokenize(text);
}
