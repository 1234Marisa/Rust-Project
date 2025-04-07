use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub price: f64,
}

#[post("/add", format = "json", data = "<book>")]
pub async fn add_book(book: Json<Book>) -> Json<Book> {
    // 在这里保存书籍信息到数据库
    Json(book.into_inner())  // 假设成功保存后返回书籍数据
}

#[get("/books")]
pub async fn get_books() -> Json<Vec<Book>> {
    // 假设从数据库中查询所有书籍
    let books = vec![
        Book {
            title: "Book 1".to_string(),
            author: "Author 1".to_string(),
            price: 20.5,
        },
        Book {
            title: "Book 2".to_string(),
            author: "Author 2".to_string(),
            price: 15.3,
        },
    ];

    Json(books)
}
