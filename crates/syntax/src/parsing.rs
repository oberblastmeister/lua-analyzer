mod text_token_source;
mod text_tree_sink;

use itertools::{Either, Itertools};
use parser::{tokenize_iter, Token};
use rowan::GreenNode;
use text_token_source::TextTokenSource;
use text_tree_sink::TextTreeSink;

use crate::SyntaxError;

pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let (tokens, errors): (Vec<Token>, Vec<SyntaxError>) =
        tokenize_iter(&text).partition_map(|r| match r {
            Ok(v) => Either::Left(v),
            Err(v) => Either::Right(v.into()),
        });

    let token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);

    parser::parse(token_source, &mut tree_sink);

    let (tree, mut parser_errors) = tree_sink.finish();
    parser_errors.extend(errors);

    (tree, parser_errors)
}
