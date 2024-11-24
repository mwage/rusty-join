mod all_hash;
mod encoder;
mod helper;
mod quintuple_sort;
mod split_during_read;

use all_hash::all_hash;
use quintuple_sort::quintuple_sort;
use split_during_read::split_during_read;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    all_hash(args);          // ~136m
    // quintuple_sort(args);    // ~129m
    // split_during_read(args);    // ~133/138m
}