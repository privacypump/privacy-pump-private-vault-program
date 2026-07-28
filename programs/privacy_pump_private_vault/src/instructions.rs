use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};
use solana_sha256_hasher::hashv;

use crate::{
    constants::{
        ACTION_DOMAIN_APPROVE, ACTION_DOMAIN_REJECT, PRIVACY_PUMP_ZK_POOL_PROGRAM_ID,
        PRIVATE_CONFIG_SEED, PRIVATE_NULLIFIER_SEED, PRIVATE_PROPOSAL_SEED, PRIVATE_TREASURY_SEED,
        PRIVATE_VAULT_SEED, RECORD_FUNDED_NOTE_DISCRIMINATOR, ZK_POOL_SEED, ZK_POOL_VAULT_SEED,
    },
    errors::PrivacyPumpPrivateVaultError,
    state::{
        PrivacyPumpPrivateExecutionResult, PrivacyPumpPrivateNullifier, PrivacyPumpPrivateProposal,
        PrivacyPumpPrivateProposalStatus, PrivacyPumpPrivateProposalType, PrivacyPumpPrivateVault,
        PrivacyPumpPrivateVaultConfig, PrivacyPumpPrivateVaultStatus,
    },
    verifier_receipt::{
        verify_privacy_pump_arcium_receipt, PRIVACY_PUMP_ARCIUM_VERIFIER_PROGRAM_ID,
    },
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct InitializePrivacyPumpPrivateVaultConfigArgs {
    pub factory: Pubkey,
    pub relayer: Pubkey,
    pub reimbursement_wallet: Pubkey,
    pub arcium_verifier: Pubkey,
    pub min_treasury_rent_reserve_lamports: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreatePrivacyPumpPrivateVaultArgs {
    pub vault_nonce: u64,
    pub threshold: u16,
    pub member_count: u16,
    pub signer_commitment_root: [u8; 32],
    pub signer_set_version: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreatePrivacyPumpPrivateProposalArgs {
    pub proposal_type: PrivacyPumpPrivateProposalType,
    pub payload_hash: [u8; 32],
    pub max_relayer_reimbursement_lamports: u64,
    pub actual_relayer_reimbursement_lamports: u64,
    pub minimum_recipient_receive_lamports: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RecordPrivacyPumpPrivateActionArgs {
    pub nullifier_hash: [u8; 32],
    pub action_domain: u8,
    pub authorization_payload_hash: [u8; 32],
    pub max_relayer_reimbursement_lamports: u64,
    pub actual_relayer_reimbursement_lamports: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ExecutePrivacyPumpPrivateProposalArgs {
    pub recipient: Pubkey,
    pub amount_lamports: u64,
    pub authorization_payload_hash: [u8; 32],
    pub actual_relayer_reimbursement_lamports: u64,
    pub minimum_recipient_receive_lamports: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ExecutePrivacyPumpPrivateProtectedWithdrawalArgs {
    pub commitment: [u8; 32],
    pub amount_lamports: u64,
    pub authorization_payload_hash: [u8; 32],
    pub actual_relayer_reimbursement_lamports: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ExecutePrivacyPumpPrivateThresholdProposalArgs {
    pub current_threshold: u16,
    pub new_threshold: u16,
    pub authorization_payload_hash: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ExecutePrivacyPumpPrivateProfileProposalArgs {
    pub authorization_payload_hash: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ExecutePrivacyPumpPrivateSignerSetChangeArgs {
    pub action_kind: u8,
    pub target_wallet: Pubkey,
    pub proposal_index: u64,
    pub creator_signer_index: u16,
    pub package_id: [u8; 32],
    pub package_version: u16,
    pub metadata_digest: [u8; 32],
    pub current_member_count: u16,
    pub new_member_count: u16,
    pub current_threshold: u16,
    pub new_threshold: u16,
    pub current_signer_set_version: u32,
    pub new_signer_set_version: u32,
    pub current_signer_commitment_root: [u8; 32],
    pub new_signer_commitment_root: [u8; 32],
    pub authorization_payload_hash: [u8; 32],
}

#[derive(Accounts)]
pub struct InitializePrivacyPumpPrivateVaultConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds = [PRIVATE_CONFIG_SEED],
        bump,
        space = 8 + PrivacyPumpPrivateVaultConfig::INIT_SPACE,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(args: CreatePrivacyPumpPrivateVaultArgs)]
pub struct CreatePrivacyPumpPrivateVault<'info> {
    #[account(mut)]
    pub factory: Signer<'info>,
    #[account(
        seeds = [PRIVATE_CONFIG_SEED],
        bump = config.bump,
        constraint = config.factory == factory.key() @ PrivacyPumpPrivateVaultError::InvalidFactory,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    #[account(
        init,
        payer = factory,
        seeds = [PRIVATE_VAULT_SEED, factory.key().as_ref(), &args.vault_nonce.to_le_bytes()],
        bump,
        space = 8 + PrivacyPumpPrivateVault::INIT_SPACE,
    )]
    pub vault: Account<'info, PrivacyPumpPrivateVault>,
    #[account(
        init,
        payer = factory,
        seeds = [PRIVATE_TREASURY_SEED, vault.key().as_ref()],
        bump,
        space = 0,
    )]
    /// CHECK: Treasury PDA stores no account data.
    pub treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePrivacyPumpPrivateProposal<'info> {
    #[account(mut)]
    pub relayer: Signer<'info>,
    #[account(
        seeds = [PRIVATE_CONFIG_SEED],
        bump = config.bump,
        constraint = config.relayer == relayer.key() @ PrivacyPumpPrivateVaultError::InvalidRelayer,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    #[account(mut, constraint = vault.status == PrivacyPumpPrivateVaultStatus::Active)]
    pub vault: Account<'info, PrivacyPumpPrivateVault>,
    #[account(
        mut,
        constraint = treasury.key() == vault.treasury @ PrivacyPumpPrivateVaultError::InvalidTreasury,
    )]
    /// CHECK: Treasury PDA is derived from the private vault and signs with program seeds.
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Reimbursement recipient is pinned into the proposal by the trusted relayer.
    pub reimbursement_wallet: UncheckedAccount<'info>,
    #[account(
        init,
        payer = relayer,
        seeds = [PRIVATE_PROPOSAL_SEED, vault.key().as_ref(), &vault.proposal_count.to_le_bytes()],
        bump,
        space = 8 + PrivacyPumpPrivateProposal::INIT_SPACE,
    )]
    pub proposal: Account<'info, PrivacyPumpPrivateProposal>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(args: RecordPrivacyPumpPrivateActionArgs)]
