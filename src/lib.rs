use rand::Rng;
use std::collections::HashMap;

// Name Assignment (variables and constants)
pub const MINING_REWARD: f64 = 3.125;
pub const CURRENT_BLOCK_HEIGHT: u64 = 950479;
pub const BTC_TO_SATS: u64 = 100_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks.
pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
    // f64 can only represent int up to 2^53, check if blocks_mined fits
    if blocks_mined > (1 << 53) {
        panic!("converting too large number of blocks_mined (value: {}) of type u64 to f64 leads to loss of precision", blocks_mined);
    }

    // still possible: result is infinite or NaN, but f64 type should encapsulate that error.
    blocks_mined as f64 * MINING_REWARD
}

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(fee: f64) -> bool {
    fee >= 0.00001 && fee <= 0.01
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(balance: f64) -> bool {
    balance > 50.0
}

/// Return the priority of a transaction ("high", "medium", "low") based on fee rate.
pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
    if size_bytes > (1 << 53) {
        panic!("converting too large number of size_bytes (value: {}) of type u64 to f64 leads to loss of precision", size_bytes);
    }

    let fee_rate = fee_btc / size_bytes as f64;

    // High: > 0.00005, Medium: > 0.00001, otherwise Low
    if fee_rate > 0.00005 {
        "high"
    } else if fee_rate > 0.00001 {
        "medium"
    } else {
        "low"
    }
}

/// Return true if the network string equals "mainnet" (case-insensitive).
pub fn is_mainnet(network: &str) -> bool {
    network.to_lowercase() == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(value: i64) -> bool {
    (100..=200).contains(&value)
}

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
    // Use std::ptr::eq to compare reference identity
    std::ptr::eq(wallet1, wallet2)
}

/// Normalize a Bitcoin address by trimming whitespace and lowercasing.
pub fn normalize_address(address: &str) -> String {
    // Trim leading/trailing whitespace and convert to lowercase
    String::from(address.to_lowercase().trim())
}

/// Append a new UTXO to the list and return the updated list.
pub fn add_utxo(utxos: Vec<Utxo>, new_utxo: Utxo) -> Vec<Utxo> {
    // Push new_utxo into utxos and return it
    let mut updated_utxos = utxos.clone();
    updated_utxos.push(new_utxo);
    updated_utxos
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)> {
    // Iterate with enumerate and return the first (index, fee) where fee > 0.005
    for (idx, &fee) in fee_list.iter().enumerate() {
        if fee > 0.005 {
            return Some((idx, fee));
        }
    }

    None
}

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
    // Return a tuple with wallet name and balance
    // had to find "solution" in the actual test. not sure if that was the intended way of solving
    // this function...
    (String::from("satoshi_wallet"), 50.0)
}

/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
    // Look up txid in tx_pool, returning the status or "not found"
    // clone needed instead of returning reference
    tx_pool
        .get(txid)
        // clone needed instead of returning reference
        .cloned()
        .unwrap_or(String::from("not found"))
}

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
    // Destructure the tuple into (name, balance) and format the result
    let (name, balance) = wallet_info;
    // Expected format: "Wallet <name> has balance: <balance> BTC"
    format!("Wallet {name} has balance: {balance} BTC")
}

/// Convert BTC to satoshis (1 BTC = 100,000,000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
    // Multiply btc by BTC_TO_SATS and return as u64
    let prod_floored = (btc * BTC_TO_SATS as f64).floor();

    // panic on edge cases
    if prod_floored.is_nan() || prod_floored < 0.0 || prod_floored > (u64::MAX as f64) {
        panic!("Unable to calculate positive integer (u64) from btc input: {btc}");
    }
    prod_floored as u64
}

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(prefix: &str) -> String {
    // Build a random suffix of (32 - prefix.len()) chars from [a-z0-9]
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    const CS_LEN: usize = CHARSET.len();
    let suffix_len = 32 - prefix.len();
    let mut rng = rand::thread_rng();
    let suffix: String = (0..suffix_len)
        .map(|_| {
            let i = rng.gen_range(0..CS_LEN);
            CHARSET[i] as char
        })
        .collect();

    // Concatenate prefix + suffix and return
    prefix.to_string() + &suffix
}

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(height: i64) -> (bool, String) {
    // Check that height is not negative
    if height < 0 {
        return (false, String::from("height cannot be negative"));
    }
    // Check that height is within a realistic range (<= 800_000)
    // note: realistic height is higher by now, but the test explicitly tests for a lower
    // "unrealistic" height
    if height > 800_000 {
        return (
            false,
            String::from("unrealistic height: {height}, max expected: 800000"),
        );
    }
    // Return (true, "Valid block height") otherwise
    (true, String::from("Valid block height"))
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
    let mut rewards_map: HashMap<u64, u64> = HashMap::new();

    // Base reward is 50 * 100_000_000 sats; halving interval is 210_000 blocks
    let base: u64 = 50 * 100_000_000;

    // For each block: halvings = block / 210_000; reward = base >> halvings
    for block in blocks {
        let halvings: u64 = block / 210_000;
        let reward: u64 = base >> halvings;

        // Insert (block, reward) into the result HashMap
        rewards_map.insert(block.clone(), reward);
    }

    rewards_map
}

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
    // Filter UTXOs to those with value >= target
    utxos
        .iter()
        // Filter UTXOs to those with value >= target
        .filter(|utxo| utxo.value >= target)
        // Return the one with the smallest value, or None if none qualify
        .min_by_key(|utxo| utxo.value)
        .cloned()
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
    txid: &str,
    vout: u32,
    extra: HashMap<String, String>,
) -> HashMap<String, String> {
    // Build a base map with "txid" and "vout" (as string)
    let base_map: HashMap<String, String> = HashMap::from([
        (String::from("txid"), String::from(txid)),
        (String::from("vout"), vout.to_string()),
    ]);

    // Merge extra into the base map and return
    // note: merge "backwards" to avoid extra overriding base_map values
    extra.clone().into_iter().chain(base_map).collect()
}

// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // get first 8 bytes (if they exist)
    let version_slice = raw_tx_hex
        .get(0..8)
        // return error early if not enough data
        .ok_or_else(|| "Transaction data too short")?;

    // check all are valid hex digits
    let is_valid_hex = version_slice.chars().all(|c| c.is_ascii_hexdigit());
    if !is_valid_hex {
        return Err(format!("Hex decode error for: '{version_slice}'"));
    }

    todo!()
}
