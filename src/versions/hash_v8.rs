use std::io::{stdout, BufWriter, Write};

use compact_str::CompactString;
use rustc_hash::{FxBuildHasher, FxHashMap};


// Requires: Hash Join, Explicit Buffer, Preallocation, Pattern Matching, no entry API, CompactString
// New: BufWriter
// Better because optimized for the task?
pub fn hash_v8(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
    join_first_three_and_output_with_forth(f1, f2, f3, f4);
}

pub fn hash_v8_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
}

fn read_file_to_map(file: &String) -> FxHashMap<CompactString, Vec<CompactString>> {
    let mut map: FxHashMap<CompactString, Vec<CompactString>> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());
    let contents = std::fs::read_to_string(file).unwrap();

    for line in contents.lines() {
        let (key, value) = line.split_once(',').unwrap();
        if let Some(entry) = map.get_mut(key) {
            entry.push(CompactString::from(value));
        } else {
            // ~2m better than without ::with_capacity(5)
            let mut vec = Vec::with_capacity(5);
            vec.push(CompactString::from(value));
            map.insert(CompactString::from(key), vec);
        }
    }
    map
}

fn join_first_three_and_output_with_forth(f1: FxHashMap<CompactString, Vec<CompactString>>, f2: FxHashMap<CompactString, Vec<CompactString>>, f3: FxHashMap<CompactString, Vec<CompactString>>, f4: FxHashMap<CompactString, Vec<CompactString>>) {
    let stdout = stdout();
    let lock = stdout.lock();
    let mut buffer = BufWriter::new(lock);
    for (key, vec1) in f1.iter() {
        if let (Some(vec2), Some(vec3)) = (f2.get(key), f3.get(key)) {
            for x1 in vec1 {
                for x2 in vec2 {
                    for x3 in vec3 {
                        if let Some(vec4) = f4.get(x3) {
                            for x4 in vec4 {
                                buffer.write(x3.as_bytes());
                                buffer.write(b",");
                                buffer.write(key.as_bytes());
                                buffer.write(b",");
                                buffer.write(x1.as_bytes());
                                buffer.write(b",");
                                buffer.write(x2.as_bytes());
                                buffer.write(b",");
                                buffer.write(x4.as_bytes());
                                buffer.write(b"\n");
                            }
                        }
                    }
                }
            }
        }
    }

    buffer.flush().unwrap();
}