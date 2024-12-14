#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::env;

// use polars_join::custom_polars;
//use rusty_join::split_no_encode;
//use rusty_join::all_hash;
//use rusty_join::quintuple_sort;
//use rusty_join::split_during_read;
use rusty_join::*;
use versions::*;

fn main() {
    // TODO: Bench Allocator at the very end?
    let args: Vec<String> = env::args().collect();
    // history_v3(args);
    // sorting_v1(args);
    reduced_hash_v4(args);
    // hash_v9(args);
    // parallel_hash(args);
    // parallel_reduced_hash(args);
    // all_hash(args);
    // custom_polars(args);
    // quintuple_sort(args);
    // split_during_read(args);
    //split_no_encode(args);
    // split_no_encode_pattern_matching(args);
    // split_no_encode_parallel(args);
}

/// #DON"T forget to use your implementation above!!!"
/// Tests the functionality of the current solution defined above (comment out the ones youdont want)
///
/// Panics if there is no /data/ directory with a-d.csv and abcd.csv
///
///
#[test]
fn sanity_test() {
    use std::{fs, process::Command};

    let our_output = Command::new("cargo")
        .args([
            "run",
            "data/a.csv",
            "data/b.csv",
            "data/c.csv",
            "data/d.csv",
        ])
        .output()
        .unwrap();

    let our_str = String::from_utf8(our_output.stdout).unwrap();
    let expected = fs::read_to_string("data/abcd.csv").unwrap();

    let mut our_lines: Vec<&str> = our_str.lines().collect();
    let mut expected_lines: Vec<&str> = expected.lines().collect();

    our_lines.sort();
    expected_lines.sort();
    assert_eq!(our_lines, expected_lines);
}
