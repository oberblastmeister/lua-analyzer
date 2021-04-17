mod text_token_source;
mod text_tree_sink;

use crate::lexer::tokenize;
use rowan::GreenNode;
use text_token_source::TextTokenSource;
use text_tree_sink::TextTreeSink;

use crate::SyntaxError;

pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let (tokens, errors) = tokenize(text);

    let mut token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);

    parser::parse(&mut token_source, &mut tree_sink);

    let (tree, mut parser_errors) = tree_sink.finish();
    parser_errors.extend(errors);

    (tree, parser_errors)
}
