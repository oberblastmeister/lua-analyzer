use std::any::type_name;
use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
};

use la_arena::{Arena, Idx};
use syntax::{
    ast::{self, AstNode},
    match_ast, AstPtr, SyntaxNode, SyntaxNodePtr,
};

pub struct FileAstId<N: AstNode> {
    raw: ErasedFileAstId,
    _ty: PhantomData<fn() -> N>,
}

impl<N: AstNode> Clone for FileAstId<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N: AstNode> Copy for FileAstId<N> {}

impl<N: AstNode> PartialEq for FileAstId<N> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<N: AstNode> Eq for FileAstId<N> {}

impl<N: AstNode> Hash for FileAstId<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}

impl<N: AstNode> fmt::Debug for FileAstId<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileAstId::<{}>({})", type_name::<N>(), self.raw.into_raw())
    }
}

impl<N: AstNode> FileAstId<N> {
    // Can't make this a From implementation because of coherence
    pub fn upcast<M: AstNode>(self) -> FileAstId<M>
    where
        N: Into<M>,
    {
        FileAstId { raw: self.raw, _ty: PhantomData }
    }
}

type ErasedFileAstId = Idx<SyntaxNodePtr>;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct AstIdMap {
    arena: Arena<SyntaxNodePtr>,
}

impl AstIdMap {
    pub(crate) fn from_source(node: &SyntaxNode) -> AstIdMap {
        assert!(node.parent().is_some());
        let mut res = AstIdMap::default();
        bdfs(node, |it| {
            match_ast! {
                match it {
                    ast::StmtItem(stmt) => {
                        res.alloc(stmt.syntax());
                        true
                    },
                    ast::Block(block) => {
                        res.alloc(block.syntax());
                        true
                    },
                    _ => false
                }
            }
        });
        res
    }

    pub fn ast_id<N: AstNode>(&self, item: &N) -> FileAstId<N> {
        let raw = self.erased_ast_id(item.syntax());
        FileAstId { raw, _ty: PhantomData }
    }

    fn erased_ast_id(&self, item: &SyntaxNode) -> ErasedFileAstId {
        let ptr = SyntaxNodePtr::new(item);
        match self.arena.iter().find(|(_id, i)| **i == ptr) {
            Some((it, _)) => it,
            None => panic!(
                "Can't find {:?} in AstIdMap:\n{:?}",
                item,
                self.arena.iter().map(|(_id, i)| i).collect::<Vec<_>>()
            ),
        }
    }

    pub fn get<N: AstNode>(&self, id: FileAstId<N>) -> AstPtr<N> {
        self.arena[id.raw].clone().cast::<N>().unwrap()
    }

    fn alloc(&mut self, stmt: &SyntaxNode) -> ErasedFileAstId {
        self.arena.alloc(SyntaxNodePtr::new(stmt))
    }
}

/// Walks the subtree in bdfs order, calling `f` for each node. What is bdfs
/// order? It is a mix of breadth-first and depth first orders. Nodes for which
/// `f` returns true are visited breadth-first, all the other nodes are explored
/// depth-first.
///
/// In other words, the size of the bfs queue is bound by the number of "true"
/// nodes.
fn bdfs(node: &SyntaxNode, mut f: impl FnMut(SyntaxNode) -> bool) {
    let mut curr_layer = vec![node.clone()];
    let mut next_layer = vec![];
    while !curr_layer.is_empty() {
        curr_layer.drain(..).for_each(|node| {
            let mut preorder = node.preorder();
            while let Some(event) = preorder.next() {
                match event {
                    syntax::WalkEvent::Enter(node) => {
                        if f(node.clone()) {
                            next_layer.extend(node.children());
                            preorder.skip_subtree();
                        }
                    }
                    syntax::WalkEvent::Leave(_) => {}
                }
            }
        });
        mem::swap(&mut curr_layer, &mut next_layer);
    }
}

#[test]
#[ignore]
fn learning() {
    let prog = ast::SourceFile::parse(
        r#"
local function testing(first, second, third)
    local new = first + second
    return new^324 .. "adfadsf"

    local function another(first)
    end
end
"#,
    )
    .ok()
    .unwrap();

    bdfs(prog.syntax(), |it| {
        match_ast! {
            match it {
                ast::StmtItem(function) => {
                    println!("Yes: {:#?}", function);
                    true
                },
                // ast::LocalFunctionDefStmt(function) => {
                //     println!("Yes: {:#?}", function);
                //     true
                // }
                ast::Block(block) => {
                    println!("Yes: {:#?}", block);
                    true
                },
                _ => {
                    println!("No: {:#?}", it);
                    false
                },
            }
        }
    });

    panic!("just for learning");
}
