use crate::helper::*;
use crate::encoder::Encoder;
use generic_array::GenericArray;
use typenum::{U3, U4};
use rustc_hash::FxHashMap;

// Reads all files into hash maps with the first column as key, then merges together
pub fn split_during_read(args: Vec<String>) {
    let mut encoder = Encoder::new();

    let (f1, f2, f3, f4) = (
        read_file_split(&args[1], &mut encoder), read_file_split(&args[2], &mut encoder), read_file_split(&args[3], &mut encoder), read_file_split(&args[4], &mut encoder)
    );

    let join_to_map = false;
    if join_to_map {    // ~138m
        join_forth_and_output_from_map(join_first_three_to_map(f1, f2, f3), f4, encoder);   // Use a hashmap for f1-3 for last join
    } else {    // ~133m
        join_forth_and_output(join_first_three(f1, f2, f3), f4, encoder);   // Simply iterate list of f1-3 for last join
    }
}

// Join first three files into a vector of rows
fn join_first_three(f1: FxHashMap<usize, Vec<usize>>, f2: FxHashMap<usize, Vec<usize>>, f3: FxHashMap<usize, Vec<usize>>) -> Vec<GenericArray<usize, U4>> {
    let mut res = Vec::new();
    for key in f1.keys() {
        if !f2.contains_key(key) || !f3.contains_key(key) {
            continue;   // Not in all 3
        }

        // Iterate all combinations
        // TODO: Maybe itertools cross product is faster?
        for x1 in f1.get(key).unwrap() {
            for x2 in f2.get(key).unwrap() {
                for x3 in f3.get(key).unwrap() {
                    res.push(GenericArray::from_array([*key, *x1, *x2, *x3]));
                }
            }
        }
    }

    res
}

// Join together from list of rows + hashmap
fn join_forth_and_output(f1_f2_f3: Vec<GenericArray<usize, U4>>, f4: FxHashMap<usize, Vec<usize>>, encoder: Encoder) {
    for row in f1_f2_f3.iter() {
        match f4.get(&row[3]) {
            Some(list) => {
                for x5 in list {
                    println!("{},{},{},{},{}", encoder.decode(row[3]), encoder.decode(row[0]), encoder.decode(row[1]), encoder.decode(row[2]), encoder.decode(*x5));
                }
            },
            None => {}
        };
    }
}

// Join first three files into a hashmap with the key being the 2nd row of the 3rd file and the value being the remaining entries in the row
fn join_first_three_to_map(f1: FxHashMap<usize, Vec<usize>>, f2: FxHashMap<usize, Vec<usize>>, f3: FxHashMap<usize, Vec<usize>>) -> FxHashMap<usize, Vec<GenericArray<usize, U3>>> {
    let mut res: FxHashMap<usize, Vec<GenericArray<usize, U3>>> = FxHashMap::default();
    for key in f1.keys() {
        if !f2.contains_key(key) || !f3.contains_key(key) {
            continue;   // Not in all 3
        }

        // Iterate all combinations
        // TODO: Maybe itertools cross product is faster?
        for x1 in f1.get(key).unwrap() {
            for x2 in f2.get(key).unwrap() {
                for x3 in f3.get(key).unwrap() {
                    res.entry(*x3).or_default().push(GenericArray::from_array([*key, *x1, *x2]));
                }
            }
        }
    }

    res
}

// Join together from two hashmaps
fn join_forth_and_output_from_map(f1_f2_f3: FxHashMap<usize, Vec<GenericArray<usize, U3>>>, f4: FxHashMap<usize, Vec<usize>>, encoder: Encoder) {
    for x3 in f1_f2_f3.keys() {
        match f4.get(&x3) {
            Some(list) => {
                for arr in f1_f2_f3.get(x3).unwrap() {
                    for x5 in list {
                        println!("{},{},{},{},{}", encoder.decode(*x3), encoder.decode(arr[0]), encoder.decode(arr[1]), encoder.decode(arr[2]), encoder.decode(*x5));
                    }
                }
            },
            None => {}
        };
    }
}