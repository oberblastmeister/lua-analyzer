use std::sync::Arc;

use base_db::FileId;
use la_arena::Arena;
use syntax::ast::{self, AstNode};

use super::Body;
use crate::{
    ast_id_map::AstIdMap,
    expr::{Expr, ExprId, ParamList},
    item_tree::AstId,
    stmt::{self, Stmt, StmtId},
    DefDatabase,
};

pub(super) fn lower(
    db: &dyn DefDatabase,
    file_id: FileId,
    params: Option<ast::ParamList>,
    body: Option<ast::Stmt>,
) -> Body {
    Collector {
        db,
        file_id,
        ast_id_map: db.ast_id_map(file_id),
        body: Body {
            exprs: Arena::default(),
            stmts: Arena::default(),
            labels: Arena::default(),
            names: Arena::default(),
            params: ParamList::default(),
            body_stmt: stmt::dummy_id(),
            block_scopes: Vec::default(),
        },
    }
    .collect(params, body)
}

struct Collector<'db> {
    db: &'db dyn DefDatabase,
    file_id: FileId,
    ast_id_map: Arc<AstIdMap>,
    body: Body,
}

impl<'db> Collector<'db> {
    fn collect(mut self, params_list: Option<ast::ParamList>, body: Option<ast::Stmt>) -> Body {
        if let Some(param_list) = params_list {}
        todo!()
    }

    fn collect_stmt_opt(&mut self, stmt: Option<ast::Stmt>) -> StmtId {
        if let Some(stmt) = stmt {
            self.collect_stmt(stmt)
        } else {
            self.missing_stmt()
        }
    }

    fn collect_stmt(&mut self, stmt: ast::Stmt) -> StmtId {
        todo!()
    }

    fn collect_block(&mut self, block: ast::Block) -> StmtId {
        let ast_id = self.ast_id(&block);
        todo!()
    }

    fn missing_stmt(&mut self) -> StmtId {
        self.alloc_stmt(Stmt::Missing)
    }

    fn alloc_stmt(&mut self, stmt: Stmt) -> StmtId {
        self.body.stmts.alloc(stmt)
    }

    fn alloc_expr(&mut self, expr: Expr) -> ExprId {
        self.body.exprs.alloc(expr)
    }

    fn ast_id<N: AstNode>(&self, item: &N) -> AstId<N> {
        let file_local_id = self.ast_id_map.ast_id(item);
        AstId::new(self.file_id, file_local_id)
    }
}
