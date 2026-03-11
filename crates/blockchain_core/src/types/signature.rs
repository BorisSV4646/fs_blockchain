use crate::error::TypeError;

#[derive(Debug, Clone)]
pub struct Signature(String);

impl Signature {
    pub fn new(input: &str) -> Result<Self, TypeError> {
        if input.trim().is_empty() {
            return Err(TypeError::EmptySignature);
        }

        Ok(Self(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
