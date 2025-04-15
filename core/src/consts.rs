use const_crypto::bs58;

use crate::{fee_seeds, flash_account_seeds, flash_loan_fee_seeds, protocol_fee_seeds};

pub const STAKE_ACCOUNT_RECORD_RENT: u64 = 1002240;

// Programs

pub const UNSTAKE_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ");

pub const SYSTEM_PROGRAM: [u8; 32] = bs58::decode_pubkey("11111111111111111111111111111111");

pub const TOKEN_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub const SYSVAR_CLOCK: [u8; 32] =
    bs58::decode_pubkey("SysvarC1ock11111111111111111111111111111111");

pub const STAKE_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("Stake11111111111111111111111111111111111111");

// Program Accounts

pub const POOL: [u8; 32] = bs58::decode_pubkey("FypPtwbY3FUfzJUtXHSyVRokVKG2jKtH29FmK4ebxRSd");
pub const PROTOCOL_FEE_VAULT: [u8; 32] =
    bs58::decode_pubkey("EeQmNqm1RcQnee8LTyx6ccVG9FnR8TezQuw2JXq2LC1T");
pub const POOL_SOL_RESERVES: [u8; 32] =
    bs58::decode_pubkey("3rBnnH9TTgd3xwu48rnzGsaQkSr1hR64nY71DrDt6VrQ");

const FEE_PDA: ([u8; 32], u8) = {
    let (s1, s2) = fee_seeds(&POOL);
    const_crypto::ed25519::derive_program_address(&[s1, s2], &UNSTAKE_PROGRAM)
};
pub const FEE: [u8; 32] = FEE_PDA.0;
pub const FEE_BUMP: u8 = FEE_PDA.1;

const PROTOCOL_FEE_PDA: ([u8; 32], u8) = {
    let seed = protocol_fee_seeds();
    const_crypto::ed25519::derive_program_address(&[seed], &UNSTAKE_PROGRAM)
};
pub const PROTOCOL_FEE: [u8; 32] = PROTOCOL_FEE_PDA.0;
pub const PROTOCOL_FEE_BUMP: u8 = PROTOCOL_FEE_PDA.1;

const FLASH_ACCOUNT_PDA: ([u8; 32], u8) = {
    let (s1, s2) = flash_account_seeds(&POOL);
    const_crypto::ed25519::derive_program_address(&[s1, s2], &UNSTAKE_PROGRAM)
};
pub const FLASH_ACCOUNT: [u8; 32] = FLASH_ACCOUNT_PDA.0;
pub const FLASH_ACCOUNT_BUMP: u8 = FLASH_ACCOUNT_PDA.1;

const FLASH_LOAN_FEE_PDA: ([u8; 32], u8) = {
    let (s1, s2) = flash_loan_fee_seeds(&POOL);
    const_crypto::ed25519::derive_program_address(&[s1, s2], &UNSTAKE_PROGRAM)
};
pub const FLASH_LOAN_FEE: [u8; 32] = FLASH_LOAN_FEE_PDA.0;
pub const FLASH_LOAN_FEE_BUMP: u8 = FLASH_LOAN_FEE_PDA.1;
