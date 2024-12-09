use crate::helper::*;
use crate::encoder::Encoder;
use compact_str::CompactString;
use generic_array::{ArrayLength, GenericArray};
use typenum::{U2, U3, U4, U5};
use rustc_hash::FxHashMap;

// Joins the columns together by sorting them and then operating on the sorted lists
pub fn quintuple_sort(args: Vec<String>) {
    let (mut f1, mut f2, mut f3, mut f4) = (
        read_file_no_encoding_compact(&args[1]), read_file_no_encoding_compact(&args[2]), read_file_no_encoding_compact(&args[3]), read_file_no_encoding_compact(&args[4])
    );
    
    sort_tuple(&mut f1, 0);
    sort_tuple(&mut f2, 0);
    let f1_f2 = join_sorted(f1, f2, 0);
    sort_tuple(&mut f3, 0);
    let mut f1_f2_f3 = join_sorted(f1_f2, f3, 0);
    sort_tuple(&mut f1_f2_f3, 3);
    sort_tuple(&mut f4, 0);
    let f1_f2_f3_f4 = join_sorted(f1_f2_f3, f4, 3);
    
    for row in f1_f2_f3_f4.iter() {
        // println!("{}", row.iter().map(|i| (*i).to_string()).collect::<Vec<String>>().join(","));
    }
}

// Join sorted
// FI = Length of first Input File, FO = Length of Output File
fn join_sorted(f1: Vec<(CompactString, CompactString)>, f2: Vec<(CompactString, CompactString)>, pos_1: usize) -> Vec<(CompactString, CompactString)> 
{
    let mut res = Vec::new();
    let mut range_map = FxHashMap::default();
    let mut last = CompactString::with_capacity(0);
    let mut start = 0;

    // Create range map (in which range do the individual elements of the second join column start and end)
    for i in 0..f2.len()+1 {
        if i == f2.len() {
            // End of loop, add end for last element
            range_map.insert(last, start..i);
            break;
        }
        // Same element as last
        if f2[i].0 == last {
            continue;
        }

        // New element, add old one
        range_map.insert(last, start..i);
        last = f2[i].0.clone();
        start = i;
    }

    // Go through all elements of first join column and match with all entries in the range of the matching second join column
    for r1 in f1.iter() {
        let range = if pos_1==0 {range_map.get(&r1.0)} else {range_map.get(&r1.1)};
        if range == None { 
            continue; 
        }

        // Found matching entry, for each row in the range, merge together
        for i2 in range.unwrap().clone() {
            let mut new = (CompactString::with_capacity(0),CompactString::with_capacity(0));
            new.0 = if pos_1==0 {r1.0.clone()} else {r1.1.clone()};
            // let mut curr = 1;
            // for i in 0..1 {
            //     if i != pos_1 {
            //         new[curr] = r1[i];
            //         curr += 1;
            //     }
            // }
            new.1 = f2[i2].1.clone() +"," + if pos_1==0 {&r1.1} else {&r1.0};
            res.push(new);
        }
    }

    res
}