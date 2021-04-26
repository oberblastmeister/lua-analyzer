use super::lexer::{tokenize, tokenize_iter};
use super::*;
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    str,
};

#[test]
fn parser() {
    dir_tests(snapshots_dir(), &["parser/ok"], |path, text| {
        let parse = SourceFile::parse(text);
        let errors = parse.errors();
        assert_errors_are_absent(errors, path);
        dump_parse(parse)
    });

    dir_tests(snapshots_dir(), &["parser/err"], |path, text| {
        let parse = SourceFile::parse(text);
        let errors = parse.errors();
        assert_errors_are_present(errors, path);
        dump_parse(parse)
    })
}

#[test]
fn lexer() {
    dir_tests(snapshots_dir(), &["lexer/ok"], |path, text| {
        let (tokens, errors) = tokenize(text);
        assert_errors_are_absent(&errors, path);
        assert_unknowns_are_absent(&tokens, path);
        format!("{:#?}", tokens)
    });

    dir_tests(snapshots_dir(), &["lexer/err"], |path, text| {
        let (tokens, errors) = tokenize(text);
        assert_errors_or_unknowns_are_present(&errors, &tokens, path);
        format!("{:#?}\n\n{:#?}", tokens, errors)
    })
}

fn dir_tests<P, F>(test_data_dir: P, paths: &[&str], f: F)
where
    P: AsRef<Path>,
    F: Fn(&Path, &str) -> String,
{
    for (path, input_code) in collect_lua_files(test_data_dir.as_ref(), paths) {
        assert_path(&path, &f(&path, &input_code))
    }
}

fn assert_path<P>(path: P, actual: &str)
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(
        path.parent()
            .expect("There is no parent for the given path"),
    );
    settings.set_prepend_module_to_snapshot(false);
    settings.set_input_file(path);
    settings.bind(|| {
        let name = path
            .file_stem()
            .expect("No file stem")
            .to_str()
            .expect("Could not turn path to str");
        insta::assert_snapshot!(name, actual);
    })
}

fn collect_lua_files(root_dir: &Path, paths: &[&str]) -> Vec<(PathBuf, String)> {
    paths
        .iter()
        .flat_map(|path| {
            let path = root_dir.join(path);
            lua_files_in_dir(&path).into_iter()
        })
        .map(|path| {
            let text = read_text(&path);
            (path, text)
        })
        .collect()
}

fn lua_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut acc = Vec::new();
    for file in fs::read_dir(&dir).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().unwrap_or_default() == "lua" {
            acc.push(path);
        }
    }
    acc.sort();
    acc
}
fn assert_errors_or_unknowns_are_present(errors: &[SyntaxError], tokens: &[Token], path: &Path) {
    assert!(
        !errors.is_empty() || !tokens.is_empty(),
        "There should be errors in the file {:?}",
        path.display()
    );
}

fn assert_errors_are_present(errors: &[SyntaxError], path: &Path) {
    assert!(
        !errors.is_empty(),
        "There should be errors in the file {:?}",
        path.display()
    );
}

fn assert_errors_are_absent(errors: &[SyntaxError], path: &Path) {
    assert_eq!(
        errors,
        &[] as &[SyntaxError],
        "There should be no errors in the file {:?}",
        path.display(),
    );
}

fn collect_unknowns(tokens: &[Token]) -> Vec<Token> {
    tokens
        .into_iter()
        .filter(|t| t.kind == T![unknown])
        .map(|t| *t)
        .collect()
}

fn assert_unknowns_are_absent(tokens: &[Token], path: &Path) {
    let unknowns = collect_unknowns(tokens);
    assert_eq!(
        unknowns,
        &[] as &[Token],
        "There should be no unknowns in the file {:?}",
        path.display()
    )
}

fn assert_unknowns_are_present(tokens: &[Token], path: &Path) {
    let unknowns = collect_unknowns(tokens);
    assert!(
        !unknowns.is_empty(),
        "There should be unknowns in the file {:?}",
        path.display()
    )
}

fn dump_parse<T: AstNode + fmt::Debug>(parse: Parse<T>) -> String {
    let mut s = String::new();
    s.push_str(&format!("{:#?}", parse.syntax_node()));
    s.push_str(&format_errors(parse.errors()));
    s
}

fn dump_parse_no_errors(parse: Parse<SourceFile>) -> Result<String, String> {
    if !parse.errors().is_empty() {
        return Err(dump_parse(parse));
        // panic!(
        //     "Should not have any errors {:#?}\n{}",
        //     parse.syntax_node(),
        //     format_errors(parse.errors())
        // )
    }
    Ok(parse.debug_dump())
}

/// Read file and normalize newlines.
///
/// `rustc` seems to always normalize `\r\n` newlines to `\n`:
///
/// ```
/// let s = "
/// ";
/// assert_eq!(s.as_bytes(), &[10]);
/// ```
///
/// so this should always be correct.
fn read_text(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("File at {:?} should be valid", path))
        .replace("\r\n", "\n")
}

fn snapshots_dir() -> PathBuf {
    project_root().join("crates/syntax/snapshots")
}

fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

#[test]
fn nothing() {
    insta::assert_debug_snapshot!(SourceFile::parse("").syntax_node(), @"SourceFile@0..0")
}

#[test]
fn fuzz1() {
    SourceFile::parse("e,,");
}

#[test]
fn local_only() {
    SourceFile::parse("local");
}
