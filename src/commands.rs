

use clap::{Parser, Subcommand};
use crate::db::Db;
use crate::errors::AppError;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Console task manager!")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add {
        title: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    List,
    Done {
        id: String,
    },
    Delete {
        id: String,
    },
    ToJson
}

impl Command {
    pub fn execute(self, db: &Db) -> Result<(), AppError> {
        match self {
            Command::Add { title, description } => {
                let id = db.add_task(&title, description.as_deref())?;
                println!("Задача добавлена [{}]", id);
            }

            Command::List => {
                let tasks = db.get_tasks()?;

                if tasks.is_empty() {
                    println!("Задач пока нет.");
                    return Ok(());
                }

                for task in &tasks {
                    let desc = task.description
                        .as_deref()
                        .unwrap_or("—");

                    println!(
                        "[{}] {} | {} | {}",
                        task.status, task.id, task.title, desc
                    );
                }

                println!("\nВсего: {}", tasks.len());
            }

            Command::Done { id } => {
                db.update_status(&id, "Complete")?;
                println!("Задача {} отмечена как выполненная", id);
            }

            Command::Delete { id } => {
                db.delete_task(&id)?;
                println!("Задача {} удалена", id);
            }

            Command::ToJson {} => {
                
            }
        }

        Ok(())
    }
}
