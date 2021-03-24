mod text_token_source;
mod text_tree_sink;

use parser::{ParseError, tokenize};
use rowan::GreenNode;
use text_token_source::TextTokenSource;
use text_tree_sink::TextTreeSink;

pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<ParseError>) {
    let (tokens, lexer_errors) = tokenize(&text);

    let mut token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);

    todo!()
    // parser::parse(&mut token_source, &mut tree_sink);

    // let (tree, mut parser_errors) = tree_sink.finish();
    // parser_errors.extend(lexer_errors);

    // (tree, parser_errors)
}
