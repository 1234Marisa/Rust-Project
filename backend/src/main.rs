#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use crate::book::{Book, AppState};

mod book;

#[get("/books")]
fn get_books(state: &rocket::State<AppState>) -> Json<Vec<Book>> {
    Json(state.get_books().unwrap())
}

#[post("/books", data = "<book>")]
fn create_book(state: &rocket::State<AppState>, book: Json<Book>) -> Json<Book> {
    state.create_book(&book).unwrap();
    book
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppState::new())
        .mount("/api", routes![get_books, create_book])
} 