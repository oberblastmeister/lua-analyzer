use crate::{ast::Program, lexer::tokenize};

pub fn check_parser(text: &str) {
    let _ = Program::parse(text);
}

pub fn check_lexer(text: &str) {
    let _ = tokenize(text);
}
