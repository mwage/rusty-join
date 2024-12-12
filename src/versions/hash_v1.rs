use std::fs::read_to_string;

use rustc_hash::FxHashMap;

// Requires: -
// New: Hash join implementation
// Better because no encoding necessary
pub fn hash_v1(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
    join(f1, f2, f3, f4);
}

pub fn hash_v1_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
}

fn read_file_to_map(file: &String) -> FxHashMap<String, Vec<String>> {
    let mut map: FxHashMap<String, Vec<String>> = FxHashMap::default();

    for line in read_to_string(file).unwrap().lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        map.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }

    map
}

fn join(f1: FxHashMap<String, Vec<String>>, f2: FxHashMap<String, Vec<String>>, f3: FxHashMap<String, Vec<String>>, f4: FxHashMap<String, Vec<String>>) {
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


