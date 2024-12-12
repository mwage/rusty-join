use rustc_hash::FxHashMap;

// Encodes the strings into integers to speed up the join procedures (copy, comparisons, etc.)
pub struct EncoderFx {
    dict: FxHashMap<String, usize>,
    vec: Vec<String>
}

impl EncoderFx {
    pub fn new() -> Self {
        EncoderFx {
            dict: FxHashMap::default(),
            vec: Vec::new()
        }
    }

    // Encode to int
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

    // Decode int to string
    pub fn decode(&self, idx: usize) -> &String {
        &self.vec[idx]
    }
}

use std::collections::HashMap;

// Encodes the strings into integers to speed up the join procedures (copy, comparisons, etc.)
pub struct Encoder {
    dict: HashMap<String, usize>,
    vec: Vec<String>
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            dict: HashMap::default(),
            vec: Vec::new()
        }
    }

    // Encode to int
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

    // Decode int to string
    pub fn decode(&self, idx: usize) -> &String {
        &self.vec[idx]
    }
}