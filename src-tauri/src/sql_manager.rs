use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Ok, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub struct SqlManager {
    pub conn: Arc<Mutex<Connection>>,
}

#[derive(Serialize, Deserialize)]
pub struct SqlTodo {
    pub uuid: String,
    pub title: String,
    pub checked: bool,
    pub deadline: String,
}

impl SqlManager {
    pub fn new() -> Result<Self> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            "CREATE TABLE todo (
                uuid TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                checked BOOLEAN NOT NULL,
                deadline TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn add_test_todos(&self) -> Result<()> {
        let conn = self.conn.try_lock();
        if let std::result::Result::Ok(conn) = conn {
            conn.execute(
                "INSERT INTO todo (uuid, title, checked, deadline) VALUES
                ('1', 'Test1 From SQL', 0, '2025/01/01'),
                ('2', 'Test2 From Rust', 0, '2025/01/01'),
                ('3', 'Test3 From Backend', 1, '2025/01/01')",
                [],
            )?;
        } else {
            return Err(anyhow!("Failed to lock connection"));
        }

        Ok(())
    }

    pub fn get_todos(&self) -> Result<Vec<SqlTodo>> {
        let conn = self.conn.try_lock();

        if let std::result::Result::Err(e) = &conn {
            return Err(anyhow!("Failed to lock connection: {}", e));
        }

        let conn = conn.unwrap();

        let mut stmt = conn.prepare("SELECT uuid, title, checked, deadline FROM todo")?;

        let todos = stmt.query_map([], |row| {
            std::result::Result::Ok(SqlTodo {
                uuid: row.get(0)?,
                title: row.get(1)?,
                checked: row.get(2)?,
                deadline: row.get(3)?,
            })
        })?;

        let todos_vec: Vec<SqlTodo> = todos.filter_map(|x| x.ok()).collect();

        Ok(todos_vec)
    }
}
