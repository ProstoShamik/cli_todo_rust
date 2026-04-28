use directories::ProjectDirs;
use std::path::PathBuf;
use crate::errors::DbError;

pub fn db_path() -> Result<PathBuf, DbError> {
    match ProjectDirs::from("com", "eshkereeee", "todo") {
        Some(proj_dirs)  => {
            Ok(proj_dirs.data_dir().join("tasks.db"))
        },
        None => {
            Err(DbError::InvalidPath)
        }
    }
}