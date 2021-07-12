use anyhow::anyhow;
use ethabi::Contract;
use std::fs;
use std::io;
use std::str::FromStr;

fn read_file_to_json_value(path: &str) -> io::Result<serde_json::Value> {
    let path = std::path::Path::new(path);
    let contents = fs::read_to_string(path)?;
    let val = serde_json::Value::from_str(&contents)?;
    Ok(val)
}

// TODO: better error handling
pub fn get_abi(path: &str) -> Result<Contract, anyhow::Error> {
    let abi_string = read_file_to_json_value(path)
        .expect("couldn't read CONTRACT_FILE")
        .get("abi")
        .expect("couldn't get abi from CONTRACT_FILE")
        .to_string();

    Contract::load(abi_string.as_bytes()).map_err(|_| anyhow!("load contract"))
}