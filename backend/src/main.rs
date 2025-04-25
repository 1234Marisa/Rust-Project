#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use crate::book::{Book, AppState};

mod book;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Handles all OPTIONS requests */
}

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
        .attach(CORS)
        .manage(AppState::new())
        .mount("/api", routes![get_books, create_book, all_options])
} 