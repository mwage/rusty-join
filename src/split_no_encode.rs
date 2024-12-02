use crate::helper::*;
use rustc_hash::FxHashMap;
use std::fs::read_to_string;
use compact_str::CompactString;

// Reads all files into hash maps with the first column as key, then merges together accordingly.
// Values are not encoded, just stored as a string.
pub fn split_no_encode(args: Vec<String>) {
    let (f1, f2, f3, f4) = (
        read_file_split_no_encoding_compact(&args[1]), read_file_split_no_encoding_compact(&args[2]),
        read_file_split_no_encoding_compact(&args[3]), read_file_split_no_encoding_compact(&args[4])
    );
    join_first_three_and_output_with_forth(f1, f2, f3, f4);
}

// Iterates the hashmaps and outputs all correct matches
fn join_first_three_and_output_with_forth(f1: FxHashMap<CompactString, Vec<CompactString>>, f2: FxHashMap<CompactString, Vec<CompactString>>, f3: FxHashMap<CompactString, Vec<CompactString>>, f4: FxHashMap<CompactString, Vec<CompactString>>) {
    for key in f1.keys() {
        if !f2.contains_key(key) || !f3.contains_key(key) {
            continue;   // Not in all 3
        }

        for x1 in f1.get(key).unwrap().iter() {
            for x2 in f2.get(key).unwrap().iter() {
                for x3 in f3.get(key).unwrap().iter() {
                    if !f4.contains_key(x3) {
                        continue;
                    }
                    
                    for x4 in f4.get(x3).unwrap().iter() {
                        println!("{},{},{},{},{}", x3, key, x1, x2, x4);
                    }
                }
            }
        }
    }
}

pub fn split_with_str_read(args: Vec<String>) {
    let mut f1: FxHashMap<String, Vec<String>> = FxHashMap::default();
    let f1_str = read_to_string(&args[1]).unwrap();
    for line in f1_str.lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        f1.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }
    let mut f2: FxHashMap<String, Vec<String>> = FxHashMap::default();
    let f2_str = read_to_string(&args[1]).unwrap();
    for line in f2_str.lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        f2.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }
    let mut f3: FxHashMap<String, Vec<String>> = FxHashMap::default();
    let f3_str = read_to_string(&args[1]).unwrap();
    for line in f3_str.lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        f3.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }
    let mut f4: FxHashMap<String, Vec<String>> = FxHashMap::default();
    let f4_str = read_to_string(&args[1]).unwrap();
    for line in f4_str.lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        f4.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }
    for key in f1.keys() {
        if !f2.contains_key(key) || !f3.contains_key(key) {
            continue;   // Not in all 3
        }

        for x1 in f1.get(key).unwrap().iter() {
            for x2 in f2.get(key).unwrap().iter() {
                for x3 in f3.get(key).unwrap().iter() {
                    if !f4.contains_key(x3) {
                        continue;
                    }
                    
                    for x4 in f4.get(x3).unwrap().iter() {
                        println!("{},{},{},{},{}", x3, key, x1, x2, x4);
                    }
                }
            }
        }
    }
}