pub struct RecordPrivacyPumpPrivateAction<'info> {
    #[account(mut)]
    pub relayer: Signer<'info>,
    #[account(
        seeds = [PRIVATE_CONFIG_SEED],
        bump = config.bump,
        constraint = config.relayer == relayer.key() @ PrivacyPumpPrivateVaultError::InvalidRelayer,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    #[account(
        constraint = vault.status == PrivacyPumpPrivateVaultStatus::Active,
    )]
    pub vault: Account<'info, PrivacyPumpPrivateVault>,
    #[account(
        mut,
        constraint = proposal.vault == vault.key(),
        constraint = proposal.status == PrivacyPumpPrivateProposalStatus::Pending @ PrivacyPumpPrivateVaultError::ProposalNotPending,
    )]
    pub proposal: Account<'info, PrivacyPumpPrivateProposal>,
    #[account(
        mut,
        constraint = treasury.key() == vault.treasury @ PrivacyPumpPrivateVaultError::InvalidTreasury,
    )]
    /// CHECK: Treasury PDA is derived from the private vault and signs with program seeds.
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Reimbursement recipient must match the wallet pinned into the proposal.
    pub reimbursement_wallet: UncheckedAccount<'info>,
    #[account(
        init,
        payer = relayer,
        seeds = [
            PRIVATE_NULLIFIER_SEED,
            vault.key().as_ref(),
            proposal.key().as_ref(),
            arcium_receipt.key().as_ref(),
        ],
        bump,
        space = 8 + PrivacyPumpPrivateNullifier::INIT_SPACE,
    )]
    pub nullifier: Account<'info, PrivacyPumpPrivateNullifier>,
    /// CHECK: Verified manually as a PrivacyPumpArciumVerifier-owned receipt account.
    pub arcium_receipt: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecutePrivacyPumpPrivateProposal<'info> {
    #[account(mut)]
    pub relayer: Signer<'info>,
    #[account(
        seeds = [PRIVATE_CONFIG_SEED],
        bump = config.bump,
        constraint = config.relayer == relayer.key() @ PrivacyPumpPrivateVaultError::InvalidRelayer,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    pub vault: Account<'info, PrivacyPumpPrivateVault>,
    #[account(
        mut,
        constraint = proposal.vault == vault.key(),
    )]
    pub proposal: Account<'info, PrivacyPumpPrivateProposal>,
    #[account(
        mut,
        constraint = treasury.key() == vault.treasury @ PrivacyPumpPrivateVaultError::InvalidTreasury,
    )]
    /// CHECK: Treasury PDA is derived from the private vault and signs with program seeds.
    pub treasury: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = recipient.key() != treasury.key() @ PrivacyPumpPrivateVaultError::InvalidRecipient,
    )]
    /// CHECK: Recipient can be any system-owned account or uninitialized address funded by transfer.
    pub recipient: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Reimbursement recipient must match the wallet pinned into the proposal.
    pub reimbursement_wallet: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ExecutePrivacyPumpPrivateProtectedWithdrawal<'info> {
    #[account(mut)]
    pub relayer: Signer<'info>,
    #[account(
        seeds = [PRIVATE_CONFIG_SEED],
        bump = config.bump,
        constraint = config.relayer == relayer.key() @ PrivacyPumpPrivateVaultError::InvalidRelayer,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    pub vault: Account<'info, PrivacyPumpPrivateVault>,
    #[account(
        mut,
        constraint = proposal.vault == vault.key(),
    )]
    pub proposal: Account<'info, PrivacyPumpPrivateProposal>,
    #[account(
        mut,
        seeds = [PRIVATE_TREASURY_SEED, vault.key().as_ref()],
        bump = vault.treasury_bump,
        constraint = treasury.key() == vault.treasury @ PrivacyPumpPrivateVaultError::InvalidTreasury,
    )]
    /// CHECK: Treasury PDA is derived from the private vault and signs the ZK Pool deposit CPI.
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: Reimbursement recipient must match the wallet pinned into the proposal.
    pub reimbursement_wallet: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [ZK_POOL_SEED],
        bump,
        seeds::program = zk_pool_program.key(),
    )]
    /// CHECK: ZK Pool account is owned and validated by the ZK Pool program during CPI.
    pub zk_pool: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [ZK_POOL_VAULT_SEED, zk_pool.key().as_ref()],
        bump,
        seeds::program = zk_pool_program.key(),
    )]
    /// CHECK: Pool vault PDA is validated by the ZK Pool program during CPI.
    pub pool_vault: UncheckedAccount<'info>,
    /// CHECK: Must be the deployed Privacy Pump ZK Pool program.
    pub zk_pool_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecutePrivacyPumpPrivateThresholdProposal<'info> {
    #[account(mut)]
    pub relayer: Signer<'info>,
    #[account(
        seeds = [PRIVATE_CONFIG_SEED],
        bump = config.bump,
        constraint = config.relayer == relayer.key() @ PrivacyPumpPrivateVaultError::InvalidRelayer,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    #[account(mut, constraint = vault.status == PrivacyPumpPrivateVaultStatus::Active)]
    pub vault: Account<'info, PrivacyPumpPrivateVault>,
    #[account(
        mut,
        constraint = proposal.vault == vault.key(),
    )]
    pub proposal: Account<'info, PrivacyPumpPrivateProposal>,
}

