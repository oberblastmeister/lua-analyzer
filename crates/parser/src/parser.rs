use std::{fmt, marker::PhantomData};

use accept::{Acceptor, Advancer, Any};
use drop_bomb::DropBomb;

use crate::{assert_matches, Event, ParseError, SyntaxKind, Token, TokenSet, TokenSource, T, TS};

const RECOVERY: TokenSet = TS![end];

pub struct Parser<'a> {
    token_source: &'a mut dyn TokenSource,
    events: Vec<Event>,
}

impl Advancer for Parser<'_> {
    type Item = SyntaxKind;

    fn advance(&mut self) -> Option<SyntaxKind> {
        self.push_event(Event::Token);
        self.token_source.bump().map(|Token { kind, .. }| kind)
    }

    fn nth(&self, n: u32) -> SyntaxKind {
        let Token { kind, .. } = self.token_source.lookahead_nth(n as usize);
        kind
    }

    fn nth_is_eof(&self, n: u32) -> bool {
        self.nth(n) == T![eof]
    }
}

impl<'a> Parser<'a> {
    pub fn new(token_source: &mut dyn TokenSource) -> Parser {
        Parser { token_source, events: Vec::new() }
    }

    pub(crate) fn finish(self) -> Vec<Event> {
        self.events
    }

    fn events_len(&self) -> u32 {
        self.events.len() as u32
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    pub(crate) fn start(&mut self) -> Marker<RegularMarker> {
        let pos = self.events_len();
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    pub(crate) fn start_error(&mut self) -> Marker<ErrorMarker> {
        let pos = self.events_len();
        self.push_event(Event::StartError);
        Marker::new(pos)
    }

    fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }

    pub(crate) fn error<S: fmt::Display>(&mut self, message: S) {
        self.push_event(Event::Error(ParseError::Message(message.to_string())))
    }

    // pub(crate) fn at(&self, kinds: TokenSet) -> bool {
    //     kinds.contains(self.current())
    // }

    /// Create an error node and consume the next token.
    pub(crate) fn err_recover<S: fmt::Display>(&mut self, message: S, recovery: TokenSet) {
        match self.current() {
            T![end] => {
                self.error(message.to_string());
                return;
            }
            _ => (),
        }

        if self.at(recovery) {
            self.error(message);
            return;
        }

        let m = self.start_error();
        self.accept(Any);
        m.complete(self, ParseError::Message(message.to_string()));
    }

    pub(crate) fn err_and_bump(&mut self, message: &'static str) {
        self.err_recover(message, TS![])
    }

    pub(crate) fn err_until(&mut self, message: &'static str, recovery: TokenSet) {
        let recovery = recovery.union(TS![end]);

        let m = self.start_error();

        while !self.at(recovery) && !self.at(T![eof]) {
            self.bump(Any)
        }

        m.complete(self, ParseError::Message(message.to_string()));
    }

    /// Checks if the current token is `kind`.
    pub(crate) fn expect_at(&mut self, kind: SyntaxKind) -> bool {
        let current = self.current();
        if current != kind {
            self.push_event(Event::Error(ParseError::expected(kind, current)));
            return false;
        }
        true
    }

    pub(crate) fn bump_ts(&mut self, kinds: TokenSet) {
        if !self.at(kinds) {
            panic!("Failed to bump {:?}, got {:?}", kinds, self.current())
        }
        self.bump(Any);
    }

    /// Consume the next token if it is `kind` or emit an error
    /// otherwise.
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        let current = self.current();
        if current == kind {
            self.bump(Any);
            return true;
        }
        self.push_event(Event::Error(ParseError::expected(kind, current)));
        false
    }
}

pub(crate) trait MarkerType {}

pub(crate) enum ErrorMarker {}
impl MarkerType for ErrorMarker {}

pub(crate) enum RegularMarker {}
impl MarkerType for RegularMarker {}

pub(crate) struct Marker<T: MarkerType> {
    pos: u32,
    bomb: DropBomb,
    _marker_type: PhantomData<T>,
}

impl<T: MarkerType> Marker<T> {
    fn new(pos: u32) -> Marker<T> {
        Marker {
            pos,
            bomb: DropBomb::new("Marker must be either completed or abandoned"),
            _marker_type: PhantomData,
        }
    }
}

pub(crate) type MarkerRegular = Marker<RegularMarker>;

impl Marker<RegularMarker> {
    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with forward_parent.
    pub(crate) fn complete(
        mut self,
        p: &mut Parser,
        kind: SyntaxKind,
    ) -> CompletedMarker<RegularMarker> {
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
    pub(crate) fn abandon(mut self, p: &mut Parser) {
        self.bomb.defuse();
        let idx = self.pos as usize;
        if idx == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::Start { kind: T![__], forward_parent: None }) => (),
                _ => unreachable!(),
            }
        }
    }
}

impl Marker<ErrorMarker> {
    pub(crate) fn complete(
        mut self,
        p: &mut Parser,
        e: ParseError,
    ) -> CompletedMarker<ErrorMarker> {
        self.bomb.defuse();
        let idx = self.pos as usize;
        assert_matches!(p.events[idx], Event::StartError);
        let finish_pos = p.events.len() as u32;
        p.push_event(Event::FinishError(e));
        CompletedMarker::new(self.pos, finish_pos, T![error])
    }
}

pub(crate) struct CompletedMarker<T: MarkerType> {
    start_pos: u32,
    finish_pos: u32,
    kind: SyntaxKind,
    _marker_type: PhantomData<T>,
}

impl<T: MarkerType> CompletedMarker<T> {
    fn new(start_pos: u32, finish_pos: u32, kind: SyntaxKind) -> CompletedMarker<T> {
        CompletedMarker { start_pos, finish_pos, kind, _marker_type: PhantomData }
    }
}

pub(crate) type MarkerComplete = CompletedMarker<RegularMarker>;

impl CompletedMarker<RegularMarker> {
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
    pub(crate) fn precede(self, p: &mut Parser) -> Marker<RegularMarker> {
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
    pub(crate) fn undo_completion(self, p: &mut Parser) -> Marker<RegularMarker> {
        let start_idx = self.start_pos as usize;
        let finish_idx = self.finish_pos as usize;
        match &mut p.events[start_idx] {
            Event::Start { kind, forward_parent: None } => *kind = T![__],
            _ => unreachable!(),
        }
        match &mut p.events[finish_idx] {
            slot @ Event::Finish => *slot = Event::tombstone(),
            _ => unreachable!(),
        }
        Marker::new(self.start_pos)
    }

    pub(crate) fn change_kind(self, p: &mut Parser, kind: SyntaxKind) -> MarkerComplete {
        let start_idx = self.start_pos as usize;
        match &mut p.events[start_idx] {
            Event::Start { kind: old_kind, forward_parent: None } => *old_kind = kind,
            _ => unreachable!(),
        };
        self
    }

    pub(crate) fn kind(&self) -> SyntaxKind {
        self.kind
    }
}
