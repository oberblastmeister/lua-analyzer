use std::{marker::PhantomData, sync::Arc};

use base_db::FileId;
use la_arena::Idx;
use syntax::{
    ast::{self, AstNode},
    match_ast, SourceFile, SyntaxNode, WalkEvent,
};

use crate::{ast_id_map::AstIdMap, expr::ParamList, name::AsName, DefDatabase};

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
        // match item {
        // ast::StmtItem::FunctionDefStmt(_) | ast::StmtItem::func
        // }

        let items = match item {
            ast::StmtItem::LocalAssignStmt(ast) => self.lower_local_assign(ast).map(Into::into),
            ast::StmtItem::LocalFunctionDefStmt(ast) => {
                self.lower_local_function(ast).map(Into::into)
            }
            ast::StmtItem::FunctionDefStmt(ast) => self.lower_function(ast).map(Into::into),
        };

        items
    }

    fn collect_inner_items(&mut self, container: &SyntaxNode) {
        let mut block_stack = Vec::new();

        for event in container.preorder().skip(1) {
            match event {
                WalkEvent::Enter(node) => {
                    match_ast! {
                        match node {
                            ast::Block(block) => {
                                block_stack.push(self.ast_id_map.ast_id(&block));
                            },
                            ast::StmtItem(item) => {
                                let mod_item = self.lower_mod_item(&item, true);
                                let current_block = block_stack.last();
                                if let (Some(mod_item), Some(block)) = (mod_item, current_block) {
                                    self.data().inner_items.entry(*block).or_default().push(mod_item);
                                }
                            },
                            _ => (),
                        }
                    }
                }
                WalkEvent::Leave(node) => {
                    if ast::Block::cast(node).is_some() {
                        block_stack.pop();
                    }
                }
            }
        }
    }

    fn lower_local_assign(
        &mut self,
        assign: &ast::LocalAssignStmt,
    ) -> Option<FileItemTreeId<LocalAssign>> {
        let multi_name = self.lower_multi_name(&assign.multi_name()?);
        let ast_id = self.ast_id_map.ast_id(assign);

        let res = LocalAssign { multi_name, ast_id };

        Some(id(self.data().local_assigns.alloc(res)))
    }

    fn lower_local_function(
        &mut self,
        func: &ast::LocalFunctionDefStmt,
    ) -> Option<FileItemTreeId<LocalFunction>> {
        let name = func.name()?.as_name();
        let params = self.lower_params(&func.param_list()?)?;
        let ast_id = self.ast_id_map.ast_id(func);

        let res = LocalFunction { name, params, ast_id };

        Some(id(self.data().local_functions.alloc(res)))
    }

    fn lower_function(&mut self, func: &ast::FunctionDefStmt) -> Option<FileItemTreeId<Function>> {
        let (path, name, is_method) = self.lower_function_content(&func.function_def_content()?)?;
        let ast_id = self.ast_id_map.ast_id(func);

        let res = Function { path, name, is_method, ast_id };

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

    fn lower_multi_name(&mut self, multi_name: &ast::MultiName) -> MultiName {
        let names: Vec<_> = multi_name.names().map(|n| n.as_name()).collect();
        MultiName::new(names)
    }

    fn lower_params(&mut self, params: &ast::ParamList) -> Option<ParamList> {
        let names = self.lower_multi_name(&params.multi_name()?);
        let vararg = params.triple_dot_token().is_some();
        let res = ParamList { names, vararg };
        Some(res)
    }
}
