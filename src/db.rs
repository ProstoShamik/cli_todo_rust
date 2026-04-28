

use rusqlite::{Connection, params};
use nanoid::nanoid;
use chrono::Utc;
use std::fs;

use crate::config::db_path;
use crate::errors::DbError;
use crate::models::{Status, Task};


const ALPHABET: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9',
];

fn gen_id() -> String {
    nanoid!(8, &ALPHABET)
}


pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Result<Self, DbError> {
        let path = db_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Db { conn })
    }


    pub fn add_task(&self, title: &str, description: Option<&str>) -> Result<String, DbError> {
        let id = gen_id();
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO tasks (id, title, description, created_at, status)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, title, description, now, "Todo"],
        )?;

        Ok(id)
    }


    pub fn get_tasks(&self) -> Result<Vec<Task>, DbError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, created_at, status FROM tasks"
        )?;

        let tasks = stmt
            .query_map([], |row| {
                let created_at_str: String = row.get(3)?;
                let created_at = created_at_str.parse().unwrap_or_else(|_| Utc::now());
                
                let status_str: String = row.get(4)?;
                let status = match status_str.as_str() {
                    "Todo" => Status::Todo,
                    "InProgress" | "In Progress" => Status::InProgress,
                    "Complete" => Status::Complete,
                    "Cancelled" => Status::Cancelled,
                    _ => Status::Todo,
                };

                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    created_at,
                    status,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tasks)
    }


    pub fn update_status(&self, id: &str, status: &str) -> Result<(), DbError> {
        let affected = self.conn.execute(
            "UPDATE tasks SET status = ?1 WHERE id = ?2",
            params![status, id],
        )?;

        if affected == 0 {
            return Err(DbError::NotFound(id.to_string()));
        }

        Ok(())
    }


    pub fn delete_task(&self, id: &str) -> Result<(), DbError> {
        let affected = self.conn.execute(
            "DELETE FROM tasks WHERE id = ?1",
            params![id],
        )?;

        if affected == 0 {
            return Err(DbError::NotFound(id.to_string()));
        }

        Ok(())
    }
}
