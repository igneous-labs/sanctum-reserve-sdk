#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PoolBalance {
    /// Read from [`crate::Pool::incoming_stake`]
    pub pool_incoming_stake: u64,

    /// Raw account lamport balance of the pool's SOL reserves PDA
    pub sol_reserves_lamports: u64,
}
