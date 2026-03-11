#[derive(Debug, Clone)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn new(input: u64) -> Self {
        Self(input)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}
