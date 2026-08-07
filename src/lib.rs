pub mod commands;
pub mod contracts;

use thiserror::Error;

pub type VaultId = String;
pub type EntryKey = String;
pub type SessionToken = String;
pub type AccessToken = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadVaultEntryCommand {
    pub vault_id: VaultId,
    pub entry_key: EntryKey,
    pub access_token: AccessToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteVaultEntryCommand {
    pub vault_id: VaultId,
    pub entry_key: EntryKey,
    pub sealed_entry: Vec<u8>,
    pub access_token: AccessToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteVaultEntryCommand {
    pub vault_id: VaultId,
    pub entry_key: EntryKey,
    pub access_token: AccessToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSessionCommand {
    pub identity: String,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSessionCommand {
    pub session_token: SessionToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeSessionCommand {
    pub session_token: SessionToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitAuditEventCommand {
    pub actor: String,
    pub action: String,
    pub outcome: String,
    pub details: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OsssError {
    #[error("vault not found")]
    VaultNotFound,
    #[error("entry not found")]
    EntryNotFound,
    #[error("access denied")]
    AccessDenied,
    #[error("invalid session")]
    InvalidSession,
    #[error("invalid input")]
    InvalidInput,
    #[error("operation failed")]
    OperationFailed,
}
