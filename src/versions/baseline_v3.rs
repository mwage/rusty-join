use crate::encoder::Encoder;
use generic_array::{ArrayLength, GenericArray};
use typenum::{U2, U3, U4, U5};
use crate::helper::read_file;


// Requires: Baseline, Encoder
// New: Generic Arrays
// Better because arrays can be kept on the stack, Vecs are on the heap
pub fn baseline_v3(args: Vec<String>) {
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
    let f1_f2 = join::<U2, U2, U3>(f1, f2, 0, 0);
    let f1_f2_f3 = join::<U3, U2, U4>(f1_f2, f3, 0, 0);
    let f1_f2_f3_f4 = join::<U4, U2, U5>(f1_f2_f3, f4, 3, 0);
    for row in f1_f2_f3_f4.iter() {
        println!("{}", row.iter().map(|i| encoder.decode(*i).to_string()).collect::<Vec<String>>().join(","));
    }
}

pub fn baseline_v3_read(args: Vec<String>) {
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
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
                for (i, s) in r1.iter().enumerate() {
                    if i != pos_1 {
                        new[curr] = *s;
                        curr += 1;
                    }
                }
                for (i, s) in r2.iter().enumerate() {
                    if i != pos_2 {
                        new[curr] = *s;
                        curr += 1;
                    }
                }
                res.push(new);
            }
        }
    }

    res
}
