use crate::error::TypeError;

fn strip_0x_prefix(input: &str) -> &str {
    input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
        .unwrap_or(input)
}

pub(crate) fn decode_fixed_hex<const N: usize>(
    input: &str,
    empty_err: TypeError,
    invalid_length_err: TypeError,
    invalid_hex_err: TypeError,
) -> Result<[u8; N], TypeError> {
    let hex = strip_0x_prefix(input.trim());
    if hex.is_empty() {
        return Err(empty_err);
    }

    if hex.len() != N * 2 {
        return Err(invalid_length_err);
    }

    let bytes = hex::decode(hex).map_err(|_| invalid_hex_err)?;
    let mut raw = [0_u8; N];
    raw.copy_from_slice(&bytes);
    Ok(raw)
}