#[derive(Accounts)]
pub struct ExecutePrivacyPumpPrivateProfileProposal<'info> {
    #[account(mut)]
    pub relayer: Signer<'info>,
    #[account(
        seeds = [PRIVATE_CONFIG_SEED],
        bump = config.bump,
        constraint = config.relayer == relayer.key() @ PrivacyPumpPrivateVaultError::InvalidRelayer,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    #[account(constraint = vault.status == PrivacyPumpPrivateVaultStatus::Active)]
    pub vault: Account<'info, PrivacyPumpPrivateVault>,
    #[account(
        mut,
        constraint = proposal.vault == vault.key(),
    )]
    pub proposal: Account<'info, PrivacyPumpPrivateProposal>,
}

#[derive(Accounts)]
pub struct ExecutePrivacyPumpPrivateSignerSetChange<'info> {
    #[account(mut)]
    pub relayer: Signer<'info>,
    #[account(
        seeds = [PRIVATE_CONFIG_SEED],
        bump = config.bump,
        constraint = config.relayer == relayer.key() @ PrivacyPumpPrivateVaultError::InvalidRelayer,
    )]
    pub config: Account<'info, PrivacyPumpPrivateVaultConfig>,
    #[account(mut, constraint = vault.status == PrivacyPumpPrivateVaultStatus::Active)]
    pub vault: Account<'info, PrivacyPumpPrivateVault>,
    #[account(
        mut,
        constraint = proposal.vault == vault.key(),
    )]
    pub proposal: Account<'info, PrivacyPumpPrivateProposal>,
}

pub fn handle_initialize_config(
    ctx: Context<InitializePrivacyPumpPrivateVaultConfig>,
    args: InitializePrivacyPumpPrivateVaultConfigArgs,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.authority = ctx.accounts.authority.key();
    config.factory = args.factory;
    config.relayer = args.relayer;
    config.arcium_verifier = args.arcium_verifier;
    config.bump = ctx.bumps.config;
    Ok(())
}

pub fn handle_create_vault(
    ctx: Context<CreatePrivacyPumpPrivateVault>,
    args: CreatePrivacyPumpPrivateVaultArgs,
) -> Result<()> {
    require!(
        args.threshold > 0 && args.threshold <= args.member_count,
        PrivacyPumpPrivateVaultError::InvalidThreshold
    );
    require!(
        args.signer_commitment_root != [0; 32],
        PrivacyPumpPrivateVaultError::InvalidSignerCommitmentRoot
    );

    let vault = &mut ctx.accounts.vault;
    vault.treasury = ctx.accounts.treasury.key();
    vault.threshold = args.threshold;
    vault.member_count = args.member_count;
    vault.signer_commitment_root = args.signer_commitment_root;
    vault.signer_set_version = args.signer_set_version;
    vault.proposal_count = 0;
    vault.arcium_verifier = ctx.accounts.config.arcium_verifier;
    vault.created_at = Clock::get()?.unix_timestamp;
    vault.status = PrivacyPumpPrivateVaultStatus::Active;
    vault.bump = ctx.bumps.vault;
    vault.treasury_bump = ctx.bumps.treasury;
    Ok(())
}

