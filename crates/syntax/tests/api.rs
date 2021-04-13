use syntax::{
    ast::{self, AstNode},
    match_ast, Parse,
};

fn stmt(program: ast::Program) -> ast::Stmt {
    program.stmts().next().unwrap()
}

// #[test]
// fn literal_api() {
//     let prog = ast::Program::parse(r#"local hello = "a string""#).tree();
//     let stmt = stmt(prog);
//     let stmt = stmt.syntax();
//     let assign_stmt = match_ast! {
//         match stmt {
//             ast::AssignStmt(it) => it,
//             _ => panic!(),
//         }
//     };
//     assert_eq!(assign_stmt.local_token().unwrap().text(), "local");
//     assert_eq!(
//         assign_stmt
//             .multi_name()
//             .unwrap()
//             .names()
//             .next()
//             .unwrap()
//             .ident_token()
//             .unwrap()
//             .text(),
//         "hello"
//         );
//     assert_eq!(assign_stmt.eq_token().unwrap().text(), "=");
// }
