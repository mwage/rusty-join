use crate::helper::*;
use rustc_hash::FxHashMap;
use compact_str::CompactString;
use std::{cell::UnsafeCell, sync::{Arc, Mutex, RwLock}, thread::{self, JoinHandle}, time::Instant};
use smallvec::SmallVec;
use kanal::unbounded;

// Reads all files into hash maps with the first column as key, then merges together accordingly.
// Values are not encoded, just stored as a string.
pub fn split_no_encode_parallel(args: Vec<String>) {
    let (sender, recv) = unbounded();
    for i in 1..5 {
        let sender = sender.clone();
        let filename = args[i].clone();
        thread::spawn(move || {
            let data = read_file_no_entry_api_small_vec(&filename);
            sender.send((i - 1, data)).unwrap();
        });
    }
    let mut maps: Vec<FxHashMap<CompactString, SmallVec<[CompactString; 5]>>> = vec![FxHashMap::default(); 4];
    for _ in 0..4 {
        let (index, data) = recv.recv().unwrap();
        maps[index] = data;
    }
    
    join_first_three_and_output_with_forth_small_vec(maps);
}

fn join_first_three_and_output_with_forth_small_vec(
    maps: Vec<FxHashMap<CompactString, SmallVec<[CompactString; 5]>>>
) {
    let num = maps[0].len();
    let size = (num + 7) / 8;
    let mut chunks = Vec::with_capacity(8);
    let mut start = 0;
    while start < num {
        let end = (start + size).min(num);
        chunks.push((start, end)); // Collect each chunk as a String
        start = end;
    }
    let (sender, receiver) = unbounded();
    let map = Arc::new(MapWrapper::new(maps));
    for i in 0..chunks.len() {
        let map = Arc::clone(&map);
        let sender = sender.clone();
        let chunk = chunks[i].clone();
        thread::spawn(move || {
            sender.send(gen_buffer(chunk, Arc::clone(&map))).unwrap();      
        });
    };

    for _ in 0..chunks.len() {
        print!("{}", receiver.recv().unwrap());
    }
}

fn gen_buffer(chunks: (usize, usize), map: Arc<MapWrapper>) -> String {
    let mut buffer = String::new();
    for (key, vec1) in map.f1().iter().skip(chunks.0).take(chunks.1 - chunks.0) {
        if let (Some(vec2), Some(vec3)) = (map.f2().get(key), map.f3().get(key)) {
            for x1 in vec1 {
                for x2 in vec2 {
                    for x3 in vec3 {
                        if let Some(vec4) = map.f4().get(x3) {
                            for x4 in vec4 {
                                buffer.push_str(x3);
                                buffer.push(',');
                                buffer.push_str(key);
                                buffer.push(',');
                                buffer.push_str(x1);
                                buffer.push(',');
                                buffer.push_str(x2);
                                buffer.push(',');
                                buffer.push_str(x4);
                                buffer.push('\n');
                            }
                        }
                    }
                }
            }
        }
    }

    buffer
}

#[derive(Debug)]
pub struct MapWrapper {
    maps: UnsafeCell<Vec<FxHashMap<CompactString, SmallVec<[CompactString; 5]>>>>
}

impl MapWrapper {
    pub fn new(
        maps: Vec<FxHashMap<CompactString, SmallVec<[CompactString; 5]>>>) -> MapWrapper {
        MapWrapper {
            maps: UnsafeCell::new(maps)
        }
    }

    pub fn f1(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 5]>> {
        unsafe{&(*self.maps.get())[0]}
    }

    pub fn f2(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 5]>> {
        unsafe{&(*self.maps.get())[1]}
    }

    pub fn f3(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 5]>> {
        unsafe{&(*self.maps.get())[2]}
    }

    pub fn f4(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 5]>> {
        unsafe{&(*self.maps.get())[3]}
    }
}

// #[derive(Debug)]
// pub struct MapWrapper {
//     f1: UnsafeCell<FxHashMap<CompactString, SmallVec<[CompactString; 5]>>>,
//     f2: UnsafeCell<FxHashMap<CompactString, SmallVec<[CompactString; 5]>>>,
//     f3: UnsafeCell<FxHashMap<CompactString, SmallVec<[CompactString; 5]>>>,
//     f4: UnsafeCell<FxHashMap<CompactString, SmallVec<[CompactString; 5]>>>
// }

// impl MapWrapper {
//     pub fn new(
//         f1: FxHashMap<CompactString, SmallVec<[CompactString; 5]>>, 
//         f2: FxHashMap<CompactString, SmallVec<[CompactString; 5]>>,
//         f3: FxHashMap<CompactString, SmallVec<[CompactString; 5]>>, 
//         f4: FxHashMap<CompactString, SmallVec<[CompactString; 5]>>) -> MapWrapper {
//         MapWrapper {
//             f1: UnsafeCell::new(f1),
//             f2: UnsafeCell::new(f2),
//             f3: UnsafeCell::new(f3),
//             f4: UnsafeCell::new(f4),
//         }
//     }

//     pub fn f1(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 5]>> {
//         unsafe{&(*self.f1.get())}
//     }

//     pub fn f2(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 5]>> {
//         unsafe{&(*self.f2.get())}
//     }

//     pub fn f3(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 5]>> {
//         unsafe{&(*self.f3.get())}
//     }

//     pub fn f4(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 5]>> {
//         unsafe{&(*self.f4.get())}
//     }
// }

unsafe impl Sync for MapWrapper {}