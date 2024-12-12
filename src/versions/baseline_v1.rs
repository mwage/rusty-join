use std::fs::read_to_string;

// Requires: -
// New: Baseline
pub fn baseline_v1(args: Vec<String>) {
    let (f1, f2, f3, f4) = (read_file(&args[1]), read_file(&args[2]), read_file(&args[3]), read_file(&args[4]));
    let f1_f2 = join(f1, f2, 0, 0);
    let f1_f2_f3 = join(f1_f2, f3, 0, 0);
    let f1_f2_f3_f4 = join(f1_f2_f3, f4, 3, 0);
    for row in f1_f2_f3_f4.iter() {
        println!("{}", row.join(","));
    }
}

pub fn baseline_v1_read(args: Vec<String>) {
    let (f1, f2, f3, f4) = (read_file(&args[1]), read_file(&args[2]), read_file(&args[3]), read_file(&args[4]));
}

fn read_file(file: &String) -> Vec<Vec<String>> {
    read_to_string(file).unwrap().lines().map(|line| line.split(",").map(|x| x.to_string()).collect::<Vec<String>>()).collect()   
}

fn join(f1: Vec<Vec<String>>, f2: Vec<Vec<String>>, pos_1: usize, pos_2: usize) -> Vec<Vec<String>> {
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
