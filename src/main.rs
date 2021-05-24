use block::Blockchain;

mod block;

fn main() {
    let mut chain = Blockchain::new();
    chain.add(vec![0]);
    chain.add(vec![0]);
    chain.add(vec![0]);
    chain.add(vec![0]);
    chain.add(vec![0]);
    chain.add(vec![0]);
    chain.add(vec![0]);
    chain.add(vec![0]);

    for block in chain.iter() {
        println!("{:02x?} {}", block.get_hash(), block.get_nonce());
    }
}
