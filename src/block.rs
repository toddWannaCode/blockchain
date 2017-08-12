use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use chrono::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    data: String,
    previous_hash: Option<u64>,
    next_hash: Option<u64>,
    self_hash: Option<u64>,
    
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H){
        self.index.hash(state);
        self.timestamp.hash(state);
        self.data.hash(state);
    }
}

impl Block {
    
    pub fn new(index: u64, data: String) -> Block{
        let temp = Block {
            index,
            data,
            timestamp: Utc::now(),
            previous_hash: None,
            next_hash: None,
            self_hash: None
        };
        let self_hash = Some(Block::calculate_hash(&temp));
        Block {
            self_hash,
            ..temp
        }
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    pub fn set_next_hash(&mut self, hash: Option<u64>) -> Result<(), String>{
        if self.next_hash.is_none() {
            if self.check_hash() {
               self.next_hash = hash;
               Ok(())
            } else {
                Err(format!("Block {} has been tampered.", self.index))
            }
        } else {
            Err(format!("Next hash has already been set for {}.", self.index))
        }
    }

    pub fn get_next_hash(&self) -> Option<u64> {
        self.next_hash
    }

    pub fn set_previous_hash(&mut self, hash: Option<u64>) -> Result<(), String> {
         if self.previous_hash.is_none() {
            if self.check_hash() {
               self.previous_hash = hash;
               Ok(())
            } else {
                Err(format!("Block {} has been tampered", self.index))
            }
        } else {
            Err(format!("Previous hash has already been set for {}.", self.index))
        }
 
    }
    pub fn get_previous_hash(&self) -> Option<u64> {
        self.previous_hash
    }
    pub fn get_index(&self) -> u64 {
        self.index
    }
    
    pub fn get_self_hash(&self) -> Option<u64> {
        self.self_hash
    }
    
    fn check_hash(&self) -> bool {
        self.self_hash == Some(Block::calculate_hash(&self))
    }
    
    pub fn empty() -> Block{
        Block {
            index: 0,
            data: String::new(),
            timestamp: Utc::now(),
            previous_hash: None,
            next_hash: None,
            self_hash: None
        }
    }
     
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hash = match self.self_hash.as_ref() {
            Some(ref value) => format!("{}",value),
            None            => "None".to_owned()
        };

        write!(f, "  Block: {{\n    timestamp: {},\n    data: {},\n    hash: {}\n  }}", self.timestamp, self.data, hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let b = Block::new(1, "Genesis".to_string());
        println!("{}", b.self_hash.as_ref().unwrap());
    }
}
