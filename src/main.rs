use maud::{html, Markup};
use actix_web::{web, App, HttpServer};
use serde::Deserialize;

mod markup;
use crate::markup::{root, redirect};

/*
Endpoints:
mydomain.com/test/ # Post list view for /c/all
mydomain.com/test/post/51234 # Post view
mydomain.com/test/community/rust # Post list view
mydomain.com/test/post/51234/comment/1523 # Comment view
mydomain.com/test/user/anon123 # User view
mydomain.com/test/communities # Community list
*/

#[derive(Deserialize)]
struct RedirForm {
    i: Option<String>,
}

#[derive(Deserialize)]
struct ListParams {
    s: Option<String>, // Sort
    p: Option<i64> // Page
}

fn index(web::Query(query): web::Query<RedirForm>) -> Markup {
    match query.i {
        Some(i) => {
            if i.trim().is_empty() {
                root()
            } else {
                redirect(i)
            }
        },
        None => root(),
    }
}

fn instance(path: web::Path<String>, query: web::Query<ListParams>) -> Markup {
    // Get all
    // http://path/api/v1/post/list
    // Make a request
    // Pass params to markup

    html! {
        h1 { "Hello " (path)  "!" }
    }
}

fn lvl1(path: web::Path<(String, String)>) -> Markup {
    html! {
        h1 { "Hello " (path.0)  " 2!" }
    }
}

fn lvl2(path: web::Path<(String, String, String)>) -> Markup {
    html! {
        h1 { "Hello " (path.0)  " 3!" }
    }
}

fn lvl3(path: web::Path<(String, String, String, String)>) -> Markup {
    html! {
        h1 { "Hello " (path.0)  " 4!" }
    }
}

fn lvl4(path: web::Path<(String, String, String, String, String)>) -> Markup {
    html! {
        h1 { "Hello " (path.0)  " 5!" }
    }
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new()
        .service(
            web::resource("/").route(web::get().to(index))
        ).route(
            "/{instance}", web::get().to(instance)
        ).route(
            "/{instance}/{lvl1}", web::get().to(lvl1)
        ).route(
            "/{instance}/{lvl1}/{lvl2}", web::get().to(lvl2)
        ).route(
            "/{instance}/{lvl1}/{lvl2}/{lvl3}", web::get().to(lvl3)
        ).route(
            "/{instance}/{lvl1}/{lvl2}/{lvl3}/{lvl4}", web::get().to(lvl4)
        )
    })
    .bind("127.0.0.1:80")?
    .run()
}