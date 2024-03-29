use std::error::Error;
use std::ops::Range;
use std::sync::Arc;

use crate::from_proto;

use ide::LineIndex;
use ide_db::base_db::Canceled;
use lsp_server::Notification;

pub(crate) fn is_canceled(e: &(dyn Error + 'static)) -> bool {
    e.downcast_ref::<Canceled>().is_some()
}

pub(crate) fn notification_is<N: lsp_types::notification::Notification>(
    notification: &Notification,
) -> bool {
    notification.method == N::METHOD
}

pub(crate) fn apply_document_changes(
    old_text: &mut String,
    content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) {
    let mut line_index = Arc::new(LineIndex::new(old_text));

    // The changes we got must be applied sequentially, but can cross lines so we
    // have to keep our line index updated.
    // Some clients (e.g. Code) sort the ranges in reverse. As an optimization, we
    // remember the last valid line in the index and only rebuild it if needed.
    // The VFS will normalize the end of lines to `\n`.
    enum IndexValid {
        All,
        UpToLineExclusive(u32),
    }

    impl IndexValid {
        fn covers(&self, line: u32) -> bool {
            match *self {
                IndexValid::UpToLineExclusive(to) => to > line,
                _ => true,
            }
        }
    }

    let mut index_valid = IndexValid::All;
    for change in content_changes {
        match change.range {
            Some(range) => {
                if !index_valid.covers(range.end.line) {
                    line_index = Arc::new(LineIndex::new(&old_text));
                }
                index_valid = IndexValid::UpToLineExclusive(range.start.line);
                let range = from_proto::text_range(&line_index, range);
                old_text.replace_range(Range::<usize>::from(range), &change.text);
            }
            None => {
                *old_text = change.text;
                index_valid = IndexValid::UpToLineExclusive(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    #[test]
    fn test_apply_document_changes() {
        macro_rules! c {
            [$($sl:expr, $sc:expr; $el:expr, $ec:expr => $text:expr),+] => {
                vec![$(TextDocumentContentChangeEvent {
                    range: Some(Range {
                        start: Position { line: $sl, character: $sc },
                        end: Position { line: $el, character: $ec },
                    }),
                    range_length: None,
                    text: String::from($text),
                }),+]
            };
        }

        let mut text = String::new();
        apply_document_changes(&mut text, vec![]);
        assert_eq!(text, "");
        apply_document_changes(
            &mut text,
            vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: String::from("the"),
            }],
        );
        assert_eq!(text, "the");
        apply_document_changes(&mut text, c![0, 3; 0, 3 => " quick"]);
        assert_eq!(text, "the quick");
        apply_document_changes(&mut text, c![0, 0; 0, 4 => "", 0, 5; 0, 5 => " foxes"]);
        assert_eq!(text, "quick foxes");
        apply_document_changes(&mut text, c![0, 11; 0, 11 => "\ndream"]);
        assert_eq!(text, "quick foxes\ndream");
        apply_document_changes(&mut text, c![1, 0; 1, 0 => "have "]);
        assert_eq!(text, "quick foxes\nhave dream");
        apply_document_changes(
            &mut text,
            c![0, 0; 0, 0 => "the ", 1, 4; 1, 4 => " quiet", 1, 16; 1, 16 => "s\n"],
        );
        assert_eq!(text, "the quick foxes\nhave quiet dreams\n");
        apply_document_changes(&mut text, c![0, 15; 0, 15 => "\n", 2, 17; 2, 17 => "\n"]);
        assert_eq!(text, "the quick foxes\n\nhave quiet dreams\n\n");
        apply_document_changes(
            &mut text,
            c![1, 0; 1, 0 => "DREAM", 2, 0; 2, 0 => "they ", 3, 0; 3, 0 => "DON'T THEY?"],
        );
        assert_eq!(text, "the quick foxes\nDREAM\nthey have quiet dreams\nDON'T THEY?\n");
        apply_document_changes(&mut text, c![0, 10; 1, 5 => "", 2, 0; 2, 12 => ""]);
        assert_eq!(text, "the quick \nthey have quiet dreams\n");

        text = String::from("❤️");
        apply_document_changes(&mut text, c![0, 0; 0, 0 => "a"]);
        assert_eq!(text, "a❤️");

        text = String::from("a\nb");
        apply_document_changes(&mut text, c![0, 1; 1, 0 => "\nțc", 0, 1; 1, 1 => "d"]);
        assert_eq!(text, "adcb");

        text = String::from("a\nb");
        apply_document_changes(&mut text, c![0, 1; 1, 0 => "ț\nc", 0, 2; 0, 2 => "c"]);
        assert_eq!(text, "ațc\ncb");
    }
}
