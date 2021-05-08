use crate::{
    ast::SourceFile,
    lexer::{tokenize, Lexer},
};

pub fn check_parser(text: &str) {
    let _ = SourceFile::parse(text);
}

pub fn check_lexer<'a, L>(text: &'a str)
where
    L: Lexer<'a>,
{
    let _ = tokenize::<L>(text);
}
