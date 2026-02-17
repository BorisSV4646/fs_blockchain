#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub difficulty: usize,
}

impl NodeConfig {
    pub fn new(difficulty: usize) -> Self {
        Self { difficulty }
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self { difficulty: 2 }
    }
}
