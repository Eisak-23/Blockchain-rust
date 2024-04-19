use std::time::SystemTime;

use crypto::{digest::Digest, sha2::Sha256};

pub type Result<T> = std::result::Result<T, failure::Error>;

const  TARGET_HEIGTH: usize  = 4;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp:u128,
    trasactions:String,
    prev_hash:String,
    hash:String,
    height:usize,
    nonce:u32,
}


impl Block {

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    } 

    pub fn new_genesiss() -> Block {
        Block::new_block(String::from("genesis"), String::new(), 0).unwrap()
    }

    pub fn new_block(data: String, prev_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();

        let mut block = Block {
            timestamp,
            trasactions: data,
            prev_hash,
            hash: String::new(),
            height,
            nonce: 0,

        
        };
        block.proof_of_work()?;
        Ok(block)

    } 

    fn proof_of_work(&mut self) -> Result<()> {

        while !self.validate()? {
            self.nonce +=1;
        }

        let data = self.prepare_hash()?;
        let mut  hasher = Sha256::new();
        hasher.input(&data);

        self.hash = hasher.result_str();
        Ok(())


    
    }


    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash()?;
        let mut  hasher = Sha256::new();
        hasher.input(&data);
        let  mut vec1: Vec<u8>  = vec![];
        vec1.resize(TARGET_HEIGTH, '0' as u8);
        Ok(&hasher.result_str()[0..4] == String::from_utf8(vec1)?)
    }



    fn prepare_hash(&self) -> Result<Vec<u8>> {
        let content = (
            self.trasactions.clone(),
            self.prev_hash.clone(),
            self.timestamp,
            TARGET_HEIGTH,
            self.nonce
        );

        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }
}




#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>
}
impl  Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesiss()]
        }
    }
    
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev  = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), TARGET_HEIGTH)?;
        self.blocks.push(new_block);
        Ok(())

    }

}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new();
        b.add_block("data".to_string());
        b.add_block("data1".to_string());
        b.add_block("data2".to_string());
        dbg!(b);

    }

}