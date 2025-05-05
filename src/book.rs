use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::State;

// 创建一个全局计数器，用于生成唯一的图书ID
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,
    pub title: String,
    pub author: String,
    pub price: f64,
    pub description: Option<String>,
    pub category: Option<String>,
    pub image: Option<String>,
}

// 使用Mutex创建一个线程安全的数据存储
pub type BookStore = Mutex<HashMap<usize, Book>>;

#[post("/books", format = "json", data = "<book>")]
pub async fn add_book(book_store: &State<BookStore>, book: Json<Book>) -> Result<Json<Book>, Status> {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    
    let mut book_inner = book.into_inner();
    book_inner.id = Some(id);
    
    // 设置默认图片如果没有提供
    if book_inner.image.is_none() {
        book_inner.image = Some("/assets/images/inner.jpg".to_string());
    }
    
    let mut store = book_store.lock().unwrap();
    store.insert(id, book_inner.clone());
    
    Ok(Json(book_inner))
}

#[get("/books")]
pub async fn get_books(book_store: &State<BookStore>) -> Json<Vec<Book>> {
    let store = book_store.lock().unwrap();
    
    // 如果数据库为空，添加几本示例书籍
    if store.is_empty() {
        // 注意：这里不执行添加操作，因为锁已经被获取，避免死锁
        // 直接返回示例数据
        return Json(vec![
            Book {
                id: Some(1),
                title: "Rust编程语言".to_string(),
                author: "Steve Klabnik".to_string(),
                price: 45.0,
                description: Some("Rust编程语言的官方指南".to_string()),
                category: Some("编程".to_string()),
                image: Some("/assets/images/inner.jpg".to_string()),
            },
            Book {
                id: Some(2),
                title: "深入理解计算机系统".to_string(),
                author: "Randal E. Bryant".to_string(),
                price: 99.0,
                description: Some("计算机系统基础知识教程".to_string()),
                category: Some("计算机科学".to_string()),
                image: Some("/assets/images/inner.jpg".to_string()),
            },
        ]);
    }
    
    let books: Vec<Book> = store.values().cloned().collect();
    Json(books)
}

#[get("/books/<id>")]
pub async fn get_book_by_id(book_store: &State<BookStore>, id: usize) -> Result<Json<Book>, Status> {
    let store = book_store.lock().unwrap();
    match store.get(&id) {
        Some(book) => Ok(Json(book.clone())),
        None => Err(Status::NotFound)
    }
}

#[put("/books/<id>", format = "json", data = "<book>")]
pub async fn update_book(book_store: &State<BookStore>, id: usize, book: Json<Book>) -> Result<Json<Book>, Status> {
    let mut store = book_store.lock().unwrap();
    
    if !store.contains_key(&id) {
        return Err(Status::NotFound);
    }
    
    let mut updated_book = book.into_inner();
    updated_book.id = Some(id);
    
    store.insert(id, updated_book.clone());
    Ok(Json(updated_book))
}

#[delete("/books/<id>")]
pub async fn delete_book(book_store: &State<BookStore>, id: usize) -> Status {
    let mut store = book_store.lock().unwrap();
    
    if store.remove(&id).is_some() {
        Status::NoContent
    } else {
        Status::NotFound
    }
}
