use solana_instruction::AccountMeta;
use solana_pubkey::Pubkey;

pub fn metas_from_keys_signer_writer<const N: usize>(
    keys: [[u8; 32]; N],
    is_signer: [bool; N],
    is_writer: [bool; N],
) -> Vec<AccountMeta> {
    keys.into_iter()
        .zip(is_signer)
        .zip(is_writer)
        .map(|((pubkey, is_signer), is_writable)| AccountMeta {
            pubkey: Pubkey::new_from_array(pubkey),
            is_signer,
            is_writable,
        })
        .collect()
}
