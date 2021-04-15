use super::*;
use std::{fmt, fs, str};

fn dump_parse<T: AstNode + fmt::Debug>(parse: Parse<T>) -> String {
    let mut s = String::new();
    s.push_str(&format!("{:#?}", parse.syntax_node()));
    s.push_str(&format_errors(parse.errors()));
    s
}

fn dump_parse_no_errors(parse: Parse<Program>) -> Result<String, String> {
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

#[test]
fn successes() {
    insta::glob!("snapshot_inputs/successes/*.lua", |path| {
        let input = fs::read_to_string(path).unwrap();
        let suffix = path.file_stem().unwrap().to_str().unwrap();
        insta::with_settings!({snapshot_path => "snapshots/successes", snapshot_suffix => suffix}, {
            let parse = Program::parse(&input);
            let res = dump_parse_no_errors(parse);
            match res {
                Ok(s) => insta::assert_snapshot!(s),
                Err(s) => panic!("Test {} should not have any errors:\n{}", path.display(), s),
            }
        })
    })
}

#[test]
fn luajit() {
    insta::glob!("snapshot_inputs/luajit/*.lua", |path| {
        let input = fs::read_to_string(path).unwrap();
        let suffix = path.file_stem().unwrap().to_str().unwrap();
        insta::with_settings!({snapshot_path => "snapshots/luajit", snapshot_suffix => suffix}, {
            let parse = Program::parse(&input);
            let res = dump_parse_no_errors(parse);
            match res {
                Ok(s) => insta::assert_snapshot!(s),
                Err(s) => panic!("Test {} should not have any errors:\n{}", path.display(), s),
            }
        })
    })
}

#[test]
fn fails() {
    insta::glob!("snapshot_inputs/fails/*.lua", |path| {
        let input = fs::read_to_string(path).unwrap();
        let suffix = path.file_stem().unwrap().to_str().unwrap();
        insta::with_settings!({snapshot_path => "snapshots/fails", snapshot_suffix => suffix}, {
            let parse = Program::parse(&input);
            insta::assert_snapshot!(dump_parse(parse));
        })
    })
}

#[test]
fn nothing() {
    insta::assert_debug_snapshot!(Program::parse("").syntax_node(), @"Program@0..0")
}

#[test]
fn fuzz1() {
    Program::parse("e,,");
}
