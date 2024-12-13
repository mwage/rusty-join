use rustc_hash::{FxBuildHasher, FxHashMap};


// Requires: Hash Join, Explicit Buffer, Preallocation, Pattern Matching
// New: Avoiding Entry API, use split_once to get tuple // TODO: Maybe split into two?
// Better because of API overheads?
pub fn hash_v6(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
    join_first_three_and_output_with_forth(f1, f2, f3, f4);
}

pub fn hash_v6_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_to_map(&args[1]), read_file_to_map(&args[2]),
        read_file_to_map(&args[3]), read_file_to_map(&args[4])
    );
}

fn read_file_to_map(file: &String) -> FxHashMap<String, Vec<String>> {
    let mut map: FxHashMap<String, Vec<String>> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());
    let contents = std::fs::read_to_string(file).unwrap();

    for line in contents.lines() {
        if let Some((key, value)) = line.split_once(',') {
            if let Some(entry) = map.get_mut(key) {
                entry.push(value.to_string());
            } else {
                let mut vec = Vec::with_capacity(5);
                vec.push(value.to_string());
                map.insert(key.to_string(), vec);
            }
        }
    }
    map
}

fn join_first_three_and_output_with_forth(f1: FxHashMap<String, Vec<String>>, f2: FxHashMap<String, Vec<String>>, f3: FxHashMap<String, Vec<String>>, f4: FxHashMap<String, Vec<String>>) {
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