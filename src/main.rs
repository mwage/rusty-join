mod all_hash;
mod encoder;
mod helper;
mod quintuple_sort;
mod split_during_read;
mod split_no_encode;

use all_hash::all_hash;
use quintuple_sort::quintuple_sort;
use split_during_read::split_during_read;
use split_no_encode::split_no_encode;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // all_hash(args);          // ~136m
    // quintuple_sort(args);    // ~129m
    // split_during_read(args);    // ~133/138m
    split_no_encode(args); // ~100m
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
            "--release",
            "--",
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
