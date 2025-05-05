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
use book::{add_book, get_books, get_book_by_id, update_book, delete_book, Book};

#[launch]
fn rocket() -> _ {
    rocket::build()
        // 提供静态文件：挂载 `/assets` 路径用于提供静态文件
        .mount("/assets", FileServer::from(relative!("/assets")))
        // 渲染HTML页面：将根路径（/）指向 index.html
        .mount("/", routes![index])
        // 添加API路由
        .mount("/api", routes![add_book, get_books, get_book_by_id, update_book, delete_book])
        // 添加应用程序状态
        .manage(Mutex::new(HashMap::<usize, Book>::new()))
}

#[get("/")]
fn index() -> rocket::response::content::RawHtml<String> {
    // 返回动态的 HTML 页面
    let html_content = include_str!("../assets/index.html");
    rocket::response::content::RawHtml(html_content.to_string())
}
