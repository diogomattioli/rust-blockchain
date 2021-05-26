use std::{convert::TryInto, marker::PhantomData};

use sha2::{Digest, Sha256};

static DIFFICULTY: u8 = 16;

type Hash = [u8; 32];

pub trait Consensus {
    fn to_bytes(block: &Block) -> Vec<u8>;
    fn calculate_hash(block: &mut Block);
    fn is_valid(block: &Block) -> bool;
}

pub struct ProofOfWork {
}

impl ProofOfWork {
    fn digest_sha(bytes: &Vec<u8>) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        hasher.finalize().try_into().unwrap()
    }

    fn prefix(hash: &Hash) -> u32 {
        u32::from_be_bytes(hash[0..4].try_into().unwrap()) >> (32 - DIFFICULTY)
    }
}

impl Consensus for ProofOfWork {
    fn to_bytes(block: &Block) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&block.nonce.to_be_bytes());
        bytes.extend_from_slice(&block.previous_hash[..]);
        bytes.extend_from_slice(&block.payload[..]);
        bytes
    }

    fn calculate_hash(block: &mut Block) {

        let mut bytes = Self::to_bytes(&block);

        loop {
            let nonce_bytes = block.nonce.to_be_bytes();
            bytes[0] = nonce_bytes[0];
            bytes[1] = nonce_bytes[1];
            bytes[2] = nonce_bytes[2];
            bytes[3] = nonce_bytes[3];

            let hash = Self::digest_sha(&bytes);
            if Self::prefix(&hash) == 0 {
                block.hash = hash;
                break;
            }

            block.nonce += 1;
        }
    }

    fn is_valid(block: &Block) -> bool {
        let bytes = Self::to_bytes(&block);
        let hash = Self::digest_sha(&bytes);
        block.hash == hash && Self::prefix(&hash) == 0
    }
}

#[derive(Debug)]
pub struct Block {
    nonce: u32,
    previous_hash: Hash,
    payload: Vec<u8>,
    hash: Hash,
}

impl Block {
    
    fn genesis<T: AsRef<[u8]>>(payload: T) -> Self {
        Block::new([0; 32], payload)
    }

    fn chain<T: AsRef<[u8]>>(&self, payload: T) -> Self {
        Block::new(self.hash, payload)
    }

    fn new<T: AsRef<[u8]>>(previous_hash: Hash, payload: T) -> Self {
        Block::raw(previous_hash, payload, 0, [0; 32])
    }

    pub fn raw<T: AsRef<[u8]>>(previous_hash: Hash, payload: T, nonce: u32, hash: Hash) -> Self {

        let mut v = Vec::new();
        v.extend_from_slice(payload.as_ref());

        Block {
            nonce,
            previous_hash,
            payload: v,
            hash,
        }
    }

    pub fn get_previous_hash(&self) -> Hash {
        self.previous_hash
    }

    pub fn get_payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    pub fn get_nonce(&self) -> u32 {
        self.nonce
    }

    pub fn get_hash(&self) -> Hash {
        self.hash
    }
}

pub struct Blockchain<C> {
    phantom: PhantomData<C>,
    chain: Vec<Block>,
}

impl<C: Consensus> Blockchain<C> {

    pub fn new() -> Self {
        Blockchain {
            phantom: PhantomData,
            chain: vec![],
        }
    }

    pub fn add<T: AsRef<[u8]>>(&mut self, payload: T) -> Option<&Block> {
        let mut block: Block;

        if self.chain.len() == 0 {
            block = Block::genesis(payload);
        }
        else {
            block = self.chain.last().unwrap().chain(payload);
        }
        C::calculate_hash(&mut block);

        self.chain.push(block);
        self.chain.last()
    }

    pub fn iter(&self) -> std::slice::Iter<Block> {
        self.chain.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::block::{Blockchain, ProofOfWork};

    #[test]
    fn blockchain_pow() {

        let mut chain = Blockchain::<ProofOfWork>::new();
        chain.add(vec![0]);
        chain.add("testing");
        chain.add([1,2,3]);
        chain.add(vec![0]);
        chain.add(vec![0]);

        let mut iter = chain.iter();

        let mut block = iter.next().unwrap();
        assert_eq!(block.get_nonce(), 66693);
        assert_eq!(block.get_hash(), [0x00, 0x00, 0xf6, 0xa4, 0x4e, 0x5a, 0x00, 0xf8, 0x67, 0x6d, 0x62, 0xcc, 0x0d, 0xdd, 0x66, 0xee, 0x57, 0x03, 0x94, 0xc4, 0x53, 0x8e, 0x2b, 0x1c, 0xf0, 0xb7, 0xbd, 0x36, 0x2c, 0x06, 0xab, 0x75]);

        block = iter.next().unwrap();
        assert_eq!(block.get_nonce(), 6392);
        assert_eq!(block.get_hash(), [0x00, 0x00, 0xde, 0x34, 0xf5, 0x9e, 0x84, 0xef, 0x95, 0x15, 0xa7, 0xe4, 0x08, 0xc1, 0x3f, 0x30, 0x5c, 0xed, 0x4d, 0xfd, 0xa4, 0x44, 0x22, 0xd6, 0x66, 0x86, 0x2c, 0x2b, 0x5d, 0xc2, 0x09, 0x82]);

        block = iter.next().unwrap();
        assert_eq!(block.get_nonce(), 67878);
        assert_eq!(block.get_hash(), [0x00, 0x00, 0xdf, 0x0a, 0x46, 0x85, 0x53, 0xf0, 0xd9, 0x6e, 0xf3, 0xda, 0x40, 0x08, 0x6b, 0xd9, 0x1b, 0xbc, 0xb8, 0xcd, 0x5b, 0x8a, 0xa3, 0xee, 0xb0, 0x4a, 0xb3, 0x19, 0xfb, 0xae, 0x24, 0x29]);

        block = iter.next().unwrap();
        assert_eq!(block.get_nonce(), 6064);
        assert_eq!(block.get_hash(), [0x00, 0x00, 0x69, 0xaf, 0x4e, 0xfa, 0xb7, 0xfb, 0x0a, 0x31, 0xf2, 0x2a, 0x5b, 0x46, 0xd7, 0xfb, 0x37, 0x3a, 0xe8, 0x08, 0xfc, 0x04, 0x6b, 0x24, 0x98, 0xd0, 0xf2, 0x05, 0x72, 0xa4, 0x8f, 0x99]);

        block = iter.next().unwrap();
        assert_eq!(block.get_nonce(), 80666);
        assert_eq!(block.get_hash(), [0x00, 0x00, 0x3d, 0x3c, 0x35, 0x07, 0xb3, 0x9f, 0xcd, 0x8b, 0xdb, 0xf5, 0x09, 0xd0, 0x40, 0x1c, 0x61, 0x8c, 0xe9, 0x8f, 0xf3, 0x1f, 0x60, 0xd6, 0xe0, 0x34, 0x35, 0xb2, 0xb7, 0x91, 0x6b, 0xee]);
    }
}