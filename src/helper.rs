use generic_array::{ArrayLength, GenericArray};
use rustc_hash::{FxHashMap, FxBuildHasher};
use typenum::U2;
use std::fs::read_to_string;
use crate::encoder::{Encoder, EncoderFx};
use compact_str::CompactString;
use smallvec::SmallVec;

// Sorts file by key position
pub fn sort<F: ArrayLength>(file: &mut Vec<GenericArray<usize, F>>, pos: usize) {
    file.sort_by_key(|f| f[pos]);
}

// Sorts file by key position
pub fn sort_tuple(file: &mut Vec<(CompactString, CompactString)>, pos: usize) {
    file.sort_by_key(|f| if pos == 0 {f.0.clone() } else {f.1.clone()} );
}

// Reads file into a vector of rows (as fixed size arrays)
// Encoder version w/o hash map
pub fn read_file(file: &String, encoder: &mut Encoder) -> Vec<GenericArray<usize, U2>> {
    read_to_string(file).unwrap().lines().map(
        |line| *GenericArray::from_slice(&line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>())
    ).collect()
}

pub fn read_file_fx(file: &String, encoder: &mut EncoderFx) -> Vec<GenericArray<usize, U2>> {
    read_to_string(file).unwrap().lines().map(
        |line| *GenericArray::from_slice(&line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>())
    ).collect()
}

// Reads file into a vector of rows (as fixed size arrays)
// First version unencoded no hashmap
pub fn read_file_no_encoding(file: &String) -> Vec<(String, String)> {
    read_to_string(file).unwrap().lines().map(
        |line| { let mut split=line.split(","); (split.next().unwrap().to_string(), split.next().unwrap().to_string()) }
    ).collect()
}

// Second version unencoded (with compact strings) no hashmap
pub fn read_file_no_encoding_compact(file: &String) -> Vec<(CompactString, CompactString)> {
    read_to_string(file).unwrap().lines().map(
        |line| { let (key,val)=line.split_once(",").unwrap(); (CompactString::from(key), CompactString::from(val)) }
    ).collect()
}

pub fn read_file_no_encoding_compact_split(file: &String) -> Vec<(CompactString, CompactString)> {

    let contents = std::fs::read_to_string(file).unwrap();
    let mut vec = Vec::with_capacity(12000000);
    let mut remainder = contents.as_str();

    while let Some((key, rem)) = remainder.split_once(',') {
        let (value, rem) = rem.split_once('\n').unwrap();
        remainder = rem;
        vec.push((CompactString::from(key), CompactString::from(value)));
    }

    vec
}

// Reads file into a hashmap (key = the different entries, value = list of all elements it appears with)
// Encoder version with hash map
pub fn read_file_split(file: &String, encoder: &mut Encoder) -> FxHashMap<usize, Vec<usize>> {
    let mut map: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    for line in read_to_string(file).unwrap().lines() {
        let mut split = line.split(",").map(|x| encoder.encode(x));
        map.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }

    map
}

// First version hashmap parse 
pub fn read_file_split_no_encoding(file: &String) -> FxHashMap<String, Vec<String>> {
    let mut map: FxHashMap<String, Vec<String>> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());

    for line in read_to_string(file).unwrap().lines() {
        let mut split = line.split(",").map(|x| x.to_string());
        map.entry(split.next().unwrap())
            .or_insert_with(|| Vec::with_capacity(5))
            .push(split.next().unwrap());
    }

    map
}

// Second version hashmap parse (with compact strings)
pub fn read_file_split_no_encoding_compact(file: &String) -> FxHashMap<CompactString, Vec<CompactString>> {
    let mut map: FxHashMap<CompactString, Vec<CompactString>> = FxHashMap::default();

    for line in read_to_string(file).unwrap().lines() {
        let mut split = line.split(",").map(|x| CompactString::from(x));
        map.entry(split.next().unwrap()).or_default().push(split.next().unwrap());
    }

    map
}

/**
 * Reads the file, while avoiding the entry api as it seems to be a bit slower
 */
// Third version hashmap parse (with capacity and no entry API)
pub fn read_file_no_entry_api(file: &String) -> FxHashMap<CompactString, Vec<CompactString>> {
    let mut map: FxHashMap<CompactString, Vec<CompactString>> = FxHashMap::default();
    let contents = std::fs::read_to_string(file).unwrap();

    for line in contents.lines() {
        let (key, value) = line.split_once(',').unwrap();
        if let Some(entry) = map.get_mut(key) {
            entry.push(CompactString::from(value));
        } else {
            map.insert(CompactString::from(key), vec![CompactString::from(value)]);
        }
    }
    map
}

// Fourth version hashmap parse (with capacity on hashMap and Vec and no entry API)
pub fn read_file_no_entry_api_prealloc_vec(file: &String) -> FxHashMap<CompactString, Vec<CompactString>> {
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

// Fifth version hashmap parse (with capacity, SmallVec and no entry API)
pub fn read_file_no_entry_api_small_vec(file: &String) -> FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
    let mut map: FxHashMap<CompactString, SmallVec<[CompactString; 1]>> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());
    let contents = std::fs::read_to_string(file).unwrap();

    for line in contents.lines() {
        let (key, value) = line.split_once(',').unwrap();
        if let Some(entry) = map.get_mut(key) {
            entry.push(CompactString::from(value));
        } else {
            // ~2m better than without ::with_capacity(5)
            let mut vec = SmallVec::new();
            vec.push(CompactString::from(value));
            map.insert(CompactString::from(key), vec);
        }
    }
    map
}

// Fifth version hashmap parse (with capacity, SmallVec and no entry API)
pub fn read_file_no_entry_api_small_vec_split(file: &String) -> FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
    let mut map: FxHashMap<CompactString, SmallVec<[CompactString; 1]>> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());
    let contents = std::fs::read_to_string(file).unwrap();

    let mut remainder = contents.as_str();

    while let Some((key, rem)) = remainder.split_once(',') {
        let (value, rem) = rem.split_once('\n').unwrap();

        if let Some(entry) = map.get_mut(key) {
            entry.push(CompactString::from(value));
        } else {
            let mut vec = SmallVec::new();
            vec.push(CompactString::from(value));
            map.insert(CompactString::from(key), vec);
        }

        remainder = rem;
    }
    map
}