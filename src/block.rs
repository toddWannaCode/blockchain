use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use chrono::prelude::*;

#[derive(Debug)]
pub struct Block {
    index: u32,
    pub timestamp: DateTime<Utc>,
    pub data: String,
    pub previous_hash: Option<u64>,
    pub next_hash: Option<u64>,
    pub self_hash: Option<u64>
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.index.hash(state);
        self.timestamp.hash(state);
        self.data.hash(state);
    }
}

impl Block {
    pub fn new(index: u32, data: String) -> Block{
        let temp = Block {
            index,
            data,
            timestamp: Utc::now(),
            previous_hash: None,
            next_hash: None,
            self_hash: None
        };
        let self_hash = Some(calculate_hash(&temp));
        Block {
            self_hash,
            ..temp
        }
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let b = Block::new(1, "Test".to_string());
        println!("{:?}", b);
    }
}
