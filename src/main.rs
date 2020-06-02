#![feature(proc_macro_hygiene)]

use maud::{html, Markup};
use actix_web::{web, App, HttpServer};

fn index() -> Markup {
    html! {
        "Hello, World!"
    }
}

fn user(params: web::Path<(String, u32)>) -> Markup {
    html! {
        h1 { "Hello " (params.0) " with id " (params.1) "!" }
    }
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user/{name}/{id}", web::get().to(user))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:80")?
    .run()
}