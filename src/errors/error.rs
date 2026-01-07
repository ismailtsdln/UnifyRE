use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnifyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Binary parsing error: {0}")]
    ParseError(String),

    #[error("Unsupported architecture: {0}")]
    UnsupportedArch(String),

    #[allow(dead_code)]
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Disassembly error: {0}")]
    DisasmError(String),

    #[allow(dead_code)]
    #[error("Debugger error: {0}")]
    DebuggerError(String),

    #[allow(dead_code)]
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[allow(dead_code)]
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
}

pub type Result<T> = std::result::Result<T, UnifyError>;
