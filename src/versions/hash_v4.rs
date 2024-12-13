use rustc_hash::{FxHashMap, FxBuildHasher};
use std::fs::read_to_string;

// Requires: Hash Join, Explicit Buffer
// New: Preallocate Vector and Hashmap
// Better because it removes format overhead
pub fn hash_v4(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
    join_first_three_and_output_with_forth(f1, f2, f3, f4);
}

pub fn hash_v4_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
}

fn read_file_to_map(file: &String) -> FxHashMap<String, Vec<String>> {
    let mut map: FxHashMap<String, Vec<String>> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());

    for line in read_to_string(file).unwrap().lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        map.entry(split.next().unwrap())
            .or_insert_with(|| Vec::with_capacity(5))
            .push(split.next().unwrap());
    }

    map
}

fn join_first_three_and_output_with_forth(f1: FxHashMap<String, Vec<String>>, f2: FxHashMap<String, Vec<String>>, f3: FxHashMap<String, Vec<String>>, f4: FxHashMap<String, Vec<String>>) {
    let mut buffer = String::new();
    
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

    print!("{}", buffer);
}