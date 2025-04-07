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

#[launch]
fn rocket() -> _ {
    rocket::build()
        // 提供静态文件：挂载 `/assets` 路径用于提供静态文件
        .mount("/assets", FileServer::from(relative!("/assets")))
        // 渲染HTML页面：将根路径（/）指向 index.html
        .mount("/", routes![index])
}

#[get("/")]
fn index() -> rocket::response::content::RawHtml<String> {
    // 返回动态的 HTML 页面
    let html_content = include_str!("../assets/index.html");
    rocket::response::content::RawHtml(html_content.to_string())
}
