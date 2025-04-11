use mollusk_svm::Mollusk;
use sanctum_reserve_core::{self as reserve_core};
use solana_pubkey::Pubkey;

use super::{test_fixtures_dir, BPF_LOADER_UPGRADEABLE_PROGRAM_ID};

pub fn mollusk_unstake_prog() -> Mollusk {
    let mut res = Mollusk::default();
    res.add_program_with_elf_and_loader(
        &Pubkey::new_from_array(reserve_core::UNSTAKE_PROGRAM),
        &std::fs::read(test_fixtures_dir().join("unstake.so")).unwrap(),
        &Pubkey::new_from_array(BPF_LOADER_UPGRADEABLE_PROGRAM_ID),
    );

    mollusk_svm_programs_token::token::add_program(&mut res);
    res
}
