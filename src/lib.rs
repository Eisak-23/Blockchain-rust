
use std::time::SystemTime;
use crypto::{digest::Digest, sha2::Sha256};
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, failure::Error>;

const  TARGET_HEIGTH: usize  = 4;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp:u128,
    trasactions:String,
    prev_hash:String,
    hash:String,
    height:usize,
    nonce:u32,
}

impl  Block {
    pub(crate) fn get_prev_hash(&self) -> String{
       self.prev_hash.clone()
    }
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



#[derive(Debug,Clone)]
pub struct Blockchain {
    current_hash:String,
    db:sled::Db
    
}


struct BlockchainIter<'a>  {
    current_hash: String,
    cb: &'a Blockchain, 
}


impl  Blockchain {
    pub fn new() -> Result<Blockchain> {
        let db = sled::open("/data/blocks").unwrap();
        match db.get("LAST")? {
            Some(hash) => {
                let lasthash =  String::from_utf8(hash.to_vec())?;
                Ok(Blockchain {
                    current_hash: lasthash,
                    db,
                })

            }
            None => {
                let block = Block::new_genesiss();
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;
                let bc = Blockchain {
                    current_hash: block.get_hash(),
                    db
                };

                bc.db.flush()?;
                Ok(bc)



            }

        }
       
    }
    
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let lashash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(data, String::from_utf8( lashash.to_vec())?, TARGET_HEIGTH)?;
        self.db.insert(new_block.get_hash(),  bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;

        self.current_hash = new_block.get_hash();
        Ok(())

    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.current_hash.clone(),
            cb: &self
        }
        
    }


}


impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let  Ok(encode_block) =  self.cb.db.get(&self.current_hash) {
            return  match encode_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    }else {
                        None
                    }
                }
               None => None 
            };
        }
    None
    }
    
}



#[cfg(test)]
mod test {
    use colored::Colorize;

    use super::*;
    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new().unwrap();
        b.add_block("data".to_string());
        b.add_block("data1".to_string());
        b.add_block("data2".to_string());
        

        for item  in  b.iter() {
            
            let colored_item = format!("{:?}", item.trasactions).green();
            println!("Item: {}", colored_item);
        }


    }

}