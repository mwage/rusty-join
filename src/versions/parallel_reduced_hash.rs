use rustc_hash::{FxBuildHasher, FxHashMap};
use compact_str::CompactString;
use std::{cell::UnsafeCell, io::{stdout, BufWriter, Write}, sync::Arc, thread};
use smallvec::SmallVec;
use kanal::unbounded;

// Multithreaded version of reduced_hash. Slower since not everything is parallelized
pub fn parallel_reduced_hash(args: Vec<String>) {
    let (sender, recv) = unbounded();
    let (sender_map, recv_map) = unbounded();
    for i in 1..4 {
        let sender = sender.clone();
        let filename = args[i].clone();
        thread::spawn(move || {
            let data = read_file(&filename);
            sender.send((i - 1, data)).unwrap();
        });
    }
    thread::spawn(move || {
        sender_map.send(read_file_to_map(&args[4])).unwrap();
    });

    let mut maps: Vec<Vec<(CompactString, CompactString)>> = vec![Vec::new(); 3];
    for _ in 0..3 {
        let (index, data) = recv.recv().unwrap();
        maps[index] = data;
    }

    let mut dict_a: FxHashMap<CompactString, (SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>)> = FxHashMap::default();
    for (key, value) in maps[1].iter() {
        if let Some(entry) = dict_a.get_mut(key) {
            entry.0.push(value.clone());
        } else {
            let mut vec = SmallVec::new();
            vec.push(value.clone());
            dict_a.insert(key.clone(), (vec, SmallVec::new(), SmallVec::new()));
        }
    }
    for data in maps[0].iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            entry.1.push(data.1.clone());
        }
    }
    for data in maps[2].iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            if !entry.1.is_empty() {
                entry.2.push(data.1.clone());
            }
        }
    }
    
    join(dict_a, recv_map.recv().unwrap());
}

pub fn parallel_reduced_hash_read(args: Vec<String>) {
    let (sender, recv) = unbounded();
    let (sender_map, recv_map) = unbounded();
    for i in 1..4 {
        let sender = sender.clone();
        let filename = args[i].clone();
        thread::spawn(move || {
            let data = read_file(&filename);
            sender.send((i - 1, data)).unwrap();
        });
    }
    thread::spawn(move || {
        sender_map.send(read_file_to_map(&args[5])).unwrap();
    });

    let mut maps: Vec<Vec<(CompactString, CompactString)>> = vec![Vec::new(); 3];
    for _ in 0..3 {
        let (index, data) = recv.recv().unwrap();
        maps[index] = data;
    }

    let mut dict_a: FxHashMap<CompactString, (SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>)> = FxHashMap::default();
    for (key, value) in maps[1].iter() {
        if let Some(entry) = dict_a.get_mut(key) {
            entry.0.push(value.clone());
        } else {
            let mut vec = SmallVec::new();
            vec.push(value.clone());
            dict_a.insert(key.clone(), (vec, SmallVec::new(), SmallVec::new()));
        }
    }
    for data in maps[0].iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            entry.1.push(data.1.clone());
        }
    }
    for data in maps[2].iter() {
        if let Some(entry) = dict_a.get_mut(&data.0) {
            if !entry.1.is_empty() {
                entry.2.push(data.1.clone());
            }
        }
    }

    recv_map.recv().unwrap();
}

pub fn read_file(file: &String) -> Vec<(CompactString, CompactString)> {
    let contents = std::fs::read_to_string(file).unwrap();
    let mut vec = Vec::with_capacity(12000000);
    let mut remainder = contents.as_str();

    while let Some((key, rem)) = remainder.split_once(',') {
        let (value, rem) = rem.split_once('\n').unwrap();
        remainder = rem;
        vec.push((CompactString::from(key), CompactString::from(value)));
    }

    vec
}

fn read_file_to_map(file: &String) -> FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
    let mut map: FxHashMap<CompactString, SmallVec<[CompactString; 1]>> = FxHashMap::with_capacity_and_hasher(5000000, FxBuildHasher::default());
    let contents = std::fs::read_to_string(file).unwrap();
    let mut remainder = contents.as_str();

    while let Some((key, rem)) = remainder.split_once(',') {
        let (value, rem) = rem.split_once('\n').unwrap();

        if let Some(entry) = map.get_mut(key) {
            entry.push(CompactString::from(value));
        } else {
            let mut vec = SmallVec::new();
            vec.push(CompactString::from(value));
            map.insert(CompactString::from(key), vec);
        }

        remainder = rem;
    }
    
    map
}

pub fn join(dict_a: FxHashMap<CompactString, (SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>)>, f4: FxHashMap<CompactString, SmallVec<[CompactString; 1]>>) {
    let num = dict_a.len();
    let size = (num + 7) / 8;
    let mut chunks = Vec::with_capacity(8);
    let mut start = 0;
    while start < num {
        let end = (start + size).min(num);
        chunks.push((start, end)); // Collect each chunk as a String
        start = end;
    }
    let (sender, receiver) = unbounded();
    let map = Arc::new(MapWrapper::new(dict_a, f4));
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
    for (a_val, (f2_2, f1_2, f3_2)) in map.dict().iter().skip(chunks.0).take(chunks.1 - chunks.0) {
        for f3_2_val in f3_2.iter() {
            if let Some(f4_2_list) = map.f4().get(f3_2_val) {
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

    buffer
}

#[derive(Debug)]
pub struct MapWrapper {
    dict: UnsafeCell<FxHashMap<CompactString, (SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>)>>,
    map: UnsafeCell<FxHashMap<CompactString, SmallVec<[CompactString; 1]>>>
}

impl MapWrapper {
    pub fn new(
        dict: FxHashMap<CompactString, (SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>)>,
        map: FxHashMap<CompactString, SmallVec<[CompactString; 1]>>
    ) -> MapWrapper {
        MapWrapper {
            dict: UnsafeCell::new(dict),
            map: UnsafeCell::new(map)
        }
    }

    pub fn dict(&self) -> &FxHashMap<CompactString, (SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>, SmallVec<[CompactString; 1]>)> {
        unsafe {&(*self.dict.get())}
    }

    pub fn f4(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
        unsafe{&(*self.map.get())}
    }
}

unsafe impl Sync for MapWrapper {}