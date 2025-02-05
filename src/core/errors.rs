use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DelegateNotFound(u64),
    TransactionError(String),
    NoDelegates,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DelegateNotFound(id) => write!(f, "Делегат с id {} не найден", id),
            AppError::TransactionError(msg) => write!(f, "Ошибка транзакции: {}", msg),
            AppError::NoDelegates => write!(f, "Нет делегатов"),
        }
    }
}

impl std::error::Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
