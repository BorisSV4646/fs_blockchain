pub mod context;
pub mod events;
pub mod reconcile;
pub mod state;

use context::NodeContext;
use events::NodeEvent;
use state::NodeState;

pub struct Node {
    pub state: NodeState,
    pub context: NodeContext,
}

impl Node {
    pub fn new() -> Result<Self, String> {
        todo!("node initialization is not added yet")
    }

    pub fn handle_event(&mut self, event: NodeEvent) -> Result<(), String> {
        let _ = (self, event);
        todo!("event handling is not added yet")
    }

    pub fn tick(&mut self) -> Result<(), String> {
        let _ = self;
        todo!("node tick is not added yet")
    }

    pub fn run(&mut self) -> Result<(), String> {
        let _ = self;
        todo!("node loop is not added yet")
    }
}

pub fn run() -> Result<(), String> {
    let mut node = Node::new()?;
    node.run()
}
