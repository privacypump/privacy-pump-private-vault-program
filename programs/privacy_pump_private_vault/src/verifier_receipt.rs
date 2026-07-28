use anchor_lang::prelude::*;
use solana_sha256_hasher::hashv;

use crate::{
    errors::PrivacyPumpPrivateVaultError,
    state::{PrivacyPumpPrivateProposal, PrivacyPumpPrivateVault},
};

pub const PRIVACY_PUMP_ARCIUM_VERIFIER_PROGRAM_ID: Pubkey =
    pubkey!("GK8uG6xFTvmGxh6XqPsGqZwfTfpBWXpPdKv86AdvPT5L");

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivacyPumpAuthorizationStatus {
    Created,
    Pending,
    Verified,
    Failed,
    Consumed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PrivacyPumpArciumReceiptData {
    pub request: Pubkey,
    pub vault: Pubkey,
    pub proposal: Pubkey,
    pub signer_root: [u8; 32],
    pub signer_set_version: u32,
    pub action_domain: u8,
    pub nullifier_hash: [u8; 32],
    pub payload_hash: [u8; 32],
    pub arcium_computation_hash: [u8; 32],
    pub computation_offset: u64,
    pub verified_slot: u64,
    pub consumed: bool,
    pub consumed_by: Pubkey,
    pub status: PrivacyPumpAuthorizationStatus,
    pub bump: u8,
}

pub fn verify_privacy_pump_arcium_receipt(
    receipt_info: &AccountInfo,
    vault_key: Pubkey,
    vault: &PrivacyPumpPrivateVault,
    proposal_key: Pubkey,
    proposal: &PrivacyPumpPrivateProposal,
    expected_action_domain: u8,
    expected_nullifier_hash: [u8; 32],
) -> Result<()> {
    require!(
        receipt_info.owner == &PRIVACY_PUMP_ARCIUM_VERIFIER_PROGRAM_ID,
        PrivacyPumpPrivateVaultError::InvalidArciumReceiptOwner
    );

    let data = receipt_info.try_borrow_data()?;
    require!(
        data.len() >= 8,
        PrivacyPumpPrivateVaultError::InvalidArciumReceipt
    );
    require!(
        data[..8] == privacy_pump_arcium_receipt_discriminator(),
        PrivacyPumpPrivateVaultError::InvalidArciumReceipt
    );

    let receipt = PrivacyPumpArciumReceiptData::try_from_slice(&data[8..])
        .map_err(|_| PrivacyPumpPrivateVaultError::InvalidArciumReceipt)?;

    require!(
        receipt.status == PrivacyPumpAuthorizationStatus::Verified,
        PrivacyPumpPrivateVaultError::ArciumReceiptNotVerified
    );
    require!(
        !receipt.consumed,
        PrivacyPumpPrivateVaultError::ArciumReceiptAlreadyConsumed
    );
    require!(
        receipt.vault == vault_key,
        PrivacyPumpPrivateVaultError::ArciumReceiptVaultMismatch
    );
    require!(
        receipt.proposal == proposal_key,
        PrivacyPumpPrivateVaultError::ArciumReceiptProposalMismatch
    );
    require!(
        receipt.signer_root == proposal.signer_commitment_root_snapshot
            && receipt.signer_root == vault.signer_commitment_root,
        PrivacyPumpPrivateVaultError::ArciumReceiptSignerRootMismatch
    );
    require!(
        receipt.signer_set_version == proposal.signer_set_version_snapshot
            && receipt.signer_set_version == vault.signer_set_version,
        PrivacyPumpPrivateVaultError::ArciumReceiptSignerSetVersionMismatch
    );
    require!(
        receipt.action_domain == expected_action_domain,
        PrivacyPumpPrivateVaultError::ArciumReceiptActionDomainMismatch
    );
    require!(
        receipt.nullifier_hash == expected_nullifier_hash && receipt.nullifier_hash != [0; 32],
        PrivacyPumpPrivateVaultError::ArciumReceiptNullifierMismatch
    );
    require!(
        receipt.payload_hash == proposal.payload_hash,
        PrivacyPumpPrivateVaultError::ArciumReceiptPayloadMismatch
    );
    require!(
        receipt.verified_slot > 0,
        PrivacyPumpPrivateVaultError::ArciumReceiptNotVerified
    );

    Ok(())
}

fn privacy_pump_arcium_receipt_discriminator() -> [u8; 8] {
    let hash = hashv(&[b"account:PrivacyPumpArciumReceipt"]);
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash.to_bytes()[..8]);
    discriminator
}
