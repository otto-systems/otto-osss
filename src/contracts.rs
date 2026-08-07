use crate::{
    CreateSessionCommand,
    DeleteVaultEntryCommand,
    EmitAuditEventCommand,
    OsssError,
    ReadVaultEntryCommand,
    RevokeSessionCommand,
    SessionToken,
    ValidateSessionCommand,
    WriteVaultEntryCommand,
};

pub trait VaultContract {
    fn read_entry(&self, cmd: ReadVaultEntryCommand) -> Result<Vec<u8>, OsssError>;
    fn write_entry(&self, cmd: WriteVaultEntryCommand) -> Result<(), OsssError>;
    fn delete_entry(&self, cmd: DeleteVaultEntryCommand) -> Result<(), OsssError>;
}

pub trait SessionContract {
    fn create_session(&self, cmd: CreateSessionCommand) -> Result<SessionToken, OsssError>;
    fn validate_session(&self, cmd: ValidateSessionCommand) -> Result<bool, OsssError>;
    fn revoke_session(&self, cmd: RevokeSessionCommand) -> Result<(), OsssError>;
}

pub trait AuditContract {
    fn emit_audit_event(&self, cmd: EmitAuditEventCommand) -> Result<(), OsssError>;
    fn query_audit_log(&self, filter: &str) -> Result<Vec<String>, OsssError>;
}
