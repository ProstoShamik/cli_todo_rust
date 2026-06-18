use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

type TaskId = String;

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Complete,
    Cancelled,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Complete => write!(f, "Complete"),
            Status::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub status: Status,
}
