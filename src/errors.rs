use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorldCompilerError {
    #[error("Schema error: {0}")]
    SchemaError(String),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_yaml::Error),

    #[error("Generation error: {0}")]
    GenerationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, WorldCompilerError>;
