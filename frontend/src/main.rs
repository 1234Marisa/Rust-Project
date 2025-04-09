use yew::prelude::*;
use serde::{Deserialize, Serialize};
//use wasm_bindgen_futures::spawn_local;
#[derive(Serialize, Deserialize, Clone)]
struct Book {
    id: Option<i32>,
    title: String,
    author: String,
    price: f32,
}

struct App {
    books: Vec<Book>,
    new_book: Book,
}

enum Msg {
    FetchBooks,
    BooksFetched(Vec<Book>),
    UpdateTitle(String),
    UpdateAuthor(String),
    UpdatePrice(String),
    AddBook,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchBooks);
        Self {
            books: Vec::new(),
            new_book: Book {
                id: None,
                title: String::new(),
                author: String::new(),
                price: 0.0,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchBooks => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_books: Vec<Book> = reqwest::get("http://localhost:8000/api/books")
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    link.send_message(Msg::BooksFetched(fetched_books));
                });
                false
            }
            Msg::BooksFetched(books) => {
                self.books = books;
                true
            }
            Msg::UpdateTitle(title) => {
                self.new_book.title = title;
                true
            }
            Msg::UpdateAuthor(author) => {
                self.new_book.author = author;
                true
            }
            Msg::UpdatePrice(price) => {
                self.new_book.price = price.parse().unwrap_or(0.0);
                true
            }
            Msg::AddBook => {
                let book = self.new_book.clone();
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = reqwest::Client::new()
                        .post("http://localhost:8000/api/books")
                        .json(&book)
                        .send()
                        .await;
                    link.send_message(Msg::FetchBooks);
                });
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <h1>{"Campus Book Exchange"}</h1>
                
                <div class="book-form">
                    <input
                        type="text"
                        placeholder="Title"
                        value={self.new_book.title.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateTitle(input.value())
                        })}
                    />
                    <input
                        type="text"
                        placeholder="Author"
                        value={self.new_book.author.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateAuthor(input.value())
                        })}
                    />
                    <input
                        type="number"
                        placeholder="Price"
                        value={self.new_book.price.to_string()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdatePrice(input.value())
                        })}
                    />
                    <button onclick={ctx.link().callback(|_| Msg::AddBook)}>
                        {"Add Book"}
                    </button>
                </div>

                <div class="book-list">
                    {for self.books.iter().map(|book| {
                        html! {
                            <div class="book-item">
                                <h3>{&book.title}</h3>
                                <p>{format!("Author: {}", book.author)}</p>
                                <p>{format!("Price: ${:.2}", book.price)}</p>
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
} 