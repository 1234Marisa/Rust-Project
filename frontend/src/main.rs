use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
//use wasm_bindgen_futures::spawn_local;

// Add console logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/add")]
    AddBook,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Book {
    id: Option<i32>,
    title: String,
    author: String,
    price: f32,
    description: String,
    image_url: String,
    category: String,
    condition: String,
    isbn: String,
}

#[function_component(Home)]
fn home() -> Html {
    let books = use_state(Vec::new);
    let error = use_state(|| None::<String>);

    {
        let books = books.clone();
        let error = error.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                console_log!("Fetching books...");
                match reqwest::get("http://localhost:8000/api/books").await {
                    Ok(response) => {
                        match response.json::<Vec<Book>>().await {
                            Ok(fetched_books) => {
                                console_log!("Fetched {} books", fetched_books.len());
                                books.set(fetched_books);
                                error.set(None);
                            }
                            Err(e) => {
                                console_log!("Error parsing books: {:?}", e);
                                error.set(Some(format!("Error parsing books: {:?}", e)));
                            }
                        }
                    }
                    Err(e) => {
                        console_log!("Error fetching books: {:?}", e);
                        error.set(Some(format!("Error fetching books: {:?}", e)));
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="container">
            <h1>{"Campus Book Exchange"}</h1>
            <div class="nav-buttons">
                <Link<Route> to={Route::AddBook} classes="add-book-btn">
                    {"Add New Book"}
                </Link<Route>>
            </div>
            if let Some(err) = (*error).clone() {
                <div class="error-message">
                    {err}
                </div>
            }
            <div class="book-grid">
                {for (*books).iter().map(|book| {
                    html! {
                        <div class="book-card">
                            <div class="book-image">
                                <img src={book.image_url.clone()} alt={book.title.clone()} />
                            </div>
                            <div class="book-info">
                                <h3>{&book.title}</h3>
                                <p class="author">{format!("By {}", book.author)}</p>
                                <p class="price">{format!("${:.2}", book.price)}</p>
                                <p class="category">{&book.category}</p>
                                <p class="condition">{format!("Condition: {}", book.condition)}</p>
                                <div class="description">
                                    <p>{&book.description}</p>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

#[function_component(AddBook)]
fn add_book() -> Html {
    let new_book = use_state(|| Book {
        id: None,
        title: String::new(),
        author: String::new(),
        price: 0.0,
        description: String::new(),
        image_url: String::new(),
        category: String::new(),
        condition: String::new(),
        isbn: String::new(),
    });
    let error = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let new_book = new_book.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let book = (*new_book).clone();
            let navigator = navigator.clone();
            let error = error.clone();
            
            console_log!("Submitting book: {:?}", book);
            wasm_bindgen_futures::spawn_local(async move {
                match reqwest::Client::new()
                    .post("http://localhost:8000/api/books")
                    .json(&book)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            console_log!("Book added successfully");
                            navigator.push(&Route::Home);
                        } else {
                            let err_msg = format!("Server error: {}", response.status());
                            console_log!("{}", err_msg);
                            error.set(Some(err_msg));
                        }
                    }
                    Err(e) => {
                        let err_msg = format!("Error adding book: {:?}", e);
                        console_log!("{}", err_msg);
                        error.set(Some(err_msg));
                    }
                }
            });
        })
    };

    let update = |field: &'static str| {
        let new_book = new_book.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut book = (*new_book).clone();
            match field {
                "title" => book.title = input.value(),
                "author" => book.author = input.value(),
                "price" => book.price = input.value().parse().unwrap_or(0.0),
                "description" => book.description = input.value(),
                "image_url" => book.image_url = input.value(),
                "category" => book.category = input.value(),
                "condition" => book.condition = input.value(),
                "isbn" => book.isbn = input.value(),
                _ => {}
            }
            new_book.set(book);
        })
    };

    html! {
        <div class="container">
            <h1>{"Add New Book"}</h1>
            if let Some(err) = (*error).clone() {
                <div class="error-message">
                    {err}
                </div>
            }
            <form class="book-form" onsubmit={onsubmit}>
                <div class="form-group">
                    <input
                        type="text"
                        placeholder="Title"
                        value={new_book.title.clone()}
                        onchange={update("title")}
                        required=true
                    />
                </div>
                <div class="form-group">
                    <input
                        type="text"
                        placeholder="Author"
                        value={new_book.author.clone()}
                        onchange={update("author")}
                        required=true
                    />
                </div>
                <div class="form-group">
                    <input
                        type="number"
                        step="0.01"
                        placeholder="Price"
                        value={new_book.price.to_string()}
                        onchange={update("price")}
                        required=true
                    />
                </div>
                <div class="form-group">
                    <textarea
                        placeholder="Description"
                        value={new_book.description.clone()}
                        onchange={update("description")}
                        required=true
                    />
                </div>
                <div class="form-group">
                    <input
                        type="url"
                        placeholder="Image URL"
                        value={new_book.image_url.clone()}
                        onchange={update("image_url")}
                        required=true
                    />
                </div>
                <div class="form-group">
                    <input
                        type="text"
                        placeholder="Category"
                        value={new_book.category.clone()}
                        onchange={update("category")}
                        required=true
                    />
                </div>
                <div class="form-group">
                    <select onchange={update("condition")} required=true>
                        <option value="">{"Select Condition"}</option>
                        <option value="New">{"New"}</option>
                        <option value="Like New">{"Like New"}</option>
                        <option value="Good">{"Good"}</option>
                        <option value="Fair">{"Fair"}</option>
                    </select>
                </div>
                <div class="form-group">
                    <input
                        type="text"
                        placeholder="ISBN"
                        value={new_book.isbn.clone()}
                        onchange={update("isbn")}
                        required=true
                    />
                </div>
                <div class="form-buttons">
                    <button type="submit">{"Add Book"}</button>
                    <Link<Route> to={Route::Home} classes="cancel-btn">
                        {"Cancel"}
                    </Link<Route>>
                </div>
            </form>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::AddBook => html! { <AddBook /> },
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
} 