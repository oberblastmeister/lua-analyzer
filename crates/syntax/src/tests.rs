use super::*;
use core::fmt;
use std::fs;

fn dump_parse<T: AstNode + fmt::Debug>(parse: Parse<T>) -> String {
    let mut s = String::new();
    s.push_str(&format!("{:#?}", parse.syntax_node()));
    s.push_str(&format_errors(parse.errors()));
    s
}

fn dump_parse_no_errors(parse: Parse<Program>) -> String {
    if !parse.errors().is_empty() {
        // panic!(
        //     "Should not have any errors {:#?}\n{}",
        //     parse.syntax_node(),
        //     format_errors(parse.errors())
        // )
        panic!("Should not have any errors {:#?}", parse.syntax_node(),)
    }
    parse.debug_dump()
}

macro_rules! get_insta_path {
    ($dir:literal, $name:ident, $closure:expr) => {{
        let single_path_glob = concat!("snapshot_inputs/", $dir, "/", stringify!($name), ".lua");
        insta::glob!(single_path_glob, $closure)
    }};
}

macro_rules! do_test {
    ("successes", $name:ident) => {
        #[test]
        fn $name() {
            get_insta_path!("successes", $name, |path| {
                use std::fs;

                let input = fs::read_to_string(path)
                    .unwrap_or_else(|_| panic!("Failed to read path {} to a string", path.display()));
                let suffix = path.file_stem().unwrap().to_str().unwrap();

                insta::with_settings!(
                    {
                        snapshot_suffix => suffix,
                        snapshot_path => "snapshots/successes",
                    },
                    {
                        insta::assert_snapshot!(dump_parse_no_errors(Program::parse(&input)))
                    }
                    )
            })
        }
    };
    ("fails", $name:ident) => {
        #[test]
        fn $name() {
            get_insta_path!("fails", $name, |path| {
                use std::fs;

                let input = fs::read_to_string(path)
                    .unwrap_or_else(|_| panic!("Failed to read path {} to a string", path.display()));
                let suffix = path.file_stem().unwrap().to_str().unwrap();

                insta::with_settings!(
                    {
                        snapshot_suffix => suffix,
                        snapshot_path => "snapshots/fails",
                    },
                    {
                        insta::assert_snapshot!(dump_parse(Program::parse(&input)))
                    }
                    )
            })
        }
    }
}

macro_rules! test_successes {
    ($($name:ident),+ $(,)?) => {
        $(
            do_test!("successes", $name);
         )+
    };
}

macro_rules! test_fails {
    ($($name:ident),+ $(,)?) => {
        $(
            do_test!("fails", $name);
         )+
    };
}

test_successes![
    assign,
    comments,
    function_call,
    function_call_multi,
    function_def,
    function_literal,
    higher_order_function,
    simple_math,
    multi_name,
    variable_def,
    do_block,
    index_expr,
    dot_expr,
    array_table_expr,
    map_table_expr,
    both_table_expr,
    types,
    string_call_expr,
    table_call_expr,
    break_stmt,
    while_stmt,
    for_stmt,
    // large_test,
    ffi_test,
    prefix_expr,
    operators,
];

test_fails![can_call, cannot_call_literal, missing_paren];

#[test]
fn successes() {
    insta::glob!("snapshot_inputs/successes/*.lua", |path| {
        let input = fs::read_to_string(path).unwrap();
        insta::assert_json_snapshot!(input.to_uppercase());
    })
}

fn fails() {
    insta::glob!("snapshot_inputs/successes/*.lua", |path| {
        let input = fs::read_to_string(path).unwrap();
        insta::assert_json_snapshot!(input.to_uppercase());
    })
}

#[test]
fn nothing() {
    insta::assert_debug_snapshot!(Program::parse("").syntax_node(), @"Program@0..0
                                  ");
}
