mod reparsing;
mod text_token_source;
mod text_tree_sink;

pub(crate) use reparsing::incremental_reparse;

use crate::{lexer::tokenize, lua_lexer::LuaLexer};
use rowan::GreenNode;
use text_token_source::TextTokenSource;

use crate::SyntaxError;

use self::text_tree_sink::DeferedTextTreeSink;

const DEFER_AMOUNT: u8 = 2;

pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let (tokens, errors) = tokenize::<LuaLexer>(text);

    let mut token_source = TextTokenSource::new(text, &tokens);
    let tree_sink = DeferedTextTreeSink::new(text, &tokens, DEFER_AMOUNT);

    let (tree, mut parser_errors) = parser::parse(&mut token_source, tree_sink);

    parser_errors.extend(errors);

    (tree, parser_errors)
}
