use syntax::{
    ast::{self, AstNode},
    match_ast, Parse,
};

fn stmt(program: ast::SourceFile) -> ast::Stmt {
    program.stmts().next().unwrap()
}

#[test]
#[ignore]
fn learning() {
    let prog = ast::SourceFile::parse(r#"
local hello = "a string"
local function testing(first, second, third)
    local new = first + second
    return new^324 .. "adfadsf"
end"#).ok().unwrap();
    // for (i, it) in prog.syntax_node().descendants().enumerate() {
    //     println!("{}: {:#?}", i, it);
    // }
    //
    // for (i, it) in prog.syntax_node().ancestors().enumerate() {
    //     println!("{}: {:#?}", i, it);
    // }
    //
    // for (i, it) in prog.syntax_node().children().enumerate() {
    //     println!("{}: {:#?}", i, it);
    // }
    //
    // for (i, it) in prog.syntax_node().children_with_tokens().enumerate() {
    //     println!("{}: {:#?}", i, it);
    // }
    let mut preorder = prog.syntax().preorder();
    for event in preorder {
        println!("{:#?}", event);
    }
    panic!("Just to test")
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
