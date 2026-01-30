use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Todo #{0} not found")]
    NotFound(u64),

    #[error("Todo #{0} is already done")]
    AlreadyDone(u64),
}
