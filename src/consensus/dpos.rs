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
    current_delegate_index: usize,
}

impl DPoS {
    pub fn new(delegates: Vec<Delegate>) -> Self {
        DPoS {
            delegates,
            current_delegate_index: 0,
        }
    }

    pub fn vote(&mut self, delegate_id: u64, votes: u64) -> Result<()> {
        if let Some(delegate) = self.delegates.iter_mut().find(|d| d.id == delegate_id) {
            delegate.votes += votes;
            Ok(())
        } else {
            Err(AppError::DelegateNotFound(delegate_id))
        }
    }

    pub fn select_delegate(&mut self) -> Result<&Delegate> {
        if self.delegates.is_empty() {
            return Err(AppError::NoDelegates);
        }

        let mut top_delegates: Vec<&Delegate> = self.delegates.iter().collect();
        top_delegates.sort_by_key(|delegate| std::cmp::Reverse(delegate.votes));

        let top = if top_delegates.len() > 10 {
            &top_delegates[..10]
        } else {
            &top_delegates[..]
        };

        let idx = self.current_delegate_index % top.len();
        self.current_delegate_index = self.current_delegate_index.wrapping_add(1);

        Ok(top[idx])
    }
}
