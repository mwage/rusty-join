use crate::helper::*;
use rustc_hash::FxHashMap;

// Reads all files into hash maps with the first column as key, then merges together accordingly.
// Values are not encoded, just stored as a string.
pub fn split_no_encode(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_split_no_encoding(&args[1]), read_file_split_no_encoding(&args[2]),
        read_file_split_no_encoding(&args[3]), read_file_split_no_encoding(&args[4])
    );
    join_first_three_and_output_with_forth(f1, f2, f3, f4);
}

// Iterates the hashmaps and outputs all correct matches
fn join_first_three_and_output_with_forth(f1: FxHashMap<String, Vec<String>>, f2: FxHashMap<String, Vec<String>>, f3: FxHashMap<String, Vec<String>>, f4: FxHashMap<String, Vec<String>>) {
    for key in f1.keys() {
        if !f2.contains_key(key) || !f3.contains_key(key) {
            continue;   // Not in all 3
        }

        for x1 in f1.get(key).unwrap() {
            for x2 in f2.get(key).unwrap() {
                for x3 in f3.get(key).unwrap() {
                    if !f4.contains_key(x3) {
                        continue;
                    }
                    
                    for x4 in f4.get(x3).unwrap() {
                        println!("{},{},{},{},{}", x3, key, x1, x2, x4);
                    }
                }
            }
        }
    }
}