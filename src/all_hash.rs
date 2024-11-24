
use crate::helper::*;
use crate::encoder::Encoder;

use rustc_hash::FxHashMap;

pub fn all_hash(args: Vec<String>){
    let mut encoder = Encoder::new();
    let (f1, f2, f3, f4) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file(&args[4], &mut encoder)
    );

    let mut dict_d: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
    for data in f4 {
        match dict_d.get_mut(&data[0]) {
            Some(entry) => { entry.push(data[1]); }
            None => { dict_d.insert(data[0], vec![data[1]]); }
        }
    }

    let mut dict_a: FxHashMap<usize, (Vec<(usize,Vec<usize>)>,Vec<usize>,Vec<usize>)> = FxHashMap::default();
    for data in f3 {
        if let Some(d_entry) = dict_d.get(&data[1]) {
            match dict_a.get_mut(&data[0]) {
                Some(entry) => { entry.0.push((data[1], d_entry.clone())); }
                None => { dict_a.insert(data[0], (vec![(data[1], d_entry.clone())], Vec::new(), Vec::new())); }
            }
        }
    }
    for data in f2 {
        if let Some(entry) = dict_a.get_mut(&data[0]) {
            entry.1.push(data[1]);
        }
    }
    for data in f1 {
        if let Some(entry) = dict_a.get_mut(&data[0]) {
            if !entry.1.is_empty() {
                entry.2.push(data[1]);
            }
        }
    }

    for (a_val, (f4_data, f2_2, f1_2)) in dict_a.iter() {
        for f1_2_val in f1_2 {
            for (f4_1, f4_2_list) in f4_data.iter() {
                for f4_2_val in f4_2_list {
                    for f2_2_val in f2_2.iter() {
                        println!("{},{},{},{},{}", encoder.decode(*f4_1), encoder.decode(*a_val), encoder.decode(*f1_2_val), encoder.decode(*f2_2_val), encoder.decode(*f4_2_val));
                    }
                }
            }
        }
    }
}