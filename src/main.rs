mod all_hash;
mod encoder;
mod helper;
mod quintuple_sort;

use all_hash::all_hash;
use quintuple_sort::quintuple_sort;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // all_hash(args);
    quintuple_sort(args);
}