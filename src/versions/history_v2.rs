use std::collections::HashMap;

use crate::encoder::Encoder;
use generic_array::{ArrayLength, GenericArray};
use typenum::{U2, U3, U4, U5};
use std::fs::read_to_string;


// Requires: Baseline, Encoder, Generic Arrays, loop unrolling, Join History
// New: Hashmap Join History
// Better because BTreeMap scales with log(n) => Hashmap is better for large n
pub fn history_v2(args: Vec<String>) {
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
    let f1_f2 = join::<U2, U2, U3>(f1, f2, 0);
    let f1_f2_f3 = join::<U3, U2, U4>(f1_f2, f3, 0);
    let f1_f2_f3_f4 = join::<U4, U2, U5>(f1_f2_f3, f4, 3);
    for row in f1_f2_f3_f4.iter() {
        println!("{}", row.iter().map(|i| encoder.decode(*i).to_string()).collect::<Vec<String>>().join(","));
    }
}

pub fn history_v2_read(args: Vec<String>) {
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
}

fn read_file(file: &String, encoder: &mut Encoder) -> Vec<GenericArray<usize, U2>> {
    read_to_string(file).unwrap().lines().map(
        |line| *GenericArray::from_slice(&line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>())
    ).collect()
}

fn join<F1, F2, F3>(f1: Vec<GenericArray<usize, F1>>, f2: Vec<GenericArray<usize, F2>>, pos_1: usize) -> Vec<GenericArray<usize, F3>> 
where F1: ArrayLength, F2: ArrayLength, F3: ArrayLength 
{
    let mut res = Vec::new();
    let mut seen: HashMap<usize, Vec<usize>> = HashMap::new();
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
