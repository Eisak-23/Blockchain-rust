

struct Block {
    timestamp:u128,
    transactions: String,
    prev_hash: String,
    hash:String,
    heigth: usize,
    nonce:u32,

}

impl Block {
    pub fn new_block(data: String, prev_hash: String, heigth:usize) -> Result<Block> {
        let  timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis();
        // SE CREA LA INTACIA DEL BLOQUE 

        let mut block = Block {
            timestamp,
            transactions: data,
            prev_hash,
            hash: String::new(),
            heigth,
            nonce: 0,
        };
        // validacion de prof of work
        block.run_proof_of_work()?;
        Ok(block)
    }


    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining block");
        while !self.validate {
            self.nonce += 1;

        }
        ///// aca almacenamos el hash que da 0000 
        let data = self.prepare_hash()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())

    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(4, '0' as u8);
        Ok(&hasher.result_str()[0..4] == String::from_utf8(vec1)?)





    }

    fn prepare_hash() ->Result<Vec<u8>> {
        let content = (
            self.prepare_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            4,
            self.nonce,

        );

        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }

}






struct Blockchain {
    blocks: Vec<Block>
}