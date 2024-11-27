use generic_array::{ArrayLength, GenericArray};
use rustc_hash::FxHashMap;
use typenum::U2;
use std::fs::read_to_string;
use crate::encoder::Encoder;

// Sorts file by key position
pub fn sort<F: ArrayLength>(file: &mut Vec<GenericArray<usize, F>>, pos: usize) {
    file.sort_by_key(|f| f[pos]);
}

// Reads file into a vector of rows (as fixed size arrays)
// TODO: BufReader
// TODO: Try: Split into separate hashmap for each column type 
pub fn read_file(file: &String, encoder: &mut Encoder) -> Vec<GenericArray<usize, U2>> {
    read_to_string(file).unwrap().lines().map(
        |line| *GenericArray::from_slice(&line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>())
    ).collect()
}

// Reads file into a hashmap (key = the different entries, value = list of all elements it appears with)
// TODO: This could probably be done with vectors instead of hash maps.
// TODO: Try: Split into separate hashmap for each column type 
// TODO: BufReader
pub fn read_file_split(file: &String, encoder: &mut Encoder) -> FxHashMap<usize, Vec<usize>> {
    let mut map: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    for line in read_to_string(file).unwrap().lines() {
        let mut split = line.split(",").map(|x| encoder.encode(x));
        map.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }

    map
}

pub fn read_file_split_no_encoding(file: &String) -> FxHashMap<String, Vec<String>> {
    let mut map: FxHashMap<String, Vec<String>> = FxHashMap::default();

    for line in read_to_string(file).unwrap().lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        map.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }

    map
}

/**
 * Reads the file, while avoiding the entry api as it seems to be a bit slower
 */
pub fn read_file_no_entry_api(file: &String) -> FxHashMap<String, Vec<String>> {
    let mut map: FxHashMap<String, Vec<String>> = FxHashMap::default();
    let contents = std::fs::read_to_string(file).unwrap();

    for line in contents.lines() {
        if let Some((key, value)) = line.split_once(',') {
            if let Some(entry) = map.get_mut(key) {
                entry.push(value.to_string());
            } else {
                map.insert(key.to_string(), vec![value.to_string()]);
            }
        }
    }
    map
}