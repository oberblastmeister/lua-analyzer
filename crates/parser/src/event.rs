use crate::{ParseErrorKind, SyntaxKind};

pub enum Event {
    Start {
        kind: SyntaxKind,
        forward_parent: Option<u32>,
    },

    Finish,

    Token(SyntaxKind),

    Error(ParseErrorKind),
}

impl Event {
    pub(crate) fn tombstone() -> Event {
        Event::Start {
            kind: T![__],
            forward_parent: None,
        }
    }
}