pub fn handle_create_proposal(
    ctx: Context<CreatePrivacyPumpPrivateProposal>,
    args: CreatePrivacyPumpPrivateProposalArgs,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let proposal = &mut ctx.accounts.proposal;
    let proposal_id = vault.proposal_count;
    reimburse_from_treasury(
        &ctx.accounts.treasury.to_account_info(),
        &ctx.accounts.reimbursement_wallet.to_account_info(),
        args.actual_relayer_reimbursement_lamports,
        args.max_relayer_reimbursement_lamports,
        0,
    )?;

    proposal.vault = vault.key();
    proposal.proposal_id = proposal_id;
    proposal.proposal_type = args.proposal_type;
    proposal.payload_hash = args.payload_hash;
    proposal.threshold_snapshot = vault.threshold;
    proposal.member_count_snapshot = vault.member_count;
    proposal.signer_commitment_root_snapshot = vault.signer_commitment_root;
    proposal.signer_set_version_snapshot = vault.signer_set_version;
    proposal.approval_count = 0;
    proposal.rejection_count = 0;
    proposal.status = PrivacyPumpPrivateProposalStatus::Pending;
    proposal.execution_result = PrivacyPumpPrivateExecutionResult::NotExecuted;
    proposal.created_at = Clock::get()?.unix_timestamp;
    proposal.executed_at = 0;
    proposal.max_relayer_reimbursement_lamports = args.max_relayer_reimbursement_lamports;
    proposal.minimum_recipient_receive_lamports = args.minimum_recipient_receive_lamports;
    proposal.reimbursement_wallet = ctx.accounts.reimbursement_wallet.key();
    proposal.fee_protection_enabled = true;
    proposal.bump = ctx.bumps.proposal;

    vault.proposal_count = vault.proposal_count.saturating_add(1);
    Ok(())
}

pub fn handle_record_private_action(
    ctx: Context<RecordPrivacyPumpPrivateAction>,
    args: RecordPrivacyPumpPrivateActionArgs,
) -> Result<()> {
    require!(
        args.nullifier_hash != [0; 32],
        PrivacyPumpPrivateVaultError::InvalidPrivateNullifier
    );
    require!(
        args.action_domain == ACTION_DOMAIN_APPROVE || args.action_domain == ACTION_DOMAIN_REJECT,
        PrivacyPumpPrivateVaultError::InvalidActionDomain
    );

    verify_arcium_private_authorization(&ctx, &args)?;
    require!(
        ctx.accounts.proposal.fee_protection_enabled,
        PrivacyPumpPrivateVaultError::LegacyProposalMissingFeeProtection
    );
    require!(
        ctx.accounts.proposal.reimbursement_wallet == ctx.accounts.reimbursement_wallet.key(),
        PrivacyPumpPrivateVaultError::InvalidReimbursementWallet
    );
    reimburse_from_treasury(
        &ctx.accounts.treasury.to_account_info(),
        &ctx.accounts.reimbursement_wallet.to_account_info(),
        args.actual_relayer_reimbursement_lamports,
        args.max_relayer_reimbursement_lamports,
        0,
    )?;

    let proposal = &mut ctx.accounts.proposal;
    if args.action_domain == ACTION_DOMAIN_APPROVE {
        proposal.approval_count = proposal.approval_count.saturating_add(1);
        if proposal.approval_count >= proposal.threshold_snapshot {
            proposal.status = PrivacyPumpPrivateProposalStatus::Approved;
        }
    } else {
        proposal.rejection_count = proposal.rejection_count.saturating_add(1);
        let rejection_limit = proposal
            .member_count_snapshot
            .saturating_sub(proposal.threshold_snapshot)
            .saturating_add(1);
        if proposal.rejection_count >= rejection_limit {
            proposal.status = PrivacyPumpPrivateProposalStatus::Rejected;
        }
    }

    let nullifier = &mut ctx.accounts.nullifier;
    nullifier.vault = ctx.accounts.vault.key();
    nullifier.proposal = proposal.key();
    nullifier.nullifier_hash = args.nullifier_hash;
    nullifier.action_domain = args.action_domain;
    nullifier.signer_set_version = proposal.signer_set_version_snapshot;
    nullifier.recorded_at = Clock::get()?.unix_timestamp;
    nullifier.bump = ctx.bumps.nullifier;
    Ok(())
}

