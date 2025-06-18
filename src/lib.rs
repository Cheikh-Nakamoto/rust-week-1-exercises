// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Decode the hex string to bytes
    let bytes = hex::decode(raw_tx_hex).map_err(|_| "Hex decode error".to_string())?;

    // Check if we have at least 4 bytes for the version
    if bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    // Extract the first 4 bytes and convert to u32 (little-endian)
    // In Bitcoin transactions, the version is the first 4 bytes in little-endian format
    let version_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
    let version = u32::from_le_bytes(version_bytes);

    Ok(version)
}
