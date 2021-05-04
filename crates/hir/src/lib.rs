mod ast_id_map;
pub mod body;
pub mod expr;
mod ids;
mod item_tree;
mod name;
mod semantics;
pub mod stmt;
mod with_body;

use std::sync::Arc;

use base_db::{salsa, FileId, SourceDatabase, Upcast};
use syntax::{ast::AstNode, SyntaxNode};

use ast_id_map::AstIdMap;
use ids::{
    FunctionId, FunctionLoc, LocalAssignId, LocalAssignLoc, LocalFunctionId, LocalFunctionLoc,
};
use item_tree::ItemTree;
pub use semantics::Semantics;

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: SourceDatabase {
    #[salsa::interned]
    fn intern_function(&self, loc: FunctionLoc) -> FunctionId;

    #[salsa::interned]
    fn intern_local_function(&self, loc: LocalFunctionLoc) -> LocalFunctionId;

    #[salsa::interned]
    fn intern_local_assign(&self, loc: LocalAssignLoc) -> LocalAssignId;
}

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase: SourceDatabase {
    fn ast_id_map(&self, file_id: FileId) -> Arc<AstIdMap>;

    #[salsa::transparent]
    fn syntax_node(&self, file_id: FileId) -> SyntaxNode;
}

fn syntax_node(db: &dyn SourceDatabase, file_id: FileId) -> SyntaxNode {
    db.parse(file_id).tree().syntax().clone()
}

fn ast_id_map(db: &dyn AstDatabase, file_id: FileId) -> Arc<AstIdMap> {
    let map = db
        .parse(file_id)
        .ok()
        .map_or_else(|_| AstIdMap::default(), |it| AstIdMap::from_source(&it.syntax()));
    Arc::new(map)
}

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase:
    InternDatabase + AstDatabase + SourceDatabase + Upcast<dyn AstDatabase>
{
    #[salsa::invoke(ItemTree::file_item_tree_query)]
    fn file_item_tree(&self, file_id: FileId) -> Arc<ItemTree>;
}

#[salsa::query_group(HirDatabaseStorage)]
pub trait HirDatabase: DefDatabase + Upcast<dyn DefDatabase> {}

trait Intern {
    type ID;
    fn intern(self, db: &dyn DefDatabase) -> Self::ID;
}

pub trait Lookup {
    type Data;
    fn lookup(&self, db: &dyn DefDatabase) -> Self::Data;
}

/// `InFile<T>` stores a value of `T` inside a particular file/syntax tree.
///
/// Typical usages are:
///
/// * `InFile<SyntaxNode>` -- syntax node in a file
/// * `InFile<ast::FnDef>` -- ast node in a file
/// * `InFile<TextSize>` -- offset in a file
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InFile<T> {
    pub file_id: FileId,
    pub value: T,
}

impl<T> InFile<T> {
    pub fn new(file_id: FileId, value: T) -> InFile<T> {
        InFile { file_id, value }
    }

    pub fn with_value<U>(&self, value: U) -> InFile<U> {
        InFile::new(self.file_id, value)
    }

    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> InFile<U> {
        InFile::new(self.file_id, f(self.value))
    }
    pub fn as_ref(&self) -> InFile<&T> {
        self.with_value(&self.value)
    }
    pub fn file_syntax(&self, db: &dyn crate::AstDatabase) -> SyntaxNode {
        db.parse(self.file_id).tree().syntax().clone()
    }
}

impl<T: Clone> InFile<&T> {
    pub fn cloned(&self) -> InFile<T> {
        self.with_value(self.value.clone())
    }
}

impl<T> InFile<Option<T>> {
    pub fn transpose(self) -> Option<InFile<T>> {
        let value = self.value?;
        Some(InFile::new(self.file_id, value))
    }
}
