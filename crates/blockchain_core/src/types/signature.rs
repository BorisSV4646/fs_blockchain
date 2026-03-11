#[derive(Debug, Clone)]
pub struct Signature(String);

impl Signature {
    pub fn new(input: &str) -> Result<Self, String> {
        if input.trim().is_empty() {
            return Err("signature cannot be empty".to_string());
        }

        Ok(Self(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
