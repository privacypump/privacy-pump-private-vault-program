use anchor_lang::prelude::*;

pub const PRIVATE_VAULT_SEED: &[u8] = b"pp-private-vault";
pub const PRIVATE_TREASURY_SEED: &[u8] = b"pp-private-treasury";
pub const PRIVATE_PROPOSAL_SEED: &[u8] = b"pp-private-proposal";
pub const PRIVATE_NULLIFIER_SEED: &[u8] = b"pp-private-nullifier";
pub const PRIVATE_CONFIG_SEED: &[u8] = b"pp-private-vault-config";

pub const ACTION_DOMAIN_APPROVE: u8 = 1;
pub const ACTION_DOMAIN_REJECT: u8 = 2;

pub const ZK_POOL_SEED: &[u8] = b"private-pool";
pub const ZK_POOL_VAULT_SEED: &[u8] = b"private-pool-vault";
pub const PRIVACY_PUMP_ZK_POOL_PROGRAM_ID: Pubkey =
    pubkey!("FiDC7T29hYYdaJvD8WqxkHiXTWFAtmqdDuZ5BJ54admT");
pub const RECORD_FUNDED_NOTE_DISCRIMINATOR: [u8; 8] = [134, 107, 164, 138, 88, 209, 205, 164];
