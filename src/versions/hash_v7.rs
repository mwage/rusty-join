use compact_str::CompactString;
use rustc_hash::{FxBuildHasher, FxHashMap};
use crate::helper::read_file_no_entry_api_prealloc_vec;

// Requires: Hash Join, Explicit Buffer, Preallocation, Pattern Matching, no entry API
// New: CompactString
// Better because the small CompactStrings can be stored on the stack
pub fn hash_v7(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_no_entry_api_prealloc_vec(&args[1]), read_file_no_entry_api_prealloc_vec(&args[2]),
        read_file_no_entry_api_prealloc_vec(&args[3]), read_file_no_entry_api_prealloc_vec(&args[4])
    );
    join_first_three_and_output_with_forth(f1, f2, f3, f4);
}

pub fn hash_v7_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_no_entry_api_prealloc_vec(&args[1]), read_file_no_entry_api_prealloc_vec(&args[2]),
        read_file_no_entry_api_prealloc_vec(&args[3]), read_file_no_entry_api_prealloc_vec(&args[4])
    );
}

fn join_first_three_and_output_with_forth(f1: FxHashMap<CompactString, Vec<CompactString>>, f2: FxHashMap<CompactString, Vec<CompactString>>, f3: FxHashMap<CompactString, Vec<CompactString>>, f4: FxHashMap<CompactString, Vec<CompactString>>) {
    let mut buffer = String::new();
    for (key, vec1) in f1.iter() {
        if let (Some(vec2), Some(vec3)) = (f2.get(key), f3.get(key)) {
            for x1 in vec1 {
                for x2 in vec2 {
                    for x3 in vec3 {
                        if let Some(vec4) = f4.get(x3) {
                            for x4 in vec4 {
                                buffer.push_str(x3);
                                buffer.push(',');
                                buffer.push_str(key);
                                buffer.push(',');
                                buffer.push_str(x1);
                                buffer.push(',');
                                buffer.push_str(x2);
                                buffer.push(',');
                                buffer.push_str(x4);
                                buffer.push('\n');
                            }
                        }
                    }
                }
            }
        }
    }

    print!("{}", buffer);
}