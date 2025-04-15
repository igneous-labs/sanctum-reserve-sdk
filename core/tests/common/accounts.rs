use std::{fs::File, path::Path};

use sanctum_reserve_core::SYSTEM_PROGRAM;
use serde::{Deserialize, Serialize};
use solana_account::Account;
use solana_account_decoder_client_types::UiAccount;
use solana_pubkey::Pubkey;

use super::{test_fixtures_dir, UnstakeMainnet, UnstakeMainnetKeyedAccounts};

/// This is the json format of
/// `solana account -o <FILENAME>.json --output json <ACCOUNT-PUBKEY>`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyedUiAccount {
    pub pubkey: String,
    pub account: UiAccount,
}

impl KeyedUiAccount {
    fn from_file<P: AsRef<Path>>(json_file_path: P) -> Self {
        let mut file = File::open(json_file_path).unwrap();
        serde_json::from_reader(&mut file).unwrap()
    }

    /// Loads a KeyedUiAccount from `<test_fixtures_dir()>/relative_json_file_path.json`
    pub fn from_test_fixtures_file<P: AsRef<Path>>(relative_json_file_path: P) -> Self {
        Self::from_file(
            test_fixtures_dir()
                .join(relative_json_file_path)
                .with_extension("json"),
        )
    }

    /// Assumes data is not `UiAccountData::Json`
    pub fn account_data(&self) -> Vec<u8> {
        self.account.data.decode().unwrap()
    }
}

pub fn payer_account(lamports: u64) -> Account {
    Account::new(lamports, 0, &Pubkey::new_from_array(SYSTEM_PROGRAM))
}

fn load_fixture_account(fname: &str) -> (Pubkey, Account) {
    let KeyedUiAccount { pubkey, account } = KeyedUiAccount::from_test_fixtures_file(fname);
    (pubkey.parse().unwrap(), account.decode().unwrap())
}

pub fn unstake_mainnet_accounts() -> UnstakeMainnetKeyedAccounts {
    UnstakeMainnet([
        load_fixture_account("fee"),
        load_fixture_account("pool"),
        load_fixture_account("pool-sol-reserves"),
        load_fixture_account("protocol-fee"),
        load_fixture_account("protocol-fee-vault"),
        load_fixture_account("stake-account"),
        load_fixture_account("user-wsol-token"),
    ])
}
