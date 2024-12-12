
use std::io::{stdout, BufWriter, Write};

use crate::helper::*;

use rustc_hash::{FxBuildHasher, FxHashMap};
use compact_str::CompactString;
use smallvec::SmallVec;

pub fn all_hash(args: Vec<String>){
    /* Hash map for storing merge options of the first three files.
    */
    let mut dict_a: FxHashMap<CompactString, (SmallVec<[CompactString; 1]>,SmallVec<[CompactString; 1]>,SmallVec<[CompactString; 1]>)> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());

    for (key, value) in read_file_no_encoding_compact_split(&args[2]).iter() {
        if let Some(entry) = dict_a.get_mut(key) {
            entry.0.push(value.clone());
        } else {
            let mut vec = SmallVec::new();
            vec.push(value.clone());
            dict_a.insert(key.clone(), (vec, SmallVec::new(), SmallVec::new()));
        }
    }

    // add values of the second column of the second file to the appropriate keys (first column of second file) in dict_a
    for data in read_file_no_encoding_compact_split(&args[1]).iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            entry.1.push(data.1.clone());
        }
    }
    // add values of the second column of the third file to the appropriate keys (first column of first file) in dict_a
    for data in read_file_no_encoding_compact_split(&args[3]).iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            // only add an entry if the join vector of file 2 is not empty
            if !entry.1.is_empty() {
                entry.2.push(data.1.clone());
            }
        }
    }

    let dict_d = read_file_no_entry_api_small_vec_split(&args[4]);
    /* dict_a contains for every key (e.g. first column of first file) several lists that have to be combined
    via a cartesian product, i.e. all combinations have to be generated
    */
    let stdout = stdout();
    let lock = stdout.lock();
    let mut buffer = BufWriter::new(lock);
    for (a_val, (f2_2, f1_2, f3_2)) in dict_a.iter() {
        /* the last vector is only filled if a join is possible (the value occurs in all files), iterating over
        the last vector in the outer loop ensures combinations are only generated for keys where join is possible
        */
        for f3_2_val in f3_2.iter() {
            // fourth file is merged here
            if let Some(f4_2_list) = dict_d.get(f3_2_val) {
                for f4_2_val in f4_2_list.iter() {
                    for f2_2_val in f2_2.iter() {
                        for f1_2_val in f1_2.iter() {
                            buffer.write(f3_2_val.as_bytes());
                            buffer.write(b",");
                            buffer.write(a_val.as_bytes());
                            buffer.write(b",");
                            buffer.write(f1_2_val.as_bytes());
                            buffer.write(b",");
                            buffer.write(f2_2_val.as_bytes());
                            buffer.write(b",");
                            buffer.write(f4_2_val.as_bytes());
                            buffer.write(b"\n");
                        }
                    }
                }
            }
        }
    }

    buffer.flush().unwrap()
}