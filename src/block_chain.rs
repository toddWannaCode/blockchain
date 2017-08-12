use super::block::Block;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem::replace;
use std::fmt;

type Chain = Option<Rc<RefCell<BlockChain>>>;

pub struct BlockChain {
    elem_block: Block,
    hash_pointer: Option<u64>,
    rest: Chain,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let elem_block = Block::new(0, "Genesis Block".to_owned());
        BlockChain {
            elem_block,
            hash_pointer: None,
            rest: None,
        }
    }

    pub fn append(&mut self, data: String) -> Result<(), String> {
        let chain = self.rest.take();
        let mut previous_block = replace(&mut self.elem_block, Block::empty());
        let previous_pointer = self.hash_pointer.take();
        self.hash_pointer = previous_block.get_self_hash();
        let mut block = Block::new(previous_block.get_index() + 1, data);
        previous_block.set_next_hash(block.get_self_hash())?;
        block.set_previous_hash(previous_block.get_self_hash())?;
        self.rest = Some(Rc::new(RefCell::new(BlockChain {
            elem_block: previous_block,
            hash_pointer: previous_pointer,
            rest: chain,
        })));
        self.elem_block = block;
        Ok(())
    }
}

impl fmt::Display for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chain = if self.rest.is_some() {
            let chain = &*self.rest.as_ref().cloned().unwrap();
            let chain = &*chain.borrow();
            format!("{}", chain)
        } else {
            "Beginning is the most important part of the work.\n                                           -Plato".to_owned()
        };
        let hash_pointer = match self.hash_pointer.as_ref() {
            Some(ref value) => format!("{}", value),
            None => "None".to_owned(),
        };

        write!(
            f,
            "{{\n{},\n  index: {},\n  hash_pointer: {:?} \n}}\n\n{}",
            self.elem_block,
            self.elem_block.get_index(),
            hash_pointer,
            chain
        )
    }
}

#[cfg(test)]
mod test {
    use super::BlockChain;

    #[test]
    fn basics() {
        let mut b = BlockChain::new();
        b.append("Alice gave to Bob".to_owned()).unwrap();
        b.append("Bob gave to Alice".to_owned()).unwrap();
        println!("{}", b);
        b.append("Charlie gave Alice 30".to_owned()).unwrap();
        println!("{}", b);
    }
}
