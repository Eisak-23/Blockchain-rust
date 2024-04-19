use std::time::SystemTime;


use bincode::Error;
use crypto::{digest::Digest, sha2::Sha256};
fn main() -> () {
    let Timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("gaa").as_millis();

    let content = (String::new(),String::new(),Timestamp,4,7);
    let bytes = bincode::serialize(&content).expect("error");

    println!("{:?}", bytes);
    
    let mut hasher = Sha256::new();
    hasher.input(&bytes[..]);
    let mut vec1: Vec<u8> = vec![];
    vec1.resize(4, '0' as u8);
    println!("{:?}", vec1);
    let hash_result = hasher.result_str();
    println!("{}",&hasher.result_str()[0..4]);
    println!("{}", String::from_utf8(vec1).expect("gaa"));

    
}