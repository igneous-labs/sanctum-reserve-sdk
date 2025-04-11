use const_crypto::bs58;

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

pub const FEE: [u8; 32] = bs58::decode_pubkey("5Pcu8WeQa3VbBz2vdBT49Rj4gbS4hsnfzuL1LmuRaKFY");

pub const PROTOCOL_FEE: [u8; 32] =
    bs58::decode_pubkey("2hN9UhvRFVfPYKL6rZJ5YiLEPCLTpN755pgwDJHWgFbU");

pub const PROTOCOL_FEE_VAULT: [u8; 32] =
    bs58::decode_pubkey("EeQmNqm1RcQnee8LTyx6ccVG9FnR8TezQuw2JXq2LC1T");

pub const POOL_SOL_RESERVES: [u8; 32] =
    bs58::decode_pubkey("3rBnnH9TTgd3xwu48rnzGsaQkSr1hR64nY71DrDt6VrQ");

pub const FLASH_ACCOUNT: [u8; 32] =
    bs58::decode_pubkey("BvCe729YxGXqgyVBW4r4R8S7ypkYm7KUyzVbTMai8kUc");

pub const FLASH_LOAN_FEE: [u8; 32] =
    bs58::decode_pubkey("6pf1eJA1C97znNC1m12qFGTWoq4KCQXJKrvzksFWmd2D");
