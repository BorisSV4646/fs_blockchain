#[derive(Debug, Clone)]
pub struct Address(String);

impl Address {
    pub fn new(input: &str) -> Result<Self, String> {
        if input.trim().is_empty() {
            return Err("address cannot be empty".to_string());
        }

        Ok(Self(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
