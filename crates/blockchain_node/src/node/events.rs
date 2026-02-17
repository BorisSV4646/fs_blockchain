#[derive(Debug, Clone)]
pub enum NodeEvent {
    Tick,
    NewTransaction,
    NewBlock,
    Shutdown,
}
