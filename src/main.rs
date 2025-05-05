// #[macro_use] extern crate rocket;
//
// use rocket::response::content::RawHtml;
//
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }
//
// #[get("/")]
// fn index() -> RawHtml<String> {
//     let html_content = include_str!("../assets/index.html");
//     RawHtml(html_content.to_string())  // 使用 RawHtml 类型返回 HTML 内容
// }

#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use std::collections::HashMap;
use std::sync::Mutex;
mod book;
mod user;
use book::{add_book, get_books, get_book_by_id, update_book, delete_book, Book};
use user::{register_user, login, get_user, init_users, User};

#[launch]
fn rocket() -> _ {
    rocket::build()
        // 提供静态文件：挂载 `/assets` 路径用于提供静态文件
        .mount("/assets", FileServer::from(relative!("/assets")))
        // 渲染HTML页面：将根路径（/）指向 index.html
        .mount("/", routes![index, login_page, cart_page, search_page])
        // 添加API路由
        .mount("/api", routes![add_book, get_books, get_book_by_id, update_book, delete_book])
        // 添加用户API路由
        .mount("/api", routes![register_user, login, get_user])
        // 添加应用程序状态 - 书籍存储
        .manage(Mutex::new(HashMap::<usize, Book>::new()))
        // 添加应用程序状态 - 用户存储
        .manage(Mutex::new(init_users()))
}

#[get("/")]
fn index() -> rocket::response::content::RawHtml<String> {
    // 返回动态的 HTML 页面
    let html_content = include_str!("../assets/index.html");
    rocket::response::content::RawHtml(html_content.to_string())
}

#[get("/login.html")]
fn login_page() -> rocket::response::content::RawHtml<String> {
    // 返回登录页面
    let html_content = include_str!("../assets/login.html");
    rocket::response::content::RawHtml(html_content.to_string())
}

#[get("/cart.html")]
fn cart_page() -> rocket::response::content::RawHtml<String> {
    // 返回购物车页面
    let html_content = include_str!("../assets/cart.html");
    rocket::response::content::RawHtml(html_content.to_string())
}

#[get("/search.html")]
fn search_page() -> rocket::response::content::RawHtml<String> {
    let html_content = include_str!("../assets/search.html");
    rocket::response::content::RawHtml(html_content.to_string())
}
