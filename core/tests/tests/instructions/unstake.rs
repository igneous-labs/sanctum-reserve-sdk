use const_crypto::bs58;
use mollusk_svm::{
    program::{create_keyed_account_for_builtin_program, keyed_account_for_system_program},
    result::InstructionResult,
};
use sanctum_reserve_core::{
    self as reserve_core, stake_account_record_seeds, UnstakeIxData, UnstakeIxPrefixKeysOwned, FEE,
    POOL, POOL_SOL_RESERVES, PROTOCOL_FEE, PROTOCOL_FEE_VAULT, STAKE_PROGRAM, SYSTEM_PROGRAM,
    SYSVAR_CLOCK, UNSTAKE_IX_IS_SIGNER, UNSTAKE_IX_IS_WRITER, UNSTAKE_PROGRAM,
};
use solana_account::Account;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

use crate::common::{
    metas_from_keys_signer_writer, mollusk_unstake_prog, payer_account, unstake_mainnet_accounts,
};

#[test]
fn unstake_keys() {
    let unstaker = Pubkey::from_str_const("pay1VHNPtXwQkSypEfranUVn2ToxmqqNkbYoyeHVQXj");
    let stake_account = Pubkey::from_str_const("1111111ogCyDbaRMvkdsHB3qfdyFYaG1WtRUAfdh");

    let stake_account_addr = stake_account.to_bytes();

    let stake_account_record_seeds = stake_account_record_seeds(&POOL, &stake_account_addr);

    let stake_account_record_pubkey = Pubkey::find_program_address(
        &[
            stake_account_record_seeds.0.as_ref(),
            stake_account_record_seeds.1.as_ref(),
        ],
        &Pubkey::from(UNSTAKE_PROGRAM),
    )
    .0;

    let keys = UnstakeIxPrefixKeysOwned::default()
        .with_mainnet_const_pdas()
        .with_consts()
        .with_unstaker(unstaker.to_bytes())
        .with_destination(unstaker.to_bytes())
        .with_stake(stake_account.to_bytes())
        .with_stake_account_record(stake_account_record_pubkey.to_bytes());

    assert_eq!(keys.0[0], unstaker.to_bytes());
    assert_eq!(keys.0[1], stake_account.to_bytes());
    assert_eq!(keys.0[2], unstaker.to_bytes());
    assert_eq!(keys.0[3], POOL);
    assert_eq!(keys.0[4], POOL_SOL_RESERVES);
    assert_eq!(keys.0[5], FEE);
    assert_eq!(keys.0[6], stake_account_record_pubkey.to_bytes());
    assert_eq!(keys.0[7], PROTOCOL_FEE);
    assert_eq!(keys.0[8], PROTOCOL_FEE_VAULT);
    assert_eq!(keys.0[9], SYSVAR_CLOCK);
    assert_eq!(keys.0[10], STAKE_PROGRAM);
    assert_eq!(keys.0[11], SYSTEM_PROGRAM);
}

#[test]
fn unstake_fixture() {
    let mollusk = mollusk_unstake_prog();

    let user = Pubkey::new_unique();
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

    let keys = UnstakeIxPrefixKeysOwned::default()
        .with_mainnet_const_pdas()
        .with_consts()
        .with_unstaker(user.to_bytes())
        .with_destination(user.to_bytes())
        .with_stake(stake_account_addr)
        .with_stake_account_record(stake_account_record_pubkey.to_bytes());

    let metas =
        metas_from_keys_signer_writer(keys.0, UNSTAKE_IX_IS_SIGNER.0, UNSTAKE_IX_IS_WRITER.0);

    let data = UnstakeIxData::new();

    let ix = Instruction {
        program_id: Pubkey::new_from_array(UNSTAKE_PROGRAM),
        accounts: metas,
        data: data.to_buf().into(),
    };

    let accounts = unstake_mainnet_accounts()
        .chain([
            keyed_account_for_system_program(),
            create_keyed_account_for_builtin_program(
                &Pubkey::new_from_array(STAKE_PROGRAM),
                "solana_stake_program",
            ),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (user, payer_account(1_000_000_000)),
            (stake_account_record_pubkey, Account::default()),
        ])
        .collect::<Vec<_>>();

    let InstructionResult {
        raw_result,
        resulting_accounts,
        ..
    } = mollusk.process_instruction_chain(&[ix], &accounts);

    assert!(raw_result.is_ok());

    let user_res = resulting_accounts.iter().find(|a| a.0 == user).unwrap();

    let stake_acc_rec_res = reserve_core::StakeAccountRecord::borsh_de(
        &resulting_accounts
            .iter()
            .find(|a| a.0 == stake_account_record_pubkey)
            .expect("Stake account record should exist")
            .1
            .data
            .as_slice()[8..],
    )
    .expect("Stake account record invalid data");

    // 302977251897 -> previous lamports from fixtures
    let fees_earned = resulting_accounts
        .iter()
        .find(|a| a.0.to_bytes() == PROTOCOL_FEE_VAULT)
        .expect("Protocol fee vault should exist")
        .1
        .lamports
        - 302977251897;

    // 409374014407718 -> previous lamports from fixtures
    let pool_sol_reserves_delta = 409374014407718
        - resulting_accounts
            .iter()
            .find(|a| a.0.to_bytes() == POOL_SOL_RESERVES)
            .expect("Pool sol reserves should exist")
            .1
            .lamports;

    // 1000000000 -> previous lamports from fixtures
    let user_delta = user_res.1.lamports - 1_000_000_000;

    // 1002240 is the amount of SOL pool sol reserves paid for rent exemption of the record
    assert_eq!(user_delta + fees_earned + 1002240, pool_sol_reserves_delta);
    assert_eq!(stake_acc_rec_res.lamports_at_creation, 1002282880);
}
