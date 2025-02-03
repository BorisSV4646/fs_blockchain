use fs_blockchain::blockchain::{miner::Miner, transaction::Transaction};

fn main() {
    let mut miner = Miner::new(2);

    miner.add_transaction(Transaction::new(
        "Alice".to_string(),
        "Bob".to_string(),
        10.0,
    ));
    miner.add_transaction(Transaction::new(
        "Bob".to_string(),
        "Charlie".to_string(),
        5.0,
    ));

    miner.mine_pending_transactions();

    for block in miner.blockchain.blocks.iter() {
        println!("Блок #{}: {}", block.index, block.hash);
    }
}
