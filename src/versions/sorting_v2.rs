use crate::encoder::EncoderFx;
use crate::helper::read_file_fx;
use crate::quintuple_sort;


// Requires: Baseline, Encoder, Generic Arrays, Hashmap Join History, Sorting
// New: Range map
// Better because it only iterates the elements that are necessary, NOTE: This is basically merge join
pub fn sorting_v2(args: Vec<String>) {
    quintuple_sort(args);
}

pub fn sorting_v2_read(args: Vec<String>) {
    let mut encoder = EncoderFx::new();
    let (f1, f2, f3, f4) = (
        read_file_fx(&args[1], &mut encoder), read_file_fx(&args[2], &mut encoder), read_file_fx(&args[3], &mut encoder), read_file_fx(&args[4], &mut encoder)
    );
}
