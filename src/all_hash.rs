
use crate::helper::*;
use crate::encoder::Encoder;

use rustc_hash::FxHashMap;

pub fn all_hash(args: Vec<String>){
    let mut encoder = Encoder::new();
    let (f1, f2, f3, dict_d) = (
        read_file(&args[1], &mut encoder), read_file(&args[2], &mut encoder), read_file(&args[3], &mut encoder), read_file_split(&args[4], &mut encoder)
    );

    /* Hash map for storing merge options of every file. The first Vec<(usize,Vec<usize>)> consists of pairs of a value 
    of the first column of the fourth file, together with all associated values of the second column of the fourth file. 
    The second vector contains the values of the second column of the second file while the third vector contains values 
    of the second column of the first file.
    */
    let mut dict_a: FxHashMap<usize, (Vec<(usize,Vec<usize>)>,Vec<usize>,Vec<usize>)> = FxHashMap::default();

    // merge third file with fourth file and store results in hash map dict_a, where the key is the first column of the third file
    for data in f3 {
        if let Some(d_entry) = dict_d.get(&data[1]) {
            match dict_a.get_mut(&data[0]) {
                Some(entry) => { entry.0.push((data[1], d_entry.clone())); }
                None => { dict_a.insert(data[0], (vec![(data[1], d_entry.clone())], Vec::new(), Vec::new())); }
            }
        }
    }
    // add values of the second column of the second file to the appropriate keys (first column of second file) in dict_a
    for data in f2 {
        if let Some(entry) = dict_a.get_mut(&data[0]) {
            entry.1.push(data[1]);
        }
    }
    // add values of the second column of the first file to the appropriate keys (first column of first file) in dict_a
    for data in f1 {
        if let Some(entry) = dict_a.get_mut(&data[0]) {
            // only add an entry if the join vector of file 2 is not empty
            if !entry.1.is_empty() {
                entry.2.push(data[1]);
            }
        }
    }

    /* dict_a contains for every key (e.g. first column of first file) several lists that have to be combined
    via a cartesian product, i.e. all combinations have to be generated
    */
    for (a_val, (f4_data, f2_2, f1_2)) in dict_a.iter() {
        /* the last vector is only filled if a join is possible (the value occurs in all files), iterating over
        the last vector in the outer loop ensures combinations are only generated for keys where join is possible
        */
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