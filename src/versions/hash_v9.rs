use std::io::{stdout, BufWriter, Write};

use compact_str::CompactString;
use rustc_hash::{FxBuildHasher, FxHashMap};
use crate::split_no_encode_pattern_matching;
use crate::helper::read_file_no_entry_api_small_vec;


// Requires: Hash Join, Explicit Buffer, Preallocation, Pattern Matching, no entry API, CompactString, BufWriter
// New: SmallVec
// Allows us to store small vectors on the stack
pub fn hash_v9(args: Vec<String>) {
    split_no_encode_pattern_matching(args)
}

pub fn hash_v9_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_no_entry_api_small_vec(&args[1]), read_file_no_entry_api_small_vec(&args[2]),
        read_file_no_entry_api_small_vec(&args[3]), read_file_no_entry_api_small_vec(&args[4])
    );
}
