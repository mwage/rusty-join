use rustc_hash::FxHashMap;

pub struct Encoder {
    dict: FxHashMap<String, usize>,
    vec: Vec<String>
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            dict: FxHashMap::default(),
            vec: Vec::new()
        }
    }

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

    pub fn decode(&self, idx: usize) -> &String {
        &self.vec[idx]
    }
}