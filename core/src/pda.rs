pub const FEE_SEED: [u8; 3] = *b"fee";
pub const FLASH_ACCOUNT_SEED: [u8; 12] = *b"flashaccount";
pub const FLASH_LOAN_FEE_SEED: [u8; 12] = *b"flashloanfee";
pub const PROTOCOL_FEE_SEED: [u8; 12] = *b"protocol-fee";

#[inline]
pub const fn fee_seeds(pool: &[u8; 32]) -> (&[u8; 32], &[u8; 3]) {
    (pool, &FEE_SEED)
}

#[inline]
pub const fn flash_account_seeds(pool: &[u8; 32]) -> (&[u8; 32], &[u8; 12]) {
    (pool, &FLASH_ACCOUNT_SEED)
}

#[inline]
pub const fn flash_loan_fee_seeds(pool: &[u8; 32]) -> (&[u8; 32], &[u8; 12]) {
    (pool, &FLASH_LOAN_FEE_SEED)
}

#[inline]
pub const fn protocol_fee_seeds<'a>() -> &'a [u8; 12] {
    &PROTOCOL_FEE_SEED
}

#[inline]
pub const fn pool_sol_reserves_seeds(pool: &[u8; 32]) -> &[u8; 32] {
    pool
}

#[inline]
pub const fn stake_account_record_seeds<'a>(
    pool: &'a [u8; 32],
    stake_account: &'a [u8; 32],
) -> (&'a [u8; 32], &'a [u8; 32]) {
    (pool, stake_account)
}
