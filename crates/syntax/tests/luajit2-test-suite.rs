use glob::glob;
use rayon::prelude::*;
use std::{
    fs,
    sync::atomic::{AtomicBool, AtomicU32},
};
use std::{
    panic,
    sync::{atomic::Ordering, Arc},
};
use syntax::ast::SourceFile;

const GLOB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/luajit2-test-suite/**/*.lua");

#[test]
fn run_test_suite() {
    let total = Arc::new(AtomicU32::new(0));
    let successes = Arc::new(AtomicU32::new(0));

    glob(GLOB).unwrap().par_bridge().for_each({
        let successes = successes.clone();
        let total = total.clone();
        move |res| {
            total.fetch_add(1, Ordering::Relaxed);
            let path = res.expect("Glob entry failed");
            let contents = fs::read_to_string(&path).expect("Failed to read path to string");
            let result = panic::catch_unwind(|| SourceFile::parse(&contents));
            match result {
                Ok(parse) => {
                    if !parse.errors().is_empty() {
                        eprintln!("Test case failed: {}", path.display());
                    } else {
                        successes.fetch_add(1, Ordering::Relaxed);
                        eprintln!("Test case {} succeeded", path.display());
                    }
                }
                Err(e) => {
                    eprintln!("Test case {} panicked", path.display());
                }
            }
        }
    });

    let total = total.load(Ordering::Relaxed);
    let successes = successes.load(Ordering::Relaxed);
    eprintln!("{}/{} success rate", successes, total);
    if total != successes {
        panic!("Some test cases failed");
    }
}
