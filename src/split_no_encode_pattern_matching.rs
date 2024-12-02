use crate::helper::*;
use rustc_hash::FxHashMap;

// Reads all files into hash maps with the first column as key, then merges together accordingly.
// Values are not encoded, just stored as a string.
pub fn split_no_encode_pattern_matching(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_no_entry_api(&args[1]), read_file_no_entry_api(&args[2]),
        read_file_no_entry_api(&args[3]), read_file_no_entry_api(&args[4])
    );
    
    join_first_three_and_output_with_forth(f1, f2, f3, f4);
}

// Iterates the hashmaps and outputs all correct matches
fn join_first_three_and_output_with_forth(f1: FxHashMap<String, Vec<String>>, f2: FxHashMap<String, Vec<String>>, f3: FxHashMap<String, Vec<String>>, f4: FxHashMap<String, Vec<String>>) {
    for (key, vec1) in f1.iter() {
        if let (Some(vec2), Some(vec3)) = (f2.get(key), f3.get(key)) {
            for x1 in vec1 {
                for x2 in vec2 {
                    for x3 in vec3 {
                        if let Some(vec4) = f4.get(x3) {
                            for x4 in vec4 {
                                println!("{},{},{},{},{}", x3, key, x1, x2, x4);
                            }
                        }
                    }
                }
            }
        }
    }
}