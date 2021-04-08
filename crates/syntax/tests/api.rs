use syntax::ast;

#[test]
fn literal_api() {
    let parse = ast::Program::parse(r#"local hello = "a string""#);
}
