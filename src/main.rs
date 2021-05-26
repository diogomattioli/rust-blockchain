use block::Blockchain;
use block::ProofOfWork;

mod block;

fn main() {
    let mut chain = Blockchain::<ProofOfWork>::new();
    chain.add(vec![0]);
    chain.add("testing");
    chain.add([1,2,3]);
    chain.add(vec![0]);
    chain.add(vec![0]);

    for block in chain.iter() {
        println!("{:02x?} {} {:?}", block.get_hash(), block.get_nonce(), block.get_payload());
    }
}
