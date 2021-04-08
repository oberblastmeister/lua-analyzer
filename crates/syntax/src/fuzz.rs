use crate::ast::Program;

pub fn check_parser(text: &str) {
    let program = Program::parse(text);
}
