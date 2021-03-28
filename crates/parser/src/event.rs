use std::mem;

use crate::{ParseError, SyntaxKind, Token, TreeSink};

/// TODO: add node error event
pub enum Event {
    Start {
        kind: SyntaxKind,
        forward_parent: Option<u32>,
    },

    Finish,

    StartError,

    FinishError(ParseError),

    Token,

    Error(ParseError),
    // Error {
    //     e: ParseError,
    //     token_end: Option<u32>,
    // }
}

impl Event {
    pub(crate) fn tombstone() -> Event {
        Event::Start {
            kind: T![__],
            forward_parent: None,
        }
    }
}

pub(super) fn process<TS: TreeSink>(mut sink: TS, mut events: Vec<Event>) {
    let mut forward_parents = Vec::new();

    for i in 0..events.len() {
        match mem::replace(&mut events[i], Event::tombstone()) {
            Event::Start { kind: T![__], .. } => (),
            Event::Start {
                kind,
                forward_parent,
            } => {
                // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
                // in the normal control flow, the parent-child relation: `A -> B -> C`,
                // while with the magic forward_parent, it writes: `C <- B <- A`.

                // append `A` into parents.
                forward_parents.push(kind);
                let mut idx = i;
                let mut fp = forward_parent;
                while let Some(fwd) = fp {
                    idx += fwd as usize;
                    // append `A`'s forward_parent `B`
                    fp = match mem::replace(&mut events[idx], Event::tombstone()) {
                        Event::Start {
                            kind,
                            forward_parent,
                        } => {
                            if kind != T![__] {
                                forward_parents.push(kind);
                            }
                            forward_parent
                        }
                        _ => unreachable!(),
                    };
                    // append `B`'s forward_parent `C` in the next stage.
                }

                for kind in forward_parents.drain(..).rev() {
                    sink.start_node(kind);
                }
            }
            Event::Error(e) => sink.error(e),
            Event::Finish => sink.finish_node(),
            Event::Token => sink.token(),
            Event::StartError => sink.start_error_node(),
            Event::FinishError(e) => sink.finish_error_node(e)
        }
    }
}
