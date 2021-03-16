use drop_bomb::DropBomb;

use crate::{Event, ParseErrorKind, SyntaxKind, TokenSet, TokenSource, T};

pub(crate) struct Parser<T: TokenSource> {
    token_source: T,
    events: Vec<Event>,
}

impl<T: TokenSource> Parser<T> {
    pub fn new(token_source: T) -> Parser<T> {
        Parser {
            token_source,
            events: Vec::new(),
        }
    }

    pub(crate) fn finish(self) -> Vec<Event> {
        self.events
    }

    fn nth(&self, n: usize) -> SyntaxKind {
        self.token_source.lookahead_nth(n).kind
    }

    /// Returns the kind of the current token.
    /// If parser has already reached the end of input,
    /// the special `EOF` kind is returned.
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    fn events_len(&self) -> u32 {
        self.events.len() as u32
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events_len();
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }

    fn error(&mut self, message: &'static str) {
        self.push_event(Event::Error(ParseErrorKind::Message(message)))
    }

    pub(crate) fn at_ts(&self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    pub(crate) fn bump_any(&mut self) {
        let kind = self.nth(0);
        if kind == T![eof] {
            return;
        }
        self.do_bump(kind)
    }

    /// Create an error node and consume the next token.
    pub(crate) fn err_recover(&mut self, message: &'static str, recovery: TokenSet) {
        if self.at_ts(recovery) {
            self.error(message);
            return;
        }

        let m = self.start();
        self.error(message);
        self.bump_any();
        m.complete(self, T![error]);
    }

    fn do_bump(&mut self, kind: SyntaxKind) {
        self.token_source.bump();

        self.push_event(Event::Token(kind));
    }

    fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.token_source.lookahead_nth(n).kind == kind
    }

    /// Checks if the current token is `kind`.
    fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    fn accept(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.do_bump(kind);
        true
    }

    /// Consume the next token if it is `kind` or emit an error
    /// otherwise.
    fn expect(&mut self, kind: SyntaxKind) -> bool {
        let current = self.current();
        if current == kind {
            return true;
        }
        self.push_event(Event::Error(ParseErrorKind::expected(kind, current)));
        false
    }
}

pub(crate) struct Marker {
    pos: u32,
    bomb: DropBomb,
}

impl Marker {
    fn new(pos: u32) -> Marker {
        Marker {
            pos,
            bomb: DropBomb::new("Marker must be either completed or abandoned"),
        }
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with forward_parent.
    pub(crate) fn complete<T: TokenSource>(
        mut self,
        p: &mut Parser<T>,
        kind: SyntaxKind,
    ) -> CompletedMarker {
        self.bomb.defuse();
        let idx = self.pos as usize;
        match &mut p.events[idx] {
            Event::Start { kind: slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        let finish_pos = p.events.len() as u32;
        p.push_event(Event::Finish);
        CompletedMarker::new(self.pos, finish_pos, kind)
    }

    /// Abandons the syntax tree node. All its children
    /// are attached to its parent instead.
    pub(crate) fn abandon<T: TokenSource>(mut self, p: &mut Parser<T>) {
        self.bomb.defuse();
        let idx = self.pos as usize;
        if idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::Start {
                    kind: T![__],
                    forward_parent: None,
                }) => (),
                _ => unreachable!(),
            }
        }
    }
}

pub(crate) struct CompletedMarker {
    start_pos: u32,
    finish_pos: u32,
    kind: SyntaxKind,
}

impl CompletedMarker {
    fn new(start_pos: u32, finish_pos: u32, kind: SyntaxKind) -> Self {
        CompletedMarker {
            start_pos,
            finish_pos,
            kind,
        }
    }

    /// This method allows to create a new node which starts
    /// *before* the current one. That is, parser could start
    /// node `A`, then complete it, and then after parsing the
    /// whole `A`, decide that it should have started some node
    /// `B` before starting `A`. `precede` allows to do exactly
    /// that. See also docs about `forward_parent` in `Event::Start`.
    ///
    /// Given completed events `[START, FINISH]` and its corresponding
    /// `CompletedMarker(pos: 0, _)`.
    /// Append a new `START` events as `[START, FINISH, NEWSTART]`,
    /// then mark `NEWSTART` as `START`'s parent with saving its relative
    /// distance to `NEWSTART` into forward_parent(=2 in this case);
    pub(crate) fn precede<T: TokenSource>(self, p: &mut Parser<T>) -> Marker {
        let new_pos = p.start();
        let idx = self.start_pos as usize;
        match &mut p.events[idx] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(new_pos.pos - self.start_pos);
            }
            _ => unreachable!(),
        }
        new_pos
    }

    /// Undo this completion and turns into a `Marker`
    pub(crate) fn undo_completion<T: TokenSource>(self, p: &mut Parser<T>) -> Marker {
        let start_idx = self.start_pos as usize;
        let finish_idx = self.finish_pos as usize;
        match &mut p.events[start_idx] {
            Event::Start {
                kind,
                forward_parent: None,
            } => *kind = T![__],
            _ => unreachable!(),
        }
        match &mut p.events[finish_idx] {
            slot @ Event::Finish => *slot = Event::tombstone(),
            _ => unreachable!(),
        }
        Marker::new(self.start_pos)
    }

    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
}
