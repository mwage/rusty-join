use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::read_to_string;
use generic_array::{ArrayLength, GenericArray};
use typenum::{U1, U2, U3, U4, U5};


pub struct Encoder {
    dict: HashMap<String, usize>,
    vec: Vec<String>
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            dict: HashMap::new(),
            vec: Vec::new()
        }
    }

    pub fn encode(&mut self, value: &str) -> usize {
        match self.dict.get(value) {
            Some(x) => *x,
            None => {
                let k = self.vec.len() as usize;
                self.dict.insert(value.to_string(), k);
                self.vec.push(value.to_string());

                k
            }
        }
    }

    pub fn decode(&self, idx: usize) -> &String {
        &self.vec[idx]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );

    let f1_f2 = join::<U2, U3>(f1, f2, 0);
    let f1_f2_f3 = join::<U3, U4>(f1_f2, f3, 0);
    let f1_f2_f3_f4 = join::<U4, U5>(f1_f2_f3, f4, 3);
    for row in f1_f2_f3_f4.iter() {
        println!("{}", row.iter().map(|i| encoder.decode(*i).to_string()).collect::<Vec<String>>().join(","));
    }
}

fn read_file(file: &String, encoder: &mut Encoder) -> Vec<GenericArray<usize, U2>> {
    read_to_string(file).unwrap().lines().map(
        |line| *GenericArray::from_slice(&line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>())
    ).collect()
}

// Only matches with first position of second line
fn join<FI, FO>(f1: Vec<GenericArray<usize, FI>>, f2: Vec<GenericArray<usize, U2>>, pos_1: usize) -> Vec<GenericArray<usize, FO>> 
where FI: ArrayLength, FO: ArrayLength 
{
    let mut res = Vec::new();
    let mut seen: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (i1, r1) in f1.iter().enumerate() {
        match seen.get(&i1) {
            Some(list) => {
                for other in list.iter() {
                    let mut new = GenericArray::default();
                    new[0] = r1[pos_1];
                    let mut curr = 1;
                    for i in 0..FI::to_usize() {
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
                        for i in 0..FI::to_usize() {
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
