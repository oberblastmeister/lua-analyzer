use std::{marker::PhantomData, sync::Arc};

use base_db::FileId;
use la_arena::Idx;
use syntax::{
    ast::{self, AstNode},
    SourceFile,
};

use crate::{ast_id_map::AstIdMap, name::AsName, DefDatabase};

use super::{
    FileItemTreeId, Function, IndexPath, ItemTree, ItemTreeData, ItemTreeNode, LocalAssign,
    LocalFunction, ModItem, MultiName, Name,
};

fn id<N: ItemTreeNode>(index: Idx<N>) -> FileItemTreeId<N> {
    FileItemTreeId { index, _p: PhantomData }
}

pub(super) struct Ctx {
    tree: ItemTree,
    file: FileId,
    ast_id_map: Arc<AstIdMap>,
}

impl Ctx {
    pub(super) fn new(db: &dyn DefDatabase, file: FileId) -> Ctx {
        Ctx { tree: ItemTree::default(), file, ast_id_map: db.ast_id_map(file) }
    }

    fn data(&mut self) -> &mut ItemTreeData {
        self.tree.data_mut()
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
        func: &ast::LocalFunctionDefStmt,
    ) -> Option<FileItemTreeId<LocalFunction>> {
        let name = func.name()?.as_name();
        let params = self.lower_multiname(&func.paramlist()?.multi_name()?);
        let ast_id = self.ast_id_map.ast_id(func);

        let res = LocalFunction { name, params, ast_id };

        Some(id(self.data().local_functions.alloc(res)))
    }

    fn lower_function(&mut self, func: &ast::FunctionDefStmt) -> Option<FileItemTreeId<Function>> {
        let (path, name, is_method) = self.lower_function_content(&func.function_def_content()?)?;
        let ast_id = self.ast_id_map.ast_id(func);
        let params = self.lower_multiname(&func.paramlist()?.multi_name()?);

        let res = Function { path, name, is_method, params, ast_id };

        Some(id(self.data().functions.alloc(res)))
    }

    fn lower_function_content(
        &mut self,
        content: &ast::FunctionDefContent,
    ) -> Option<(IndexPath, Name, bool)> {
        let res = match content {
            ast::FunctionDefContent::FunctionMethod(ast) => {
                let path = self.lower_index_path(&ast.index_path()?);
                let name = ast.name()?.as_name();
                (path, name, true)
            }
            ast::FunctionDefContent::FunctionStatic(ast) => {
                let path = self.lower_index_path(&ast.index_path()?);
                let name = ast.name()?.as_name();
                (path, name, false)
            }
            ast::FunctionDefContent::Name(name) => {
                let path = IndexPath::default();
                let name = name.as_name();
                (path, name, false)
            }
        };
        Some(res)
    }

    fn lower_index_path(&mut self, index_path: &ast::IndexPath) -> IndexPath {
        let text = index_path.name_refs().map(|n| n.as_name()).collect();
        IndexPath::new(text)
    }

    fn lower_multiname(&mut self, multi_name: &ast::MultiName) -> MultiName {
        let names: Vec<_> = multi_name.names().map(|n| n.as_name()).collect();
        MultiName::new(names)
    }
}