pub fn handle_execute_proposal(
    ctx: Context<ExecutePrivacyPumpPrivateProposal>,
    args: ExecutePrivacyPumpPrivateProposalArgs,
) -> Result<()> {
    require!(
        ctx.accounts.proposal.status == PrivacyPumpPrivateProposalStatus::Approved,
        PrivacyPumpPrivateVaultError::ProposalNotApproved
    );
    require!(
        ctx.accounts.proposal.fee_protection_enabled,
        PrivacyPumpPrivateVaultError::LegacyProposalMissingFeeProtection
    );
    require!(
        ctx.accounts.proposal.proposal_type == PrivacyPumpPrivateProposalType::TransferSol,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.amount_lamports > 0
            && args.authorization_payload_hash == ctx.accounts.proposal.payload_hash
            && args.recipient == ctx.accounts.recipient.key(),
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        ctx.accounts.proposal.reimbursement_wallet == ctx.accounts.reimbursement_wallet.key(),
        PrivacyPumpPrivateVaultError::InvalidReimbursementWallet
    );
    require!(
        args.actual_relayer_reimbursement_lamports
            <= ctx.accounts.proposal.max_relayer_reimbursement_lamports,
        PrivacyPumpPrivateVaultError::RelayerReimbursementExceedsCap
    );
    require!(
        args.minimum_recipient_receive_lamports
            == ctx.accounts.proposal.minimum_recipient_receive_lamports,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    let recipient_amount = args.amount_lamports;
    require!(
        recipient_amount >= args.minimum_recipient_receive_lamports,
        PrivacyPumpPrivateVaultError::WithdrawalBelowMinimumRecipientReceive
    );
    let expected_payload_hash = standard_withdrawal_payload_hash(StandardWithdrawalPayload {
        vault: ctx.accounts.vault.key(),
        treasury: ctx.accounts.treasury.key(),
        recipient: args.recipient,
        gross_amount_lamports: args.amount_lamports,
        max_relayer_reimbursement_lamports: ctx
            .accounts
            .proposal
            .max_relayer_reimbursement_lamports,
        minimum_recipient_receive_lamports: args.minimum_recipient_receive_lamports,
        reimbursement_wallet: ctx.accounts.reimbursement_wallet.key(),
        signer_set_version: ctx.accounts.proposal.signer_set_version_snapshot,
    });
    require!(
        expected_payload_hash == ctx.accounts.proposal.payload_hash,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    let total_treasury_debit = args
        .amount_lamports
        .checked_add(args.actual_relayer_reimbursement_lamports)
        .ok_or(PrivacyPumpPrivateVaultError::LamportMathOverflow)?;
    require!(
        ctx.accounts.treasury.lamports() >= total_treasury_debit,
        PrivacyPumpPrivateVaultError::TreasuryInsufficientFunds
    );
    require!(
        ctx.accounts.treasury.is_writable && ctx.accounts.recipient.is_writable,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );

    let treasury_info = ctx.accounts.treasury.to_account_info();
    let recipient_info = ctx.accounts.recipient.to_account_info();
    require!(
        treasury_info.owner == &crate::ID,
        PrivacyPumpPrivateVaultError::InvalidTreasury
    );
    let treasury_balance = treasury_info.lamports();
    let treasury_data_len = treasury_info.data_len();
    let rent_reserve = if treasury_data_len > 0 {
        Rent::get()?.minimum_balance(treasury_data_len)
    } else {
        0
    };
    let spendable_balance = treasury_balance
        .checked_sub(rent_reserve)
        .ok_or(PrivacyPumpPrivateVaultError::TreasuryRentReserve)?;
    require!(
        spendable_balance >= total_treasury_debit,
        PrivacyPumpPrivateVaultError::TreasuryInsufficientFunds
    );

    let next_treasury_balance = treasury_balance
        .checked_sub(total_treasury_debit)
        .ok_or(PrivacyPumpPrivateVaultError::TreasuryInsufficientFunds)?;
    let next_recipient_balance = recipient_info
        .lamports()
        .checked_add(recipient_amount)
        .ok_or(PrivacyPumpPrivateVaultError::LamportMathOverflow)?;
    let reimbursement_info = ctx.accounts.reimbursement_wallet.to_account_info();
    let next_reimbursement_balance = reimbursement_info
        .lamports()
        .checked_add(args.actual_relayer_reimbursement_lamports)
        .ok_or(PrivacyPumpPrivateVaultError::LamportMathOverflow)?;

    **treasury_info.try_borrow_mut_lamports()? = next_treasury_balance;
    **recipient_info.try_borrow_mut_lamports()? = next_recipient_balance;
    **reimbursement_info.try_borrow_mut_lamports()? = next_reimbursement_balance;

    let proposal = &mut ctx.accounts.proposal;
    proposal.status = PrivacyPumpPrivateProposalStatus::Executed;
    proposal.execution_result = PrivacyPumpPrivateExecutionResult::Succeeded;
    proposal.executed_at = Clock::get()?.unix_timestamp;
    Ok(())
}

pub fn handle_execute_protected_withdrawal(
    ctx: Context<ExecutePrivacyPumpPrivateProtectedWithdrawal>,
    args: ExecutePrivacyPumpPrivateProtectedWithdrawalArgs,
) -> Result<()> {
    require!(
        ctx.accounts.proposal.status == PrivacyPumpPrivateProposalStatus::Approved,
        PrivacyPumpPrivateVaultError::ProposalNotApproved
    );
    require!(
        ctx.accounts.proposal.fee_protection_enabled,
        PrivacyPumpPrivateVaultError::LegacyProposalMissingFeeProtection
    );
    require!(
        ctx.accounts.proposal.proposal_type == PrivacyPumpPrivateProposalType::ProtectedWithdrawal,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.amount_lamports > 0
            && args.authorization_payload_hash == ctx.accounts.proposal.payload_hash
            && args.commitment == ctx.accounts.proposal.payload_hash,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        ctx.accounts.proposal.reimbursement_wallet == ctx.accounts.reimbursement_wallet.key(),
        PrivacyPumpPrivateVaultError::InvalidReimbursementWallet
    );
    require!(
        ctx.accounts.zk_pool_program.key() == PRIVACY_PUMP_ZK_POOL_PROGRAM_ID,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.actual_relayer_reimbursement_lamports
            <= ctx.accounts.proposal.max_relayer_reimbursement_lamports,
        PrivacyPumpPrivateVaultError::RelayerReimbursementExceedsCap
    );

    let total_treasury_debit = args
        .amount_lamports
        .checked_add(args.actual_relayer_reimbursement_lamports)
        .ok_or(PrivacyPumpPrivateVaultError::LamportMathOverflow)?;
    let treasury_info = ctx.accounts.treasury.to_account_info();
    let treasury_balance = treasury_info.lamports();
    let rent_reserve = if treasury_info.data_len() > 0 {
        Rent::get()?.minimum_balance(treasury_info.data_len())
    } else {
        0
    };
    let spendable_balance = treasury_balance
        .checked_sub(rent_reserve)
        .ok_or(PrivacyPumpPrivateVaultError::TreasuryRentReserve)?;
    require!(
        spendable_balance >= total_treasury_debit,
        PrivacyPumpPrivateVaultError::TreasuryInsufficientFunds
    );
    require!(
        ctx.accounts.treasury.is_writable
            && ctx.accounts.pool_vault.is_writable
            && ctx.accounts.reimbursement_wallet.is_writable,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );

    let pool_vault_info = ctx.accounts.pool_vault.to_account_info();
    let reimbursement_info = ctx.accounts.reimbursement_wallet.to_account_info();
    let pool_vault_lamports_before = pool_vault_info.lamports();

    let mut record_data = Vec::with_capacity(56);
    record_data.extend_from_slice(&RECORD_FUNDED_NOTE_DISCRIMINATOR);
    record_data.extend_from_slice(&args.commitment);
    record_data.extend_from_slice(&args.amount_lamports.to_le_bytes());
    record_data.extend_from_slice(&pool_vault_lamports_before.to_le_bytes());
    let record_ix = Instruction {
        program_id: ctx.accounts.zk_pool_program.key(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.zk_pool.key(), false),
            AccountMeta::new(ctx.accounts.pool_vault.key(), false),
            AccountMeta::new_readonly(ctx.accounts.treasury.key(), true),
        ],
        data: record_data,
    };
    let vault_key = ctx.accounts.vault.key();
    let treasury_signer_seeds: &[&[u8]] = &[
        PRIVATE_TREASURY_SEED,
        vault_key.as_ref(),
        &[ctx.accounts.vault.treasury_bump],
    ];
    let next_treasury_balance = treasury_balance
        .checked_sub(total_treasury_debit)
        .ok_or(PrivacyPumpPrivateVaultError::TreasuryInsufficientFunds)?;
    let next_pool_vault_balance = pool_vault_lamports_before
        .checked_add(args.amount_lamports)
        .ok_or(PrivacyPumpPrivateVaultError::LamportMathOverflow)?;
    let next_reimbursement_balance = reimbursement_info
        .lamports()
        .checked_add(args.actual_relayer_reimbursement_lamports)
        .ok_or(PrivacyPumpPrivateVaultError::LamportMathOverflow)?;
    invoke_signed(
        &record_ix,
        &[
            ctx.accounts.zk_pool.to_account_info(),
            pool_vault_info.clone(),
            treasury_info.clone(),
        ],
        &[treasury_signer_seeds],
    )?;
    **treasury_info.try_borrow_mut_lamports()? = next_treasury_balance;
    **pool_vault_info.try_borrow_mut_lamports()? = next_pool_vault_balance;
    **reimbursement_info.try_borrow_mut_lamports()? = next_reimbursement_balance;

    let proposal = &mut ctx.accounts.proposal;
    proposal.status = PrivacyPumpPrivateProposalStatus::Executed;
    proposal.execution_result = PrivacyPumpPrivateExecutionResult::Succeeded;
    proposal.executed_at = Clock::get()?.unix_timestamp;
    Ok(())
}

pub fn handle_execute_threshold_proposal(
    ctx: Context<ExecutePrivacyPumpPrivateThresholdProposal>,
    args: ExecutePrivacyPumpPrivateThresholdProposalArgs,
) -> Result<()> {
    require!(
        ctx.accounts.proposal.status == PrivacyPumpPrivateProposalStatus::Approved,
        PrivacyPumpPrivateVaultError::ProposalNotApproved
    );
    require!(
        ctx.accounts.proposal.proposal_type == PrivacyPumpPrivateProposalType::UpdateSettings,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.current_threshold == ctx.accounts.vault.threshold
            && args.new_threshold > 0
            && args.new_threshold <= ctx.accounts.vault.member_count
            && args.authorization_payload_hash == ctx.accounts.proposal.payload_hash,
        PrivacyPumpPrivateVaultError::InvalidThreshold
    );

    let vault = &mut ctx.accounts.vault;
    vault.threshold = args.new_threshold;

    let proposal = &mut ctx.accounts.proposal;
    proposal.status = PrivacyPumpPrivateProposalStatus::Executed;
    proposal.execution_result = PrivacyPumpPrivateExecutionResult::Succeeded;
    proposal.executed_at = Clock::get()?.unix_timestamp;
    Ok(())
}

pub fn handle_execute_profile_proposal(
    ctx: Context<ExecutePrivacyPumpPrivateProfileProposal>,
    args: ExecutePrivacyPumpPrivateProfileProposalArgs,
) -> Result<()> {
    require!(
        ctx.accounts.proposal.status == PrivacyPumpPrivateProposalStatus::Approved,
        PrivacyPumpPrivateVaultError::ProposalNotApproved
    );
    require!(
        ctx.accounts.proposal.proposal_type == PrivacyPumpPrivateProposalType::UpdateSettings,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.authorization_payload_hash == ctx.accounts.proposal.payload_hash,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    let proposal = &mut ctx.accounts.proposal;
    proposal.status = PrivacyPumpPrivateProposalStatus::Executed;
    proposal.execution_result = PrivacyPumpPrivateExecutionResult::Succeeded;
    proposal.executed_at = Clock::get()?.unix_timestamp;
    Ok(())
}

pub fn handle_execute_signer_set_change(
    ctx: Context<ExecutePrivacyPumpPrivateSignerSetChange>,
    args: ExecutePrivacyPumpPrivateSignerSetChangeArgs,
) -> Result<()> {
    const SIGNER_SET_ACTION_ADD_MEMBER: u8 = 1;
    const SIGNER_SET_ACTION_REMOVE_MEMBER: u8 = 2;

    require!(
        ctx.accounts.proposal.status == PrivacyPumpPrivateProposalStatus::Approved,
        PrivacyPumpPrivateVaultError::ProposalNotApproved
    );
    require!(
        ctx.accounts.proposal.proposal_type == PrivacyPumpPrivateProposalType::SignerSetChange,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        ctx.accounts.proposal.signer_set_version_snapshot == ctx.accounts.vault.signer_set_version,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.current_member_count == ctx.accounts.vault.member_count
            && args.current_threshold == ctx.accounts.vault.threshold
            && args.current_signer_set_version == ctx.accounts.vault.signer_set_version,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.current_signer_commitment_root == ctx.accounts.vault.signer_commitment_root,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.action_kind == SIGNER_SET_ACTION_ADD_MEMBER
            || args.action_kind == SIGNER_SET_ACTION_REMOVE_MEMBER,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    if args.action_kind == SIGNER_SET_ACTION_ADD_MEMBER {
        require!(
            args.new_member_count == args.current_member_count.saturating_add(1),
            PrivacyPumpPrivateVaultError::InvalidExecutionPayload
        );
    } else {
        require!(
            args.current_member_count > 1
                && args.new_member_count.saturating_add(1) == args.current_member_count,
            PrivacyPumpPrivateVaultError::InvalidThreshold
        );
    }
    require!(
        args.new_member_count > 0
            && args.new_threshold > 0
            && args.new_threshold <= args.new_member_count,
        PrivacyPumpPrivateVaultError::InvalidThreshold
    );
    require!(
        args.new_signer_set_version == ctx.accounts.vault.signer_set_version.saturating_add(1),
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.new_signer_commitment_root != [0; 32],
        PrivacyPumpPrivateVaultError::InvalidSignerCommitmentRoot
    );
    require!(
        args.authorization_payload_hash == ctx.accounts.proposal.payload_hash,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );
    require!(
        args.proposal_index == ctx.accounts.proposal.proposal_id
            && args.creator_signer_index > 0
            && args.creator_signer_index <= args.current_member_count
            && args.target_wallet != Pubkey::default()
            && args.package_id != [0; 32]
            && args.package_version > 0
            && args.metadata_digest != [0; 32],
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );

    let expected_payload_hash = signer_set_change_payload_hash(SignerSetChangePayload {
        vault: ctx.accounts.vault.key(),
        action_kind: args.action_kind,
        target_wallet: args.target_wallet,
        proposal_index: args.proposal_index,
        creator_signer_index: args.creator_signer_index,
        package_id: args.package_id,
        package_version: args.package_version,
        metadata_digest: args.metadata_digest,
        current_member_count: args.current_member_count,
        new_member_count: args.new_member_count,
        current_threshold: args.current_threshold,
        new_threshold: args.new_threshold,
        current_signer_set_version: args.current_signer_set_version,
        new_signer_set_version: args.new_signer_set_version,
        current_signer_commitment_root: args.current_signer_commitment_root,
        new_signer_commitment_root: args.new_signer_commitment_root,
    });
    require!(
        expected_payload_hash == ctx.accounts.proposal.payload_hash,
        PrivacyPumpPrivateVaultError::InvalidExecutionPayload
    );

    let vault = &mut ctx.accounts.vault;
    vault.member_count = args.new_member_count;
    vault.threshold = args.new_threshold;
    vault.signer_commitment_root = args.new_signer_commitment_root;
    vault.signer_set_version = args.new_signer_set_version;

    let proposal = &mut ctx.accounts.proposal;
    proposal.status = PrivacyPumpPrivateProposalStatus::Executed;
    proposal.execution_result = PrivacyPumpPrivateExecutionResult::Succeeded;
    proposal.executed_at = Clock::get()?.unix_timestamp;
    Ok(())
}

fn verify_arcium_private_authorization(
    ctx: &Context<RecordPrivacyPumpPrivateAction>,
    args: &RecordPrivacyPumpPrivateActionArgs,
) -> Result<()> {
    require!(
        ctx.accounts.vault.arcium_verifier == PRIVACY_PUMP_ARCIUM_VERIFIER_PROGRAM_ID,
        PrivacyPumpPrivateVaultError::InvalidArciumVerifier
    );
    verify_privacy_pump_arcium_receipt(
        &ctx.accounts.arcium_receipt.to_account_info(),
        ctx.accounts.vault.key(),
        &ctx.accounts.vault,
        ctx.accounts.proposal.key(),
        &ctx.accounts.proposal,
        args.action_domain,
        args.nullifier_hash,
    )
}

pub struct StandardWithdrawalPayload {
    vault: Pubkey,
    treasury: Pubkey,
    recipient: Pubkey,
    gross_amount_lamports: u64,
    max_relayer_reimbursement_lamports: u64,
    minimum_recipient_receive_lamports: u64,
    reimbursement_wallet: Pubkey,
    signer_set_version: u32,
}

pub struct SignerSetChangePayload {
    vault: Pubkey,
    action_kind: u8,
    target_wallet: Pubkey,
    proposal_index: u64,
    creator_signer_index: u16,
    package_id: [u8; 32],
    package_version: u16,
    metadata_digest: [u8; 32],
    current_member_count: u16,
    new_member_count: u16,
    current_threshold: u16,
    new_threshold: u16,
    current_signer_set_version: u32,
    new_signer_set_version: u32,
    current_signer_commitment_root: [u8; 32],
    new_signer_commitment_root: [u8; 32],
}

pub fn standard_withdrawal_payload_hash(payload: StandardWithdrawalPayload) -> [u8; 32] {
    let canonical = format!(
        "{{\"action\":\"Withdraw\",\"actionDomain\":\"STANDARD_WITHDRAWAL_V1\",\"asset\":\"SOL\",\"grossAmountLamports\":\"{}\",\"maxRelayerReimbursementLamports\":\"{}\",\"minimumRecipientReceiveLamports\":\"{}\",\"privacyMode\":\"standard\",\"recipient\":\"{}\",\"reimbursementWallet\":\"{}\",\"routeBatching\":\"solo\",\"routeTiming\":\"instant\",\"signerSetVersion\":{},\"treasury\":\"{}\",\"vault\":\"{}\"}}",
        payload.gross_amount_lamports,
        payload.max_relayer_reimbursement_lamports,
        payload.minimum_recipient_receive_lamports,
        payload.recipient,
        payload.reimbursement_wallet,
        payload.signer_set_version,
        payload.treasury,
        payload.vault
    );
    hashv(&[canonical.as_bytes()]).to_bytes()
}

pub fn signer_set_change_payload_hash(payload: SignerSetChangePayload) -> [u8; 32] {
    let action_kind = if payload.action_kind == 1 {
        "AddMember"
    } else {
        "RemoveMember"
    };
    let current_root = payload
        .current_signer_commitment_root
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    let new_root = payload
        .new_signer_commitment_root
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    let package_id = payload
        .package_id
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    let metadata_digest = payload
        .metadata_digest
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    let canonical = format!(
        "{{\"action\":\"SignerSetChange\",\"actionDomain\":\"SIGNER_SET_CHANGE_V2\",\"actionKind\":\"{}\",\"creatorSignerIndex\":{},\"currentMemberCount\":{},\"currentSignerCommitmentRoot\":\"{}\",\"currentSignerSetVersion\":{},\"currentThreshold\":{},\"metadataDigest\":\"{}\",\"newMemberCount\":{},\"newSignerCommitmentRoot\":\"{}\",\"newSignerSetVersion\":{},\"newThreshold\":{},\"packageId\":\"{}\",\"packageVersion\":{},\"proposalIndex\":\"{}\",\"targetWallet\":\"{}\",\"vault\":\"{}\"}}",
        action_kind,
        payload.creator_signer_index,
        payload.current_member_count,
        current_root,
        payload.current_signer_set_version,
        payload.current_threshold,
        metadata_digest,
        payload.new_member_count,
        new_root,
        payload.new_signer_set_version,
        payload.new_threshold,
        package_id,
        payload.package_version,
        payload.proposal_index,
        payload.target_wallet,
        payload.vault
    );
    hashv(&[canonical.as_bytes()]).to_bytes()
}

fn reimburse_from_treasury<'info>(
    treasury_info: &AccountInfo<'info>,
    reimbursement_info: &AccountInfo<'info>,
    actual_lamports: u64,
    max_lamports: u64,
    min_reserve_lamports: u64,
) -> Result<()> {
    require!(
        actual_lamports <= max_lamports,
        PrivacyPumpPrivateVaultError::RelayerReimbursementExceedsCap
    );
    require!(
        treasury_info.owner == &crate::ID,
        PrivacyPumpPrivateVaultError::InvalidTreasury
    );
    if actual_lamports == 0 {
        return Ok(());
    }
    let treasury_balance = treasury_info.lamports();
    let rent_reserve = if treasury_info.data_len() > 0 {
        Rent::get()?.minimum_balance(treasury_info.data_len())
    } else {
        0
    }
    .max(min_reserve_lamports);
    let spendable_balance = treasury_balance
        .checked_sub(rent_reserve)
        .ok_or(PrivacyPumpPrivateVaultError::TreasuryRentReserve)?;
    require!(
        spendable_balance >= actual_lamports,
        PrivacyPumpPrivateVaultError::TreasuryInsufficientFunds
    );
    let next_treasury_balance = treasury_balance
        .checked_sub(actual_lamports)
        .ok_or(PrivacyPumpPrivateVaultError::TreasuryInsufficientFunds)?;
    let next_reimbursement_balance = reimbursement_info
        .lamports()
        .checked_add(actual_lamports)
        .ok_or(PrivacyPumpPrivateVaultError::LamportMathOverflow)?;

    **treasury_info.try_borrow_mut_lamports()? = next_treasury_balance;
    **reimbursement_info.try_borrow_mut_lamports()? = next_reimbursement_balance;
    Ok(())
}
