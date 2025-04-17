use generic_array_struct::generic_array_struct;

use crate::{
    FEE, POOL, POOL_SOL_RESERVES, PROTOCOL_FEE, PROTOCOL_FEE_VAULT, STAKE_PROGRAM, SYSTEM_PROGRAM,
    SYSVAR_CLOCK, TOKEN_PROGRAM,
};

pub const INSTRUCTION_DISCRIM_UNSTAKE_WSOL: [u8; 8] = [125, 93, 190, 135, 89, 174, 142, 149];

#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct UnstakeWsolIxPrefixAccs<T> {
    pub unstaker: T,
    pub stake: T,
    pub destination: T,
    pub pool: T,
    pub pool_sol_reserves: T,
    pub fee: T,
    pub stake_account_record: T,
    pub protocol_fee: T,
    pub protocol_fee_dest: T,
    pub sysvar_clock: T,
    pub stake_program: T,
    pub system_program: T,
    pub token_program: T,
}

pub type UnstakeWsolIxPrefixKeysOwned = UnstakeWsolIxPrefixAccs<[u8; 32]>;
pub type UnstakeWsolIxPrefixKeys<'a> = UnstakeWsolIxPrefixAccs<&'a [u8; 32]>;
pub type UnstakeWsolIxPrefixAccsFlag = UnstakeWsolIxPrefixAccs<bool>;

pub const UNSTAKE_WSOL_IX_IS_WRITER: UnstakeWsolIxPrefixAccsFlag =
    UnstakeWsolIxPrefixAccs([false; UNSTAKE_WSOL_IX_PREFIX_ACCS_LEN])
        .const_with_stake(true)
        .const_with_destination(true)
        .const_with_pool(true)
        .const_with_pool_sol_reserves(true)
        .const_with_stake_account_record(true)
        .const_with_protocol_fee_dest(true);

pub const UNSTAKE_WSOL_IX_IS_SIGNER: UnstakeWsolIxPrefixAccsFlag =
    UnstakeWsolIxPrefixAccs([false; UNSTAKE_WSOL_IX_PREFIX_ACCS_LEN]).const_with_unstaker(true);

impl<T: Clone> UnstakeWsolIxPrefixAccs<T> {
    #[inline]
    pub const fn new(arr: [T; UNSTAKE_WSOL_IX_PREFIX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl UnstakeWsolIxPrefixKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> UnstakeWsolIxPrefixKeys<'_> {
        UnstakeWsolIxPrefixKeys::new(self.0.each_ref())
    }

    #[inline]
    pub fn with_consts(self) -> Self {
        self.as_borrowed().with_consts().into_owned()
    }

    #[inline]
    pub fn with_mainnet_const_pdas(self) -> Self {
        self.as_borrowed().with_mainnet_const_pdas().into_owned()
    }
}

impl UnstakeWsolIxPrefixKeys<'_> {
    #[inline]
    pub fn into_owned(self) -> UnstakeWsolIxPrefixKeysOwned {
        UnstakeWsolIxPrefixKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_sysvar_clock(&SYSVAR_CLOCK)
            .const_with_stake_program(&STAKE_PROGRAM)
            .const_with_system_program(&SYSTEM_PROGRAM)
            .const_with_token_program(&TOKEN_PROGRAM)
    }

    #[inline]
    pub const fn with_mainnet_const_pdas(self) -> Self {
        self.const_with_pool(&POOL)
            .const_with_pool_sol_reserves(&POOL_SOL_RESERVES)
            .const_with_fee(&FEE)
            .const_with_protocol_fee(&PROTOCOL_FEE)
            .const_with_protocol_fee_dest(&PROTOCOL_FEE_VAULT)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]

pub struct UnstakeWsolIxData([u8; 8]);

impl UnstakeWsolIxData {
    #[inline]
    pub fn new() -> Self {
        Self(INSTRUCTION_DISCRIM_UNSTAKE_WSOL)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 8] {
        self.0
    }
}
