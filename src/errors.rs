use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Не удалось создать директорию: {0}")]
    DirCreate(#[from] std::io::Error),

    #[error("Не удалось определить путь к базе данных")]
    InvalidPath,

    #[error("Ошибка базы данных: {0}")]
    Sql(#[from] rusqlite::Error),

    #[error("Задача с id '{0}' не найдена")]
    NotFound(String),
}

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum CliError {
    #[error("Название задачи не может быть пустым")]
    EmptyTitle,

    #[error("Некорректный ID '{0}' — ожидается 8 символов (a-z, 0-9)")]
    InvalidId(String),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Db(#[from] DbError),

    #[error(transparent)]
    Cli(#[from] CliError),
}
