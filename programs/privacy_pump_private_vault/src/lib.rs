use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod verifier_receipt;

use instructions::*;

declare_id!("DQEq5ftpcvjdg656PWxayafozUX95jCmZ9zEn82qDmfx");

#[program]
pub mod privacy_pump_private_vault {
    use super::*;

    pub fn initialize_privacy_pump_private_vault_config(
        ctx: Context<InitializePrivacyPumpPrivateVaultConfig>,
        args: InitializePrivacyPumpPrivateVaultConfigArgs,
    ) -> Result<()> {
        handle_initialize_config(ctx, args)
    }

    pub fn create_privacy_pump_private_vault(
        ctx: Context<CreatePrivacyPumpPrivateVault>,
        args: CreatePrivacyPumpPrivateVaultArgs,
    ) -> Result<()> {
        handle_create_vault(ctx, args)
    }

    pub fn create_privacy_pump_private_proposal(
        ctx: Context<CreatePrivacyPumpPrivateProposal>,
        args: CreatePrivacyPumpPrivateProposalArgs,
    ) -> Result<()> {
        handle_create_proposal(ctx, args)
    }

    pub fn record_privacy_pump_private_action(
        ctx: Context<RecordPrivacyPumpPrivateAction>,
        args: RecordPrivacyPumpPrivateActionArgs,
    ) -> Result<()> {
        handle_record_private_action(ctx, args)
    }

    pub fn execute_privacy_pump_private_proposal(
        ctx: Context<ExecutePrivacyPumpPrivateProposal>,
        args: ExecutePrivacyPumpPrivateProposalArgs,
    ) -> Result<()> {
        handle_execute_proposal(ctx, args)
    }

    pub fn execute_privacy_pump_private_protected_withdrawal(
        ctx: Context<ExecutePrivacyPumpPrivateProtectedWithdrawal>,
        args: ExecutePrivacyPumpPrivateProtectedWithdrawalArgs,
    ) -> Result<()> {
        handle_execute_protected_withdrawal(ctx, args)
    }

    pub fn execute_privacy_pump_private_threshold_proposal(
        ctx: Context<ExecutePrivacyPumpPrivateThresholdProposal>,
        args: ExecutePrivacyPumpPrivateThresholdProposalArgs,
    ) -> Result<()> {
        handle_execute_threshold_proposal(ctx, args)
    }

    pub fn execute_privacy_pump_private_profile_proposal(
        ctx: Context<ExecutePrivacyPumpPrivateProfileProposal>,
        args: ExecutePrivacyPumpPrivateProfileProposalArgs,
    ) -> Result<()> {
        handle_execute_profile_proposal(ctx, args)
    }

    pub fn execute_privacy_pump_private_signer_set_change(
        ctx: Context<ExecutePrivacyPumpPrivateSignerSetChange>,
        args: ExecutePrivacyPumpPrivateSignerSetChangeArgs,
    ) -> Result<()> {
        handle_execute_signer_set_change(ctx, args)
    }
}
