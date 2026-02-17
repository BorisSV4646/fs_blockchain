use crate::network::messages::NetworkMessage;

pub fn encode(_message: &NetworkMessage) -> Result<Vec<u8>, String> {
    todo!("protocol encoding is not added yet")
}

pub fn decode(_bytes: &[u8]) -> Result<NetworkMessage, String> {
    todo!("protocol decoding is not added yet")
}
