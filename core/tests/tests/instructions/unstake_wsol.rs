use std::str::FromStr;

use const_crypto::bs58;
use mollusk_svm::{
    program::{create_keyed_account_for_builtin_program, keyed_account_for_system_program},
    result::InstructionResult,
};
use sanctum_reserve_core::{
    self as reserve_core, quote_unstake, stake_account_record_seeds, PoolUnstakeParams,
    QuoteUnstakeOpts, UnstakeWsolIxData, UnstakeWsolIxPrefixKeysOwned, POOL,
    STAKE_ACCOUNT_RECORD_RENT, STAKE_PROGRAM, UNSTAKE_PROGRAM, UNSTAKE_WSOL_IX_IS_SIGNER,
    UNSTAKE_WSOL_IX_IS_WRITER,
};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

use crate::common::{
    metas_from_keys_signer_writer, mollusk_unstake_prog, payer_account, unstake_mainnet_accounts,
};

#[test]
fn unstake_wsol_fixture() {
    let mollusk = mollusk_unstake_prog();

    let account_fixtures = unstake_mainnet_accounts();

    let protocol_fee_vault_bef = account_fixtures.protocol_fee_vault().clone();
    let pool_sol_reserves_bef = account_fixtures.pool_sol_reserves().clone();
    let stake_account_bef = account_fixtures.stake_account().1.lamports;

    let user: Pubkey = Pubkey::from_str("1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM").unwrap();
    let (dest, dest_bef) = account_fixtures.user_wsol_token().clone();
    let referrer = Pubkey::from_str("Gu7aUxceG5zETeSPRjkzYb9nfBGwwXbdkJaY7BK8xpqr").unwrap();
    let stake_account_addr = bs58::decode_pubkey("1111111ogCyDbaRMvkdsHB3qfdyFYaG1WtRUAfdh");

    let stake_account_record_seeds = stake_account_record_seeds(&POOL, &stake_account_addr);

    let stake_account_record_pubkey = Pubkey::find_program_address(
        &[
            stake_account_record_seeds.0.as_ref(),
            stake_account_record_seeds.1.as_ref(),
        ],
        &Pubkey::from(UNSTAKE_PROGRAM),
    )
    .0;

    let pool_account = &account_fixtures.pool().1;
    let pool = reserve_core::Pool::anchor_de(pool_account.data.as_slice()).unwrap();

    let fee_account = &account_fixtures.fee().1;
    let reserve_core::Fee(fee) = reserve_core::Fee::anchor_de(fee_account.data.as_slice()).unwrap();

    let protocol_fee_account = &account_fixtures.protocol_fee().1;
    let protocol_fee =
        reserve_core::ProtocolFee::anchor_de(protocol_fee_account.data.as_slice()).unwrap();

    let quote = quote_unstake(
        &PoolUnstakeParams {
            pool_incoming_stake: pool.incoming_stake,
            sol_reserves_lamports: account_fixtures.pool_sol_reserves().1.lamports,
        },
        &fee,
        &protocol_fee.fee_ratios(),
        account_fixtures.stake_account().1.lamports,
        &QuoteUnstakeOpts {
            with_referrer: true,
            ..Default::default()
        },
    )
    .expect("Quote should be valid");

    let keys = UnstakeWsolIxPrefixKeysOwned::default()
        .with_mainnet_const_pdas()
        .with_consts()
        .with_unstaker(user.to_bytes())
        .with_destination(dest.to_bytes())
        .with_stake(stake_account_addr)
        .with_stake_account_record(stake_account_record_pubkey.to_bytes());

    let metas = metas_from_keys_signer_writer(
        keys.0,
        UNSTAKE_WSOL_IX_IS_SIGNER.0,
        UNSTAKE_WSOL_IX_IS_WRITER.0,
    );

    let data = UnstakeWsolIxData::new();

    let ix = Instruction {
        program_id: Pubkey::new_from_array(UNSTAKE_PROGRAM),
        accounts: metas
            .into_iter()
            .chain([AccountMeta::new(referrer, false)])
            .collect::<Vec<_>>(),
        data: data.to_buf().into(),
    };

    let accounts = account_fixtures
        .0
        .into_iter()
        .chain([
            keyed_account_for_system_program(),
            create_keyed_account_for_builtin_program(
                &Pubkey::new_from_array(STAKE_PROGRAM),
                "solana_stake_program",
            ),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (user, payer_account(1_000_000_000)),
            (stake_account_record_pubkey, Account::default()),
            (referrer, Account::default()),
            mollusk_svm_programs_token::token::keyed_account(),
        ])
        .collect::<Vec<_>>();

    let InstructionResult {
        raw_result,
        resulting_accounts,
        ..
    } = mollusk.process_instruction_chain(&[ix], &accounts);

    assert!(raw_result.is_ok());

    let dest_res = resulting_accounts.iter().find(|a| a.0 == dest).unwrap();

    let referrer_res = resulting_accounts.iter().find(|a| a.0 == referrer).unwrap();

    let stake_acc_rec_res = reserve_core::StakeAccountRecord::anchor_de(
        resulting_accounts
            .iter()
            .find(|a| a.0 == stake_account_record_pubkey)
            .expect("Stake account record should exist")
            .1
            .data
            .as_slice(),
    )
    .expect("Stake account record invalid data");

    let protocol_fees_earned = resulting_accounts
        .iter()
        .find(|a| a.0 == protocol_fee_vault_bef.0)
        .expect("Protocol fee vault should exist")
        .1
        .lamports
        - protocol_fee_vault_bef.1.lamports;

    let pool_sol_reserves_delta = pool_sol_reserves_bef.1.lamports
        - resulting_accounts
            .iter()
            .find(|a| a.0 == pool_sol_reserves_bef.0)
            .expect("Pool sol reserves should exist")
            .1
            .lamports;

    let dest_delta = dest_res.1.lamports - dest_bef.lamports;

    assert_eq!(
        quote.fee.referrer + dest_delta + protocol_fees_earned + STAKE_ACCOUNT_RECORD_RENT,
        pool_sol_reserves_delta
    );
    assert_eq!(stake_acc_rec_res.lamports_at_creation, stake_account_bef);
    assert_eq!(quote.stake_account_lamports, stake_account_bef);
    assert_eq!(quote.lamports_to_unstaker, dest_delta);
    assert_eq!(quote.fee.protocol, protocol_fees_earned);
    assert_eq!(quote.fee.referrer, referrer_res.1.lamports);
    assert_eq!(quote.reserves_lamports_outflow(), pool_sol_reserves_delta);
}
