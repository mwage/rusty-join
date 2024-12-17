use rustc_hash::{FxHashMap, FxBuildHasher};
use std::fs::read_to_string;

// Requires: Hash Join, Explicit Buffer, Preallocation
// New: Pattern Matching
// Better because it reduces branch mispredictions?
pub fn hash_v5(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
    join(f1, f2, f3, f4);
}

pub fn hash_v5_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
}

fn read_file_to_map(file: &String) -> FxHashMap<String, Vec<String>> {
    let mut map: FxHashMap<String, Vec<String>> = 
        FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());

    for line in read_to_string(file).unwrap().lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        map.entry(split.next().unwrap())
            .or_insert_with(|| Vec::with_capacity(5))
            .push(split.next().unwrap());
    }

    map
}

fn join(f1: FxHashMap<String, Vec<String>>, f2: FxHashMap<String, Vec<String>>, 
    f3: FxHashMap<String, Vec<String>>, f4: FxHashMap<String, Vec<String>>) 
{
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