use std::{fs::read_to_string, io::{stdout, BufWriter, Write}};

use compact_str::CompactString;
use rustc_hash::{FxBuildHasher, FxHashMap};
use smallvec::SmallVec;

// Requires: Hash Join, Explicit Buffer, Preallocation, Pattern Matching, no entry API, CompactString, BufWriter, SmallVec
// New: reduced number of Hashmaps
// Faster (this version same?) because / less Hashmaps
pub fn reduced_hash_v1(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file(&args[1]), read_file(&args[2]), read_file(&args[3]), read_file_to_map(&args[4])
    );
    
    join(f1, f2, f3, f4);
}

pub fn reduced_hash_v1_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file(&args[1]), read_file(&args[2]), read_file(&args[3]), read_file_to_map(&args[4])
    );
}

pub fn read_file(file: &String) -> Vec<(CompactString, CompactString)> {
    read_to_string(file).unwrap().lines().map(
        |line| { 
            let (key, val) = line.split_once(",").unwrap(); 
            (CompactString::from(key), CompactString::from(val)) 
        }
    ).collect()
}

fn read_file_to_map(file: &String) -> FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
    let mut map: FxHashMap<CompactString, SmallVec<[CompactString; 1]>> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());
    let contents = std::fs::read_to_string(file).unwrap();

    for line in contents.lines() {
        let (key, value) = line.split_once(',').unwrap();
        if let Some(entry) = map.get_mut(key) {
            entry.push(CompactString::from(value));
        } else {
            let mut vec = SmallVec::new();
            vec.push(CompactString::from(value));
            map.insert(CompactString::from(key), vec);
        }
    }
    map
}

pub fn join(f1: Vec<(CompactString, CompactString)>, f2: Vec<(CompactString, CompactString)>, f3: Vec<(CompactString, CompactString)>, f4: FxHashMap<CompactString, SmallVec<[CompactString; 1]>>) {
    let mut dict_a: FxHashMap<CompactString, (SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>)> = FxHashMap::default();
    for (key, value) in f1.iter() {
        if let Some(entry) = dict_a.get_mut(key) {
            entry.0.push(value.clone());
        } else {
            let mut vec = SmallVec::new();
            vec.push(value.clone());
            dict_a.insert(key.clone(), (vec, SmallVec::new(), SmallVec::new()));
        }
    }
    for data in f2.iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            entry.1.push(data.1.clone());
        }
    }
    for data in f3.iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            if !entry.1.is_empty() {
                entry.2.push(data.1.clone());
            }
        }
    }

    let stdout = stdout();
    let lock = stdout.lock();
    let mut buffer = BufWriter::new(lock);
    for (a_val, (f1_2, f2_2, f3_2)) in dict_a.iter() {
        for f3_2_val in f3_2.iter() {
            if let Some(f4_2_list) = f4.get(f3_2_val) {
                for f4_2_val in f4_2_list.iter() {
                    for f2_2_val in f2_2.iter() {
                        for f1_2_val in f1_2.iter() {
                            buffer.write(f3_2_val.as_bytes());
                            buffer.write(b",");
                            buffer.write(a_val.as_bytes());
                            buffer.write(b",");
                            buffer.write(f1_2_val.as_bytes());
                            buffer.write(b",");
                            buffer.write(f2_2_val.as_bytes());
                            buffer.write(b",");
                            buffer.write(f4_2_val.as_bytes());
                            buffer.write(b"\n");
                        }
                    }
                }
            }
        }
    }

    buffer.flush().unwrap()
}