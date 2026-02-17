#[derive(Debug, Clone)]
pub enum NetworkMessage {
    Ping,
    Pong,
    NewTransaction,
    NewBlock,
    GetHeight,
    Height(u64),
    GetBlocks { from: u64 },
    Blocks,
}
