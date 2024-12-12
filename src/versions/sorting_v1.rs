use std::collections::HashMap;

use crate::encoder::EncoderFx;
use generic_array::{ArrayLength, GenericArray};
use rustc_hash::FxHashMap;
use typenum::{U2, U3, U4, U5};
use crate::helper::read_file_fx;


// Requires: Baseline, Encoder, Generic Arrays, loop unrolling, Hashmap Join History
// New: Sorting
// Better because less branch prediction misses due to same element occuring often in a row?
pub fn sorting_v1(args: Vec<String>) {
    let mut encoder = EncoderFx::new();
    let (mut f1, mut f2, mut f3, mut f4) = (
        read_file_fx(&args[1], &mut encoder), read_file_fx(&args[2], &mut encoder), read_file_fx(&args[3], &mut encoder), read_file_fx(&args[4], &mut encoder)
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

pub fn sorting_v1_read(args: Vec<String>) {
    let mut encoder = EncoderFx::new();
    let (f1, f2, f3, f4) = (
        read_file_fx(&args[1], &mut encoder), read_file_fx(&args[2], &mut encoder), read_file_fx(&args[3], &mut encoder), read_file_fx(&args[4], &mut encoder)
    );
}

fn sort<F: ArrayLength>(vec: &mut Vec<GenericArray<usize, F>>, pos: usize) {
    vec.sort_by_key(|f| f[pos]);
}


fn join<F1, F2, F3>(f1: Vec<GenericArray<usize, F1>>, f2: Vec<GenericArray<usize, F2>>, pos_1: usize) -> Vec<GenericArray<usize, F3>> 
where F1: ArrayLength, F2: ArrayLength, F3: ArrayLength 
{
    let mut res = Vec::new();
    let mut seen: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    for (i1, r1) in f1.iter().enumerate() {
        match seen.get(&i1) {
            Some(list) => {
                for other in list.iter() {
                    let mut new = GenericArray::default();
                    new[0] = r1[pos_1];
                    let mut curr = 1;
                    for i in 0..F1::to_usize() {
                        if i != pos_1 {
                            new[curr] = r1[i];
                            curr += 1;
                        }
                    }
                    new[curr] = *other;
                    res.push(new);
                }
            },
            None => {
                let mut list = Vec::new();
                for r2 in f2.iter() {
                    if r1[pos_1] == r2[0] {
                        let mut new = GenericArray::default();
                        new[0] = r1[pos_1];
                        let mut curr = 1;
                        for i in 0..F1::to_usize() {
                            if i != pos_1 {
                                new[curr] = r1[i];
                                curr += 1;
                            }
                        }
                        new[curr] = r2[1];
                        res.push(new);
                        list.push(r2[1]);
                    }
                }
                seen.insert(i1, list);
            }
        };
    }

    res
}
