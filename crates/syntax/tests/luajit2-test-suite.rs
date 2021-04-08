use glob::glob;
use rayon::prelude::*;
use std::{fs, path::PathBuf, sync::atomic::AtomicBool};
use std::{
    panic,
    sync::{atomic::Ordering, Arc, Mutex},
};
use syntax::{ast::Program, SyntaxError};

struct Failure {
    path: PathBuf,
    errors: Vec<SyntaxError>,
    panicked: Option<String>,
}

const GLOB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/luajit2-test-suite/**/*.lua");

#[test]
fn run_test_suite() {
    let errored = Arc::new(AtomicBool::new(false));

    glob(GLOB).unwrap().par_bridge().for_each({
        let errored = errored.clone();
        move |res| {
            let path = res.expect("Glob entry failed");
            let contents = fs::read_to_string(&path).expect("Failed to read path to string");
            let result = panic::catch_unwind(|| Program::parse(&contents));
            match result {
                Ok(parse) => {
                    if !parse.errors().is_empty() {
                        errored.store(true, Ordering::Relaxed);
                        eprintln!("Test case failed: {}", path.display());
                    } else {
                        eprintln!("Test case {} succeeded", path.display());
                    }
                }
                Err(e) => {
                    errored.store(true, Ordering::Relaxed);
                    eprintln!("Test case {} panicked", path.display());
                }
            }
        }
    });

    if errored.load(Ordering::Relaxed) {
        panic!("Some test cases didn't pass");
    }
}
