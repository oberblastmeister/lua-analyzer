use std::{
    hash::{Hash, Hasher},
    iter::successors,
    marker::PhantomData,
};

use parser::SyntaxKind;
use rowan::TextRange;

use crate::{ast::AstNode, SyntaxNode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNodePtr {
    pub(crate) range: TextRange,
    kind: SyntaxKind,
}

impl SyntaxNodePtr {
    pub fn new(node: &SyntaxNode) -> SyntaxNodePtr {
        SyntaxNodePtr { range: node.text_range(), kind: node.kind() }
    }

    pub fn to_node(&self, root: &SyntaxNode) -> SyntaxNode {
        assert!(root.parent().is_none());
        successors(Some(root.clone()), |node| {
            node.children().find(|it| it.text_range().contains_range(self.range))
        })
        .find(|it| it.text_range() == self.range && it.kind() == self.kind)
        .unwrap_or_else(|| panic!("can't resolve local ptr to SyntaxNode: {:?}", self))
    }

    pub fn cast<N: AstNode>(self) -> Option<AstPtr<N>> {
        if !N::can_cast(self.kind) {
            return None;
        }
        Some(AstPtr { raw: self, _ty: PhantomData })
    }
}

pub struct AstPtr<T> {
    raw: SyntaxNodePtr,
    _ty: PhantomData<fn() -> T>,
}

impl<N: AstNode> Clone for AstPtr<N> {
    fn clone(&self) -> AstPtr<N> {
        AstPtr { raw: self.raw.clone(), _ty: PhantomData }
    }
}

impl<N: AstNode> Eq for AstPtr<N> {}

impl<N: AstNode> PartialEq for AstPtr<N> {
    fn eq(&self, other: &AstPtr<N>) -> bool {
        self.raw == other.raw
    }
}

impl<N: AstNode> Hash for AstPtr<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}

impl<N: AstNode> AstPtr<N> {
    pub fn new(node: &N) -> AstPtr<N> {
        AstPtr { raw: SyntaxNodePtr::new(node.syntax()), _ty: PhantomData }
    }

    pub fn to_node(&self, root: &SyntaxNode) -> N {
        let syntax_node = self.raw.to_node(root);
        N::cast(syntax_node).unwrap()
    }

    pub fn syntax_node_ptr(&self) -> SyntaxNodePtr {
        self.raw.clone()
    }

    pub fn cast<U: AstNode>(self) -> Option<AstPtr<U>> {
        if !U::can_cast(self.raw.kind) {
            return None;
        }
        Some(AstPtr { raw: self.raw, _ty: PhantomData })
    }
}

#[test]
fn test_syntax_ptr() {
    use crate::{ast, AstNode, SourceFile};

    let file = SourceFile::parse("local function foo(n) end").ok().unwrap();
    let function = file.syntax().descendants().find_map(ast::LocalFunctionDefStmt::cast).unwrap();
    let ptr = SyntaxNodePtr::new(function.syntax());
    let function_syntax = ptr.to_node(file.syntax());
    assert_eq!(function.syntax(), &function_syntax);
}
