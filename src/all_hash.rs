
use crate::helper::*;

use rustc_hash::FxHashMap;
use compact_str::CompactString;

pub fn all_hash(args: Vec<String>){
    let (f1, f2, f3, dict_d) = (
        read_file_no_encoding_compact(&args[1]), read_file_no_encoding_compact(&args[2]), read_file_no_encoding_compact(&args[3]), read_file_no_entry_api(&args[4])
    );

    /* Hash map for storing merge options of every file. The first Vec<(usize,Vec<usize>)> consists of pairs of a value 
    of the first column of the fourth file, together with all associated values of the second column of the fourth file. 
    The second vector contains the values of the second column of the second file while the third vector contains values 
    of the second column of the first file.
    */
    let mut dict_a: FxHashMap<CompactString, (Vec<CompactString>,Vec<CompactString>,Vec<CompactString>)> = FxHashMap::default();

    // store key and values of first file in hash map dict_a, where the key is the first column of the third file
    for data in f1.iter() {
        match dict_a.get_mut(&data.0) {
            Some(entry) => { entry.0.push(data.1.clone()); }
            None => { dict_a.insert(data.0.clone(), (vec![data.1.clone()], Vec::new(), Vec::new())); }
        }
    }
    // add values of the second column of the second file to the appropriate keys (first column of second file) in dict_a
    for data in f2.iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            entry.1.push(data.1.clone());
        }
    }
    // add values of the second column of the third file to the appropriate keys (first column of first file) in dict_a
    for data in f3.iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            // only add an entry if the join vector of file 2 is not empty
            if !entry.1.is_empty() {
                entry.2.push(data.1.clone());
            }
        }
    }

    /* dict_a contains for every key (e.g. first column of first file) several lists that have to be combined
    via a cartesian product, i.e. all combinations have to be generated
    */
    let mut buffer = String::new();
    for (a_val, (f1_2, f2_2, f3_2)) in dict_a.iter() {
        /* the last vector is only filled if a join is possible (the value occurs in all files), iterating over
        the last vector in the outer loop ensures combinations are only generated for keys where join is possible
        */
        for f3_2_val in f3_2.iter() {
            // fourth file is merged here
            if let Some(f4_2_list) = dict_d.get(f3_2_val) {
                for f4_2_val in f4_2_list.iter() {
                    for f2_2_val in f2_2.iter() {
                        for f1_2_val in f1_2.iter() {
                            buffer.push_str(f3_2_val);
                            buffer.push(',');
                            buffer.push_str(a_val);
                            buffer.push(',');
                            buffer.push_str(f1_2_val);
                            buffer.push(',');
                            buffer.push_str(f2_2_val);
                            buffer.push(',');
                            buffer.push_str(f4_2_val);
                            buffer.push('\n');
                        }
                    }
                }
            }
        }
    }

    print!("{}", buffer);
}