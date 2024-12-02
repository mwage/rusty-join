use crate::helper::*;
use rustc_hash::FxHashMap;
use std::thread;
use std::fs::read_to_string;
use compact_str::CompactString;

// Reads all files into hash maps with the first column as key, then merges together accordingly.
// Values are not encoded, just stored as a string.
// Succeeded by split_no_encode_pattern_matching
pub fn split_no_encode(args: Vec<String>) {
    let t1 = thread::spawn({
        let file_path = args[1].clone();
        move || read_file_split_no_encoding(&file_path)
    });

    let t2 = thread::spawn({
        let file_path = args[2].clone();
        move || read_file_split_no_encoding(&file_path)
    });

    let t3 = thread::spawn({
        let file_path = args[3].clone();
        move || read_file_split_no_encoding(&file_path)
    });

    let t4 = thread::spawn({
        let file_path = args[4].clone();
        move || read_file_split_no_encoding(&file_path)
    });

    let f1 = t1.join().expect("Thread 1 panicked");
    let f2 = t2.join().expect("Thread 2 panicked");
    let f3 = t3.join().expect("Thread 3 panicked");
    let f4 = t4.join().expect("Thread 4 panicked");
    
    join_first_three_and_output_with_forth(f1, f2, f3, f4);
}

// Iterates the hashmaps and outputs all correct matches
fn join_first_three_and_output_with_forth(f1: FxHashMap<CompactString, Vec<CompactString>>, f2: FxHashMap<CompactString, Vec<CompactString>>, f3: FxHashMap<CompactString, Vec<CompactString>>, f4: FxHashMap<CompactString, Vec<CompactString>>) {
    let mut buffer = String::new();
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