use parser::{Reparser, Token};
use rowan::TextSize;
use text_edit::Indel;

use crate::{
    lex_first_syntax_kind,
    parsing::{text_token_source::TextTokenSource, text_tree_sink::TextTreeSink},
    syntax_node::{GreenToken, NodeOrToken},
    tokenize, GreenNode, SyntaxElement, SyntaxError, SyntaxNode, TextRange, N, T,
};

pub(crate) fn incremental_reparse(
    node: &SyntaxNode,
    edit: &Indel,
    errors: Vec<SyntaxError>,
) -> Option<(GreenNode, Vec<SyntaxError>, TextRange)> {
    if let Some((green, new_errors, old_range)) = reparse_token(node, &edit) {
        return Some((green, merge_errors(errors, new_errors, old_range, edit), old_range));
    }

    if let Some((green, new_errors, old_range)) = reparse_block(node, &edit) {
        return Some((green, merge_errors(errors, new_errors, old_range, edit), old_range));
    }

    None
}

fn reparse_token(
    root: &SyntaxNode,
    edit: &Indel,
) -> Option<(GreenNode, Vec<SyntaxError>, TextRange)> {
    let prev_token = root.covering_element(edit.delete).as_token()?.clone();
    let prev_token_kind = prev_token.kind();
    match prev_token_kind {
        T![whitespace] | T![comment] | T![ident] | T![str] => {
            if prev_token_kind == T![whitespace] || prev_token_kind == T![comment] {
                // removing a new line may extends previous token
                let deleted_range = edit.delete - prev_token.text_range().start();
                if prev_token.text()[deleted_range].contains('\n') {
                    return None;
                }
            }

            let mut new_text = get_text_after_edit(prev_token.clone().into(), &edit);
            let (new_token_kind, new_err) = lex_first_syntax_kind(&new_text)?;

            if new_token_kind != prev_token_kind {
                return None;
            }

            let new_token = GreenToken::new(rowan::SyntaxKind(prev_token_kind.into()), &new_text);
            Some((
                prev_token.replace_with(new_token),
                new_err.into_iter().collect(),
                prev_token.text_range(),
            ))
        }
        _ => None,
    }
}

fn reparse_block(
    root: &SyntaxNode,
    edit: &Indel,
) -> Option<(GreenNode, Vec<SyntaxError>, TextRange)> {
    let (node, reparser) = find_reparsable_node(root, edit.delete)?;
    let text = get_text_after_edit(node.clone().into(), edit);

    let (tokens, new_lexer_errors) = tokenize(&text);
    if !is_balanced(&tokens) {
        return None;
    }

    let mut token_source = TextTokenSource::new(&text, &tokens);
    let mut tree_sink = TextTreeSink::new(&text, &tokens);
    reparser.parse(&mut token_source, &mut tree_sink);

    let (green, mut new_parser_errors) = tree_sink.finish();
    new_parser_errors.extend(new_lexer_errors);

    Some((node.replace_with(green), new_parser_errors, node.text_range()))
}

fn get_text_after_edit(element: SyntaxElement, edit: &Indel) -> String {
    let edit = Indel::replace(edit.delete - element.text_range().start(), edit.insert.clone());

    let mut text = match element {
        NodeOrToken::Token(token) => token.text().to_string(),
        NodeOrToken::Node(node) => node.text().to_string(),
    };
    edit.apply(&mut text);
    text
}

fn find_reparsable_node(node: &SyntaxNode, range: TextRange) -> Option<(SyntaxNode, Reparser)> {
    let node = node.covering_element(range);

    node.ancestors().find_map(|node| Reparser::for_node(node.kind()).map(|r| (node, r)))
}

fn is_balanced(tokens: &[Token]) -> bool {
    if tokens.is_empty() || tokens.last().unwrap().kind != T![end] {
        return false;
    }
    true
}

fn merge_errors(
    old_errors: Vec<SyntaxError>,
    new_errors: Vec<SyntaxError>,
    range_before_reparse: TextRange,
    edit: &Indel,
) -> Vec<SyntaxError> {
    let mut res = Vec::new();

    for old_err in old_errors {
        let old_err_range = old_err.range();

        // checking if errors were before or after the reparse
        // we do not keep errors inside of the reparse
        if old_err_range.end() <= range_before_reparse.start() {
            res.push(old_err);
        } else if old_err_range.start() >= range_before_reparse.end() {
            res.push(old_err.with_range(old_err_range + edit.offset()));
        }
    }

    res.extend(new_errors.into_iter().map(|new_err| {
        let offseted_range = new_err.range() + range_before_reparse.start();
        new_err.with_range(offseted_range)
    }));

    res
}

#[cfg(test)]
mod tests {
    use crate::{AstNode, Parse, SourceFile};
    use test_utils::{assert_eq_text, extract_range};

    use super::*;

    fn check(before: &str, replace_with: &str, reparsed_len: u32) {
        let (range, before) = extract_range(before);
        let edit = Indel::replace(range, replace_with.to_owned());
        let after = {
            let mut after = before.clone();
            edit.apply(&mut after);
            after
        };

        let fully_reparsed = SourceFile::parse(&after);
        let incrementally_reparsed: Parse<SourceFile> = {
            let before = SourceFile::parse(&before);
            let (green, new_errors, range) =
                incremental_reparse(before.tree().syntax(), &edit, before.errors.to_vec()).unwrap();
            assert_eq!(range.len(), reparsed_len.into(), "reparsed fragment has wrong length");
            Parse::new(green, new_errors)
        };

        assert_eq_text!(
            &format!("{:#?}", fully_reparsed.tree().syntax()),
            &format!("{:#?}", incrementally_reparsed.tree().syntax()),
        );
        assert_eq!(fully_reparsed.errors(), incrementally_reparsed.errors());
    }

    #[test]
    fn reparse_tokens() {
        check(
            r"
local function $0foo$0()
  return 1
end
              ",
            "bar",
            3,
        );

        check(
            r#"
--$0$0
local hello = "world"
                  "#,
            "some comment",
            2,
        );

        check(
            r"
-- $0$0omment
              ",
            "c",
            9,
        );

        check(
            r#"
local person = "$0$0"
              "#,
            "my name",
            2,
        );

        check(r"print($0$0)", r"'hello'", 0);
    }
}
