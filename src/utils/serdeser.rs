use ic_cdk::api::stable::{StableWriter, StableReader};
use serde::{Deserialize, Serialize};

pub fn serialize<T: Serialize> (
    value: &T,
    version: f32,
    writer: &mut StableWriter
) -> Result<(), String> {
    let arr = rmp_serde::to_vec(value)
        .map_err(|err| err.to_string())?;
    // store version
    writer.write(&f32::to_le_bytes(version)).map_err(|e| e.to_string())?;
    // store size
    writer.write(&u64::to_le_bytes(arr.len() as u64)).map_err(|e| e.to_string())?;
    // store value
    writer.write(&arr).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn deserialize<T: Serialize + for<'a> Deserialize<'a>>(
    version: f32,
    reader: &mut StableReader
) -> Result<T, String> {
    // load version
    let mut version_buf = [0u8; 4];
    reader.read(&mut version_buf).map_err(|e| e.to_string())?;
    let stored_version = f32::from_le_bytes(version_buf);
    if stored_version != version {
        return Err("Invalid version".to_string());
    }
    // load size
    let mut size_buf = [0u8; 8];
    reader.read(&mut size_buf).map_err(|e| e.to_string())?;
    let size = u64::from_le_bytes(size_buf);
    // load value
    let mut table_buf = vec![0u8; size as usize];
    reader.read(&mut table_buf).map_err(|e| e.to_string())?;
    // decode value
    let res: T = rmp_serde::from_slice(&table_buf)
        .map_err(|err| err.to_string())?;
    Ok(res)
}