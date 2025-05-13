use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Erro de banco de dados: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Erro de I/O: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Erro ao fazer parse: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("URL não encontrada")]
    UrlNotFound,
}
