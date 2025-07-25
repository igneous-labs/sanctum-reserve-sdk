use crate::common::KeyedUiAccount;
use const_crypto::bs58;
use sanctum_reserve_core::{self as reserve_core, LiquidityLinearParams};

#[test]
fn test_pool_serde() {
    let pool_account = KeyedUiAccount::from_test_fixtures_file("pool");

    let pool = reserve_core::Pool::anchor_de(pool_account.account_data().as_slice()).unwrap();

    let fee_authority = bs58::encode_pubkey(&pool.fee_authority);
    let lp_mint = bs58::encode_pubkey(&pool.lp_mint);

    assert_eq!(
        fee_authority.str(),
        "3etKXcW2fzEJR5YXoSKSmP6UZ633g9uiFv5yuqFUf66k"
    );
    assert_eq!(lp_mint.str(), "uns3MbshJq1TmTyTF2iitMduevPktNKbePD73Wx4jQK");
    assert_eq!(pool.incoming_stake, 54148843273);
}

#[test]
fn test_protocol_fee_serde() {
    let protocol_fee_account = KeyedUiAccount::from_test_fixtures_file("protocol-fee");

    let protocol_fee =
        reserve_core::ProtocolFee::anchor_de(protocol_fee_account.account_data().as_slice())
            .unwrap();

    assert_eq!(
        protocol_fee.destination,
        bs58::decode_pubkey("EeQmNqm1RcQnee8LTyx6ccVG9FnR8TezQuw2JXq2LC1T")
    );
    assert_eq!(
        protocol_fee.authority,
        bs58::decode_pubkey("EeQmNqm1RcQnee8LTyx6ccVG9FnR8TezQuw2JXq2LC1T")
    );
    assert_eq!(
        protocol_fee.fee_ratio,
        reserve_core::Rational { num: 1, denom: 2 }
    );
    assert_eq!(
        protocol_fee.referrer_fee_ratio,
        reserve_core::Rational { num: 1, denom: 2 }
    );
}

#[test]
fn test_fee_serde() {
    let fee_account = KeyedUiAccount::from_test_fixtures_file("fee");

    let reserve_core::Fee(fee) =
        reserve_core::Fee::anchor_de(fee_account.account_data().as_slice()).unwrap();

    assert_eq!(
        fee,
        reserve_core::FeeEnum::LiquidityLinear(LiquidityLinearParams {
            max_liq_remaining: reserve_core::Rational {
                num: 1,
                denom: 1000
            },
            zero_liq_remaining: reserve_core::Rational { num: 8, denom: 100 },
        })
    );
}

#[test]
fn test_stake_account_record_serde() {
    let stake_account_record_account =
        KeyedUiAccount::from_test_fixtures_file("stake-account-record");

    let stake_account_record = reserve_core::StakeAccountRecord::anchor_de(
        stake_account_record_account.account_data().as_slice(),
    )
    .unwrap();

    assert_eq!(stake_account_record.lamports_at_creation, 4570380);
}
