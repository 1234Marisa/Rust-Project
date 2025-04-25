use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub price: f32,
    pub description: String,
    pub image_url: String,
    pub category: String,
    pub condition: String,
    pub isbn: String,
}

pub struct AppState {
    pub conn: Mutex<Connection>,
}

impl AppState {
    pub fn new() -> Self {
        let conn = Connection::open("books.db").unwrap();
        // Drop the existing table to update the schema
        conn.execute("DROP TABLE IF EXISTS books", []).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL,
                price REAL NOT NULL,
                description TEXT NOT NULL,
                image_url TEXT NOT NULL,
                category TEXT NOT NULL,
                condition TEXT NOT NULL,
                isbn TEXT NOT NULL
            )",
            [],
        ).unwrap();
        Self {
            conn: Mutex::new(conn),
        }
    }

    pub fn get_books(&self) -> Result<Vec<Book>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, author, price, description, image_url, category, condition, isbn FROM books"
        )?;
        let books = stmt.query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                price: row.get(3)?,
                description: row.get(4)?,
                image_url: row.get(5)?,
                category: row.get(6)?,
                condition: row.get(7)?,
                isbn: row.get(8)?,
            })
        })?;
        books.collect()
    }

    pub fn create_book(&self, book: &Book) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO books (title, author, price, description, image_url, category, condition, isbn) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (
                &book.title,
                &book.author,
                &book.price,
                &book.description,
                &book.image_url,
                &book.category,
                &book.condition,
                &book.isbn,
            ),
        )?;
        Ok(())
    }
} 