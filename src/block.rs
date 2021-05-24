use std::convert::TryInto;

use sha2::{Digest, Sha256};

static DIFFICULTY: u8 = 16;

type Hash = [u8; 32];

#[derive(Debug)]
pub struct Block {
    nonce: u32,
    previous_hash: Hash,
    data: Vec<u8>,
    hash: Hash,
}

impl Block {
    
    fn genesis(data: Vec<u8>) -> Self {
        Block::new([0; 32], data)
    }

    fn chain(&self, data: Vec<u8>) -> Self {
        Block::new(self.hash, data)
    }

    fn new(previous_hash: Hash, data: Vec<u8>) -> Self {

        let mut block = Block {
            nonce: 0,
            previous_hash,
            data,
            hash: [0; 32],
        };

        block.mine();
        block
    }

    pub fn from(previous_hash: Hash, data: Vec<u8>, nonce: u32, hash: Hash) -> Self {
        Block {
            nonce,
            previous_hash,
            data,
            hash,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.nonce.to_be_bytes());
        bytes.extend_from_slice(&self.previous_hash[..]);
        bytes.extend_from_slice(&self.data[..]);
        bytes
    }

    fn calculate_hash(bytes: &Vec<u8>) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().try_into().unwrap()
    }

    fn prefix(hash: &Hash) -> u32 {
        u32::from_be_bytes(hash[0..4].try_into().unwrap()) >> (32 - DIFFICULTY)
    }

    fn mine(&mut self) {

        let mut bytes = self.to_bytes();

        loop {
            let nonce_bytes = self.nonce.to_be_bytes();
            bytes[0] = nonce_bytes[0];
            bytes[1] = nonce_bytes[1];
            bytes[2] = nonce_bytes[2];
            bytes[3] = nonce_bytes[3];

            let hash = Block::calculate_hash(&bytes);
            if Block::prefix(&hash) == 0 {
                self.hash = hash;
                break;
            }

            self.nonce += 1;
        }
    }

    pub fn is_valid(&self) -> bool {
        let bytes = self.to_bytes();
        let hash = Block::calculate_hash(&bytes);
        self.hash == hash && Block::prefix(&hash) == 0
    }

    pub fn get_previous_hash(&self) -> Hash {
        self.previous_hash
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }

    pub fn get_hash(&self) -> Hash {
        self.hash
    }
}

pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {

    pub fn new() -> Self {
        Blockchain {
            chain: vec![]
        }
    }

    pub fn add(&mut self, data: Vec<u8>) -> Option<&Block> {
        let block: Block;

        if self.chain.len() == 0 {
            block = Block::genesis(data);
        }
        else {
            block = self.chain.last().unwrap().chain(data);
        }

        self.chain.push(block);
        self.chain.last()
    }

    pub fn iter(&self) -> std::slice::Iter<Block> {
        self.chain.iter()
    }
}
