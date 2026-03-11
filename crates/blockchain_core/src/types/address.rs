use crate::error::TypeError;

#[derive(Debug, Clone)]
pub struct Address(String);

impl Address {
    pub fn new(input: &str) -> Result<Self, TypeError> {
        if input.trim().is_empty() {
            return Err(TypeError::EmptyAddress);
        }

        Ok(Self(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
