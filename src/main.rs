use std::ops::Range;
use std::collections::BTreeMap;
use rustc_hash::FxHashMap;
use std::env;
use std::fs::read_to_string;
use generic_array::{ArrayLength, GenericArray};
use typenum::{U1, U2, U3, U4, U5};


pub struct Encoder {
    dict: FxHashMap<String, usize>,
    vec: Vec<String>
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            dict: FxHashMap::default(),
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
    let (mut f1, mut f2, mut f3, mut f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );

    let sort_files = true;
    let f1_f2_f3_f4;
    if sort_files{
        // Sorted version iterating in chunks
        sort(&mut f1, 0);
        sort(&mut f2, 0);
        let f1_f2 = join_sorted::<U2, U3>(f1, f2, 0);
        sort(&mut f3, 0);
        let mut f1_f2_f3 = join_sorted::<U3, U4>(f1_f2, f3, 0);
        sort(&mut f1_f2_f3, 3);
        sort(&mut f4, 0);
        f1_f2_f3_f4 = join_sorted::<U4, U5>(f1_f2_f3, f4, 3);
    } else {
        // Unsorted version with occurrence map
        let f1_f2 = join_unsorted::<U2, U3>(f1, f2, 0);
        let f1_f2_f3 = join_unsorted::<U3, U4>(f1_f2, f3, 0);
        f1_f2_f3_f4 = join_unsorted::<U4, U5>(f1_f2_f3, f4, 3);
    }

    for row in f1_f2_f3_f4.iter() {
        println!("{}", row.iter().map(|i| encoder.decode(*i).to_string()).collect::<Vec<String>>().join(","));
    }
}

fn sort<F: ArrayLength>(file: &mut Vec<GenericArray<usize, F>>, pos: usize) {
    file.sort_by_key(|f| f[pos]);
}

fn read_file(file: &String, encoder: &mut Encoder) -> Vec<GenericArray<usize, U2>> {
    read_to_string(file).unwrap().lines().map(
        |line| *GenericArray::from_slice(&line.split(",").map(|x| encoder.encode(x)).collect::<Vec<usize>>())
    ).collect()
}

// Join sorted
// FI = Length of first Input File, FO = Length of Output File
fn join_sorted<FI, FO>(f1: Vec<GenericArray<usize, FI>>, f2: Vec<GenericArray<usize, U2>>, pos_1: usize) -> Vec<GenericArray<usize, FO>> 
where FI: ArrayLength, FO: ArrayLength 
{
    let mut res = Vec::new();
    let mut last = usize::max_value();  // Last processed element
    let mut start = 0;  // Start of that element in f2
    let mut end = 0;    // End of that element in f2

    for r1 in f1.iter() {
        let join_ele = r1[pos_1];
        if last == join_ele {
            // Same as last value, only need to iter range
            update_current(&mut res, start..end, r1, &f2, pos_1);
            continue;
        }

        // First encounter with element: Need to find the correct range
        // TODO: Future optimization: create map in the beginning at once for all elements in f2, so no need to iterate for elements that don't occur
        start = end;
        last = join_ele;
        while start < f2.len() && f2[start][0] != join_ele {
            start += 1;
        }
        if start == f2.len() {
            
            continue; 
        }
        end = start;
        while end < f2.len() && f2[end][0] == join_ele {
            end += 1;
        }

        update_current(&mut res, start..end, r1, &f2, pos_1);
    }

    res
}

fn update_current<FI, FO>(res: &mut Vec<GenericArray<usize, FO>>, range: Range<usize>, r1: &GenericArray<usize, FI>, f2: &Vec<GenericArray<usize, U2>>, pos_1: usize) 
where FI: ArrayLength, FO: ArrayLength
{
    for i2 in range {
        let mut new = GenericArray::default();
        new[0] = r1[pos_1];
        let mut curr = 1;
        for i in 0..FI::to_usize() {
            if i != pos_1 {
                new[curr] = r1[i];
                curr += 1;
            }
        }
        new[curr] = f2[i2][1];
        res.push(new);
    }

}

// Only matches with first position of second line, keeps occurrence map
fn join_unsorted<FI, FO>(f1: Vec<GenericArray<usize, FI>>, f2: Vec<GenericArray<usize, U2>>, pos_1: usize) -> Vec<GenericArray<usize, FO>> 
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
