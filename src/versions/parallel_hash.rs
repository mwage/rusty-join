use rustc_hash::{FxBuildHasher, FxHashMap};
use compact_str::CompactString;
use std::{cell::UnsafeCell, sync::Arc, thread};
use smallvec::SmallVec;
use kanal::unbounded;

// Multithreaded version of hash_v9 with implementation level optimizations from reduced_hash (without the algorithmic changes)
pub fn parallel_hash(args: Vec<String>) {
    let (sender, recv) = unbounded();
    for i in 1..5 {
        let sender = sender.clone();
        let filename = args[i].clone();
        thread::spawn(move || {
            let data = read_file_to_map(&filename);
            sender.send((i - 1, data)).unwrap();
        });
    }
    let mut maps: Vec<FxHashMap<CompactString, SmallVec<[CompactString; 1]>>> = vec![FxHashMap::default(); 4];
    for _ in 0..4 {
        let (index, data) = recv.recv().unwrap();
        maps[index] = data;
    }
    
    join(maps);
}

pub fn parallel_hash_read(args: Vec<String>) {
    let (sender, recv) = unbounded();
    for i in 1..5 {
        let sender = sender.clone();
        let filename = args[i].clone();
        thread::spawn(move || {
            let data = read_file_to_map(&filename);
            sender.send((i - 1, data)).unwrap();
        });
    }
    let mut maps: Vec<FxHashMap<CompactString, SmallVec<[CompactString; 1]>>> = vec![FxHashMap::default(); 4];
    for _ in 0..4 {
        let (index, data) = recv.recv().unwrap();
        maps[index] = data;
    }
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

fn join(maps: Vec<FxHashMap<CompactString, SmallVec<[CompactString; 1]>>>) {
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
    maps: UnsafeCell<Vec<FxHashMap<CompactString, SmallVec<[CompactString; 1]>>>>
}

impl MapWrapper {
    pub fn new(
        maps: Vec<FxHashMap<CompactString, SmallVec<[CompactString; 1]>>>) -> MapWrapper {
        MapWrapper {
            maps: UnsafeCell::new(maps)
        }
    }

    pub fn f1(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
        unsafe{&(*self.maps.get())[0]}
    }

    pub fn f2(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
        unsafe{&(*self.maps.get())[1]}
    }

    pub fn f3(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
        unsafe{&(*self.maps.get())[2]}
    }

    pub fn f4(&self) -> &FxHashMap<CompactString, SmallVec<[CompactString; 1]>> {
        unsafe{&(*self.maps.get())[3]}
    }
}

unsafe impl Sync for MapWrapper {}