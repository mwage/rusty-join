use std::fs::read_to_string;

use generic_array::{ArrayLength, GenericArray};
use rustc_hash::FxHashMap;
use typenum::{U2, U3, U4, U5};

use crate::encoder::EncoderFx;


// Requires: Baseline, Encoder, Generic Arrays, Sorting, Range map, 
// New: Faster Hashmap
// Better because non-cryptographic hash
pub fn sorting_v3(args: Vec<String>) {
    let mut encoder = EncoderFx::new();
    let (mut f1, mut f2, mut f3, mut f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
    
    sort(&mut f1, 0);
    sort(&mut f2, 0);
    let f1_f2 = join::<U2, U2, U3>(f1, f2, 0);
    sort(&mut f3, 0);
    let mut f1_f2_f3 = join::<U3, U2, U4>(f1_f2, f3, 0);
    sort(&mut f1_f2_f3, 3);
    sort(&mut f4, 0);
    let f1_f2_f3_f4 = join::<U4, U2, U5>(f1_f2_f3, f4, 3);
    
    for row in f1_f2_f3_f4.iter() {
        println!("{}", row.iter().map(|i| encoder.decode(*i).to_string()).collect::<Vec<String>>().join(","));
    }
}

pub fn sorting_v3_read(args: Vec<String>) {
    let mut encoder = EncoderFx::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
}

pub fn read_file(file: &String, encoder: &mut EncoderFx) -> Vec<GenericArray<usize, U2>> {
    read_to_string(file).unwrap().lines().map(
        |line| *GenericArray::from_slice(&line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>())
    ).collect()
}

fn sort<F: ArrayLength>(vec: &mut Vec<GenericArray<usize, F>>, pos: usize) {
    vec.sort_by_key(|f| f[pos]);
}

fn join<F1, F2, F3>(f1: Vec<GenericArray<usize, F1>>, f2: Vec<GenericArray<usize, F2>>, pos_1: usize) -> Vec<GenericArray<usize, F3>> 
where F1: ArrayLength, F2: ArrayLength, F3: ArrayLength 
{
    let mut res = Vec::new();
    let mut range_map = FxHashMap::default();
    let mut last = usize::max_value();
    let mut start = 0;

    // Create range map (in which range do the individual elements of the second join column start and end)
    for i in 0..f2.len()+1 {
        if i == f2.len() {
            // End of loop, add end for last element
            range_map.insert(last, start..i);
            break;
        }
        // Same element as last
        if f2[i][0] == last {
            continue;
        }

        // New element, add old one
        range_map.insert(last, start..i);
        last = f2[i][0];
        start = i;
    }

    // Go through all elements of first join column and match with all entries in the range of the matching second join column
    for r1 in f1.iter() {
        let range = range_map.get(&r1[pos_1]);
        if range == None { 
            continue; 
        }

        // Found matching entry, for each row in the range, merge together
        for i2 in range.unwrap().clone() {
            let mut new = GenericArray::default();
            new[0] = r1[pos_1];
            let mut curr = 1;
            for i in 0..F1::to_usize() {
                if i != pos_1 {
                    new[curr] = r1[i];
                    curr += 1;
                }
            }
            new[curr] = f2[i2][1];
            res.push(new);
        }
    }

    res
}