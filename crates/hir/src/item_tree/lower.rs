use std::sync::Arc;

use base_db::FileId;
use syntax::{
    ast::{self, AstNode},
    SourceFile,
};

use crate::{ast_id_map::AstIdMap, DefDatabase};

use super::{FileItemTreeId, Function, ItemTree, LocalAssign, ModItem, MultiName, Name};

pub(super) struct Ctx {
    tree: ItemTree,
    file: FileId,
    ast_id_map: Arc<AstIdMap>,
}

impl Ctx {
    pub(super) fn new(db: &dyn DefDatabase, file: FileId) -> Ctx {
        Ctx { tree: ItemTree::default(), file, ast_id_map: db.ast_id_map(file) }
    }

    pub(super) fn lower_module_items(mut self, module: &SourceFile) -> ItemTree {
        self.tree.top_level =
            module.stmt_items().flat_map(|item| self.lower_mod_item(&item, false)).collect();

        self.tree
    }

    fn lower_mod_item(&mut self, item: &ast::StmtItem, inner: bool) -> Option<ModItem> {
        let items = match item {
            ast::StmtItem::LocalAssignStmt(ast) => self.lower_local_assign(ast).map(Into::into),
            ast::StmtItem::LocalFunctionDefStmt(ast) => {
                self.lower_local_function(ast).map(Into::into)
            }
            ast::StmtItem::FunctionDefStmt(ast) => self.lower_function(ast).map(Into::into),
        };

        items
    }

    fn lower_local_assign(
        &mut self,
        assign: &ast::LocalAssignStmt,
    ) -> Option<FileItemTreeId<LocalAssign>> {
        todo!()
    }

    fn lower_local_function(
        &mut self,
        assign: &ast::LocalFunctionDefStmt,
    ) -> Option<FileItemTreeId<Function>> {
        todo!()
    }

    fn lower_function(&mut self, func: &ast::FunctionDefStmt) -> Option<FileItemTreeId<Function>> {
        todo!()
    }

    fn lower_multiname(&mut self, multi_name: &ast::MultiName) -> MultiName {
        let names: Vec<_> =
            multi_name.names().map(|node| Name::new(node.syntax().to_string().into())).collect();
        MultiName::new(names)
    }
}
