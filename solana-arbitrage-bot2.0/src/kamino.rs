use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::sysvar;
use std::str::FromStr;
use crate::constants::SOL_MINT;

pub const KAMINO_LENDING_PROGRAM_ID: &str = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD";

pub const KAMINO_LENDING_MARKET: &str = "H6rHXmXoCQvq8Ue81MqNh7ow5ysPa1dSozwW3PU1dDH6";
pub const KAMINO_LENDING_MARKET_AUTHORITY: &str = "Dx8iy2o46sK1DzWbEcznqSKeLbLVeu7otkibA3WohGAj";

pub const KAMINO_SOL_RESERVE: &str = "6gTJfuPHEg6uRAijRkMqNc9kan4sVZejKMxmvx2grT1p";
pub const KAMINO_SOL_RESERVE_LIQUIDITY: &str = "ywaaLvG7t1vXJo8sT3UzE8yzzZtxLM7Fmev64Jbooye";
pub const KAMINO_SOL_FEE_RECEIVER: &str = "EQ7hw63aBS7aPQqXsoxaaBxiwbEzaAiY9Js6tCekkqxf";

pub const KAMINO_REFERRER_TOKEN_STATE: &str = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD";
pub const KAMINO_REFERRER_ACCOUNT: &str = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD";

pub const KAMINO_ADDITIONAL_COMPUTE_UNITS: u32 = 80_000;

pub const KAMINO_FLASHLOAN_AMOUNT: u64 = 10_000_000_000_000;

pub struct FlashBorrowReserveLiquidity;

impl FlashBorrowReserveLiquidity {
    pub fn instruction_data(amount: u64) -> Vec<u8> {
        let mut data = vec![135, 231, 52, 167, 7, 52, 212, 193]; // Anchor discriminator for flashBorrowReserveLiquidity
        data.extend_from_slice(&amount.to_le_bytes());
        data
    }
}

pub struct FlashRepayReserveLiquidity;

impl FlashRepayReserveLiquidity {
    pub fn instruction_data(amount: u64, borrow_instruction_index: u8) -> Vec<u8> {
        let mut data = vec![185, 117, 0, 203, 96, 245, 180, 186]; // Anchor discriminator for flashRepayReserveLiquidity
        data.extend_from_slice(&amount.to_le_bytes());
        data.push(borrow_instruction_index);
        data
    }
}

pub fn get_kamino_flashloan_borrow_ix(
    wallet_pk: &Pubkey,
    destination_token_account: Pubkey,
) -> anyhow::Result<Instruction> {
    let kamino_program_id = Pubkey::from_str(KAMINO_LENDING_PROGRAM_ID)?;
    let lending_market = Pubkey::from_str(KAMINO_LENDING_MARKET)?;
    let lending_market_authority = Pubkey::from_str(KAMINO_LENDING_MARKET_AUTHORITY)?;
    let reserve = Pubkey::from_str(KAMINO_SOL_RESERVE)?;
    let reserve_liquidity_mint = Pubkey::from_str(SOL_MINT)?;
    let reserve_source_liquidity = Pubkey::from_str(KAMINO_SOL_RESERVE_LIQUIDITY)?;
    let fee_receiver = Pubkey::from_str(KAMINO_SOL_FEE_RECEIVER)?;
    let referrer_token_state = Pubkey::from_str(KAMINO_REFERRER_TOKEN_STATE)?;
    let referrer_account = Pubkey::from_str(KAMINO_REFERRER_ACCOUNT)?;

    let accounts = vec![
        AccountMeta::new(*wallet_pk, true), // userTransferAuthority
        AccountMeta::new_readonly(lending_market_authority, false), // lendingMarketAuthority
        AccountMeta::new_readonly(lending_market, false), // lendingMarket
        AccountMeta::new(reserve, false),   // reserve
        AccountMeta::new_readonly(reserve_liquidity_mint, false), // reserveLiquidityMint
        AccountMeta::new(reserve_source_liquidity, false), // reserveSourceLiquidity
        AccountMeta::new(destination_token_account, false), // userDestinationLiquidity
        AccountMeta::new(fee_receiver, false), // reserveLiquidityFeeReceiver
        AccountMeta::new_readonly(referrer_token_state, false), // referrerTokenState
        AccountMeta::new_readonly(referrer_account, false), // referrerAccount
        AccountMeta::new_readonly(sysvar::instructions::id(), false), // sysvarInfo
        AccountMeta::new_readonly(spl_token::id(), false), // tokenProgram
    ];

    Ok(Instruction {
        program_id: kamino_program_id,
        accounts,
        data: FlashBorrowReserveLiquidity::instruction_data(KAMINO_FLASHLOAN_AMOUNT),
    })
}

pub fn get_kamino_flashloan_repay_ix(
    wallet_pk: &Pubkey,
    source_token_account: Pubkey,
    borrow_instruction_index: u8,
) -> anyhow::Result<Instruction> {
    let kamino_program_id = Pubkey::from_str(KAMINO_LENDING_PROGRAM_ID)?;
    let lending_market = Pubkey::from_str(KAMINO_LENDING_MARKET)?;
    let lending_market_authority = Pubkey::from_str(KAMINO_LENDING_MARKET_AUTHORITY)?;
    let reserve = Pubkey::from_str(KAMINO_SOL_RESERVE)?;
    let reserve_liquidity_mint = Pubkey::from_str(SOL_MINT)?;
    let reserve_destination_liquidity = Pubkey::from_str(KAMINO_SOL_RESERVE_LIQUIDITY)?;
    let fee_receiver = Pubkey::from_str(KAMINO_SOL_FEE_RECEIVER)?;
    let referrer_token_state = Pubkey::from_str(KAMINO_REFERRER_TOKEN_STATE)?;
    let referrer_account = Pubkey::from_str(KAMINO_REFERRER_ACCOUNT)?;

    let accounts = vec![
        AccountMeta::new(*wallet_pk, true), // userTransferAuthority
        AccountMeta::new_readonly(lending_market_authority, false), // lendingMarketAuthority
        AccountMeta::new_readonly(lending_market, false), // lendingMarket
        AccountMeta::new(reserve, false),   // reserve
        AccountMeta::new_readonly(reserve_liquidity_mint, false), // reserveLiquidityMint
        AccountMeta::new(reserve_destination_liquidity, false), // reserveDestinationLiquidity
        AccountMeta::new(source_token_account, false), // userSourceLiquidity
        AccountMeta::new(fee_receiver, false), // reserveLiquidityFeeReceiver
        AccountMeta::new_readonly(referrer_token_state, false), // referrerTokenState
        AccountMeta::new_readonly(referrer_account, false), // referrerAccount
        AccountMeta::new_readonly(sysvar::instructions::id(), false), // sysvarInfo
        AccountMeta::new_readonly(spl_token::id(), false), // tokenProgram
    ];

    Ok(Instruction {
        program_id: kamino_program_id,
        accounts,
        data: FlashRepayReserveLiquidity::instruction_data(
            KAMINO_FLASHLOAN_AMOUNT,
            borrow_instruction_index,
        ),
    })
}
