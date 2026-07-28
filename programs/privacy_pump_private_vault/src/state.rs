use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PrivacyPumpPrivateVaultConfig {
    pub authority: Pubkey,
    pub factory: Pubkey,
    pub relayer: Pubkey,
    pub arcium_verifier: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PrivacyPumpPrivateVault {
    pub treasury: Pubkey,
    pub threshold: u16,
    pub member_count: u16,
    pub signer_commitment_root: [u8; 32],
    pub signer_set_version: u32,
    pub proposal_count: u64,
    pub arcium_verifier: Pubkey,
    pub created_at: i64,
    pub status: PrivacyPumpPrivateVaultStatus,
    pub bump: u8,
    pub treasury_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PrivacyPumpPrivateProposal {
    pub vault: Pubkey,
    pub proposal_id: u64,
    pub proposal_type: PrivacyPumpPrivateProposalType,
    pub payload_hash: [u8; 32],
    pub threshold_snapshot: u16,
    pub member_count_snapshot: u16,
    pub signer_commitment_root_snapshot: [u8; 32],
    pub signer_set_version_snapshot: u32,
    pub approval_count: u16,
    pub rejection_count: u16,
    pub status: PrivacyPumpPrivateProposalStatus,
    pub execution_result: PrivacyPumpPrivateExecutionResult,
    pub created_at: i64,
    pub executed_at: i64,
    pub max_relayer_reimbursement_lamports: u64,
    pub minimum_recipient_receive_lamports: u64,
    pub reimbursement_wallet: Pubkey,
    pub fee_protection_enabled: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PrivacyPumpPrivateNullifier {
    pub vault: Pubkey,
    pub proposal: Pubkey,
    pub nullifier_hash: [u8; 32],
    pub action_domain: u8,
    pub signer_set_version: u32,
    pub recorded_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, InitSpace, PartialEq)]
pub enum PrivacyPumpPrivateVaultStatus {
    Active,
    Frozen,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, InitSpace, PartialEq)]
pub enum PrivacyPumpPrivateProposalType {
    TransferSol,
    ProtectedWithdrawal,
    UpdateSettings,
    SignerSetChange,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, InitSpace, PartialEq)]
pub enum PrivacyPumpPrivateProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, InitSpace, PartialEq)]
pub enum PrivacyPumpPrivateExecutionResult {
    NotExecuted,
    Succeeded,
    Failed,
}
