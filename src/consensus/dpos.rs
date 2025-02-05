use crate::core::errors::{AppError, Result};

#[derive(Debug, Clone)]
pub struct Delegate {
    pub id: u64,
    pub name: String,
    pub votes: u64,
}

impl Delegate {
    pub fn new(id: u64, name: &str) -> Self {
        Delegate {
            id,
            name: name.to_string(),
            votes: 0,
        }
    }
}

pub struct DPoS {
    pub delegates: Vec<Delegate>,
}

impl DPoS {
    pub fn new(delegates: Vec<Delegate>) -> Self {
        DPoS { delegates }
    }

    pub fn vote(&mut self, delegate_id: u64, votes: u64) -> Result<()> {
        if let Some(delegate) = self.delegates.iter_mut().find(|d| d.id == delegate_id) {
            delegate.votes += votes;
            Ok(())
        } else {
            Err(AppError::DelegateNotFound(42))
        }
    }

    pub fn select_delegate(&mut self) -> Option<&Delegate> {
        self.delegates.iter().max_by_key(|delegate| delegate.votes)
    }
}
