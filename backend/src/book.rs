use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub price: f32,
}

pub struct AppState {
    pub conn: Mutex<Connection>,
}

impl AppState {
    pub fn new() -> Self {
        let conn = Connection::open("books.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                price REAL NOT NULL
            )",
            [],
        ).unwrap();
        Self {
            conn: Mutex::new(conn),
        }
    }

    pub fn get_books(&self) -> Result<Vec<Book>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, author, price FROM books")?;
        let books = stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                price: row.get(3)?,
            })
        })?;
        books.collect()
    }

    pub fn create_book(&self, book: &Book) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO books (title, author, price) VALUES (?1, ?2, ?3)",
            (&book.title, &book.author, &book.price),
        )?;
        Ok(())
    }
} 