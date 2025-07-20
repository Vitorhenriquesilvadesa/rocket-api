use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Password hash generation error: {0}")]
    PasswordHashError(String),

    #[error("User not found")]
    UserNotFound,

    #[error("Repository error: {0}")]
    RepositoryError(String),

    #[error("Unknown error")]
    Unknown,
}
