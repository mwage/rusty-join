use crate::encoder::Encoder;
use generic_array::{ArrayLength, GenericArray};
use typenum::{U2, U3, U4, U5};
use std::fs::read_to_string;


// Requires: Baseline, Encoder, Generic Arrays, loop unrolling
// New: sorting
// Overhead insignificant, but we can now use the fact that it is sorted
pub fn sorting_v1(args: Vec<String>) {
    let mut encoder = Encoder::new();
    let (mut f1, mut f2, mut f3, mut f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
    sort(&mut f1, 0);
    sort(&mut f2, 0);
    let f1_f2 = join::<U2, U2, U3>(f1, f2, 0, 0);
    sort(&mut f3, 0);
    let mut f1_f2_f3 = join::<U3, U2, U4>(f1_f2, f3, 0, 0);
    sort(&mut f1_f2_f3, 3);
    sort(&mut f4, 0);
    let f1_f2_f3_f4 = join::<U4, U2, U5>(f1_f2_f3, f4, 3, 0);
    for row in f1_f2_f3_f4.iter() {
        println!("{}", row.iter().map(|i| encoder.decode(*i).to_string()).collect::<Vec<String>>().join(","));
    }
}

pub fn sorting_v1_read(args: Vec<String>) {
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
}

fn sort<F: ArrayLength>(vec: &mut Vec<GenericArray<usize, F>>, pos: usize) {
    vec.sort_by_key(|f| f[pos]);
}

fn read_file(file: &String, encoder: &mut Encoder) -> Vec<GenericArray<usize, U2>> {
    read_to_string(file).unwrap().lines().map(
        |line| *GenericArray::from_slice(&line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>())
    ).collect()
}

fn join<F1, F2, F3>(f1: Vec<GenericArray<usize, F1>>, f2: Vec<GenericArray<usize, F2>>, pos_1: usize, pos_2: usize) -> Vec<GenericArray<usize, F3>> 
where F1: ArrayLength, F2: ArrayLength, F3: ArrayLength 
{
    let mut res = Vec::new();
    for r1 in f1.iter() {
        for r2 in f2.iter() {
            if r1[pos_1] == r2[pos_2] {
                let mut new = GenericArray::default();
                new[0] = r1[pos_1];
                let mut curr = 1;
                for i in 0..F1::to_usize() {
                    if i != pos_1 {
                        new[curr] = r1[i];
                        curr += 1;
                    }
                }
                for i in 0..F2::to_usize() {
                    if i != pos_2 {
                        new[curr] = r2[i];
                        curr += 1;
                    }
                }
                res.push(new);
            }
        }
    }

    res
}
