use std::fs::read_to_string;
use crate::encoder::Encoder;

// Requires: Baseline
// New: Encoder
// Better because int copy is much faster than string copy
pub fn baseline_v2(args: Vec<String>) {
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
    let f1_f2 = join(f1, f2, 0, 0);
    let f1_f2_f3 = join(f1_f2, f3, 0, 0);
    let f1_f2_f3_f4 = join(f1_f2_f3, f4, 3, 0);
    for row in f1_f2_f3_f4.iter() {
        println!("{}", row.iter().map(|i| encoder.decode(*i).to_string()).collect::<Vec<String>>().join(","));
    }
}

pub fn baseline_v2_read(args: Vec<String>) {
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );
}

fn read_file(file: &String, encoder: &mut Encoder) -> Vec<Vec<usize>> {
    read_to_string(file).unwrap().lines().map(|line| line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>()).collect()   
}

fn join(f1: Vec<Vec<usize>>, f2: Vec<Vec<usize>>, pos_1: usize, pos_2: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for r1 in f1.iter() {
        for r2 in f2.iter() {
            if r1[pos_1] == r2[pos_2] {
                let mut new = vec![r1[pos_1].clone()];
                for (i, s) in r1.iter().enumerate() {
                    if i != pos_1 {
                        new.push(s.clone());
                    }
                }
                for (i, s) in r2.iter().enumerate() {
                    if i != pos_2 {
                        new.push(s.clone());
                    }
                }
                res.push(new);
            }
        }
    }

    res
}
