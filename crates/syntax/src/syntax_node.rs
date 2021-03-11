use rowan::{GreenNodeBuilder, Language};

use crate::SyntaxKind;

pub(crate) use rowan::{GreenNode, GreenToken, NodeOrToken};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustLanguage {}
impl Language for RustLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<RustLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<RustLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<RustLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<RustLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<RustLanguage>;

// #[derive(Default)]
// pub struct SyntaxTreeBuilder {
//     errors: Vec<SyntaxError>,
//     inner: GreenNodeBuilder<'static>,
// }

// impl SyntaxTreeBuilder {
//     pub(crate) fn finish_raw(self) -> (GreenNode, Vec<SyntaxError>) {
//         let green = self.inner.finish();
//         (green, self.errors)
//     }

//     pub fn finish(self) -> Parse<SyntaxNode> {
//         let (green, errors) = self.finish_raw();
//         if cfg!(debug_assertions) {
//             let node = SyntaxNode::new_root(green.clone());
//             crate::validation::validate_block_structure(&node);
//         }
//         Parse::new(green, errors)
//     }

//     pub fn token(&mut self, kind: SyntaxKind, text: &str) {
//         let kind = RustLanguage::kind_to_raw(kind);
//         self.inner.token(kind, text)
//     }

//     pub fn start_node(&mut self, kind: SyntaxKind) {
//         let kind = RustLanguage::kind_to_raw(kind);
//         self.inner.start_node(kind)
//     }

//     pub fn finish_node(&mut self) {
//         self.inner.finish_node()
//     }

//     pub fn error(&mut self, error: parser::ParseError, text_pos: TextSize) {
//         self.errors.push(SyntaxError::new_at_offset(*error.0, text_pos))
//     }
// }
