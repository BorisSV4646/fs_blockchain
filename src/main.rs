use env_logger::Builder;
use fs_blockchain::{
    blockchain::{miner::Miner, transaction::Transaction},
    consensus::dpos::{DPoS, Delegate},
    core::errors::Result,
};
use log::LevelFilter;

fn main() -> Result<()> {
    Builder::new().filter(None, LevelFilter::Info).init();
    log::info!("Запуск FS Spin блокчейн");

    let delegate1 = Delegate::new(1, "Alice");
    let delegate2 = Delegate::new(2, "Bob");
    let delegate3 = Delegate::new(3, "Charlie");

    let mut dpos = DPoS::new(vec![delegate1, delegate2, delegate3]);

    let votes = vec![(1, 100), (2, 200), (3, 120)];
    for (delegate_id, vote_count) in votes {
        if let Err(e) = dpos.vote(delegate_id, vote_count) {
            eprintln!("Ошибка голосования для делегата {}: {}", delegate_id, e);
        }
    }

    let mut miner = Miner::new();

    miner.add_transaction(Transaction::new(
        "0x324324234".to_string(),
        "0x3asdasd".to_string(),
        10.0,
    ));
    miner.add_transaction(Transaction::new(
        "0x3asdasd".to_string(),
        "0x324324234".to_string(),
        5.0,
    ));

    miner.produce_block(&mut dpos)?;
    Ok(())
}
