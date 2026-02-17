use blockchain_core::config::NodeConfig;

pub struct NodeContext {
    pub config: NodeConfig,
}

impl NodeContext {
    pub fn build() -> Result<Self, String> {
        todo!("node context builder is not added yet")
    }
}
