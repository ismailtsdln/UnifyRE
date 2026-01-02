use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnifyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Binary parsing error: {0}")]
    ParseError(String),

    #[error("Unsupported architecture: {0}")]
    UnsupportedArch(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Disassembly error: {0}")]
    DisasmError(String),

    #[error("Debugger error: {0}")]
    DebuggerError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
}

pub type Result<T> = std::result::Result<T, UnifyError>;
