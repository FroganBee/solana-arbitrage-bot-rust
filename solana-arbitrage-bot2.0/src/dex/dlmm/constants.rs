use solana_program::pubkey::Pubkey;
use std::str::FromStr;

pub fn dlmm_program_id() -> Pubkey {
    Pubkey::from_str("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo").unwrap()
}

pub fn dlmm_event_authority() -> Pubkey {
    Pubkey::from_str("D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6").unwrap()
}

pub const BIN_ARRAY: &[u8] = b"bin_array";
