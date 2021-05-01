mod ast_id_map;
pub mod body;
pub mod expr;
mod item_tree;
mod name;
mod semantics;
pub mod stmt;

use std::sync::Arc;

use ast_id_map::AstIdMap;
use item_tree::ItemTree;
pub use semantics::Semantics;

use base_db::{salsa, FileId, SourceDatabase, Upcast};
use syntax::ast::AstNode;

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: SourceDatabase {
    // fn intern_function(&self, loc: ItemTreeId<Function>) -> FunctionId
}

#[salsa::query_group(AstDatabaseStorage)]
pub trait AstDatabase: SourceDatabase {
    fn ast_id_map(&self, file_id: FileId) -> Arc<AstIdMap>;
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
