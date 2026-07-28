use anchor_lang::prelude::*;

#[error_code]
pub enum PrivacyPumpPrivateVaultError {
    #[msg("The requested threshold is invalid for the private signer set.")]
    InvalidThreshold,
    #[msg("The signer commitment root is required.")]
    InvalidSignerCommitmentRoot,
    #[msg("The caller is not the configured PrivacyPump Factory.")]
    InvalidFactory,
    #[msg("The caller is not the configured PrivacyPump Relayer.")]
    InvalidRelayer,
    #[msg("The proposal is not pending.")]
    ProposalNotPending,
    #[msg("The proposal is not approved for execution.")]
    ProposalNotApproved,
    #[msg("This private signer action was already recorded.")]
    DuplicatePrivateAction,
    #[msg("The private nullifier is required.")]
    InvalidPrivateNullifier,
    #[msg("The private authorization action domain is invalid.")]
    InvalidActionDomain,
    #[msg("The Arcium verifier account is not configured for this vault.")]
    InvalidArciumVerifier,
    #[msg("Arcium private membership and nullifier verification is not available.")]
    ArciumVerificationUnavailable,
    #[msg("The PrivacyPumpArciumVerifier receipt is invalid.")]
    InvalidArciumReceipt,
    #[msg("The PrivacyPumpArciumVerifier receipt is owned by the wrong program.")]
    InvalidArciumReceiptOwner,
    #[msg("The PrivacyPumpArciumVerifier receipt is not verified.")]
    ArciumReceiptNotVerified,
    #[msg("The PrivacyPumpArciumVerifier receipt is already consumed.")]
    ArciumReceiptAlreadyConsumed,
    #[msg("The PrivacyPumpArciumVerifier receipt vault binding is invalid.")]
    ArciumReceiptVaultMismatch,
    #[msg("The PrivacyPumpArciumVerifier receipt proposal binding is invalid.")]
    ArciumReceiptProposalMismatch,
    #[msg("The PrivacyPumpArciumVerifier receipt signer root binding is invalid.")]
    ArciumReceiptSignerRootMismatch,
    #[msg("The PrivacyPumpArciumVerifier receipt signer set version binding is invalid.")]
    ArciumReceiptSignerSetVersionMismatch,
    #[msg("The PrivacyPumpArciumVerifier receipt action domain binding is invalid.")]
    ArciumReceiptActionDomainMismatch,
    #[msg("The PrivacyPumpArciumVerifier receipt nullifier binding is invalid.")]
    ArciumReceiptNullifierMismatch,
    #[msg("The PrivacyPumpArciumVerifier receipt payload binding is invalid.")]
    ArciumReceiptPayloadMismatch,
    #[msg("The private proposal execution payload is invalid.")]
    InvalidExecutionPayload,
    #[msg("The private vault treasury account is invalid.")]
    InvalidTreasury,
    #[msg("The private vault withdrawal recipient is invalid.")]
    InvalidRecipient,
    #[msg("The private vault treasury has insufficient lamports.")]
    TreasuryInsufficientFunds,
    #[msg("The private vault treasury rent reserve cannot be drained.")]
    TreasuryRentReserve,
    #[msg("The private vault lamport transfer overflowed.")]
    LamportMathOverflow,
    #[msg("The private vault relayer reimbursement wallet is invalid.")]
    InvalidReimbursementWallet,
    #[msg("The private action fee exceeded the safety cap.")]
    RelayerReimbursementExceedsCap,
    #[msg("The withdrawal is below the proposal recipient minimum.")]
    WithdrawalBelowMinimumRecipientReceive,
    #[msg("This proposal was created before fee protection was added. Create a new proposal.")]
    LegacyProposalMissingFeeProtection,
}
