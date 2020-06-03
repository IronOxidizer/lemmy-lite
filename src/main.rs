/*
TODO:
- Setup nginx to serve index, css, favicon, logo
- Make page dynamic using css. Collapsable example: https://jsfiddle.net/gSPqX/1/
- Make client global and reuse it for better caching behavior

Endpoints:
mydomain.com/test/ # Post list view for /c/all
mydomain.com/test/post/51234 # Post view
mydomain.com/test/community/rust # Post list view
mydomain.com/test/post/51234/comment/1523 # Comment view
mydomain.com/test/user/anon123 # User view
mydomain.com/test/communities # Community list
*/

use maud::{html, Markup};
use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

mod templates;
mod lemmy_api;

use crate::templates::{root, redirect};
use crate::lemmy_api::{get_post_list};

#[derive(Deserialize)]
struct RedirForm {
    i: Option<String>,
}

#[derive(Deserialize)]
struct ListParams {
    s: Option<String>, // Sort
    p: Option<i64> // Page
}

async fn index(web::Query(query): web::Query<RedirForm>) -> Result<Markup> {
    Ok(match query.i { 
        Some(i) => {
            if i.trim().is_empty() {
                root()
            } else {
                redirect(i)
            }
        },
        None => root(),
    })
}

async fn instance(path: web::Path<String>, query: web::Query<ListParams>) -> Result<Markup> {
    // Get all
    // http://path/api/v1/post/list
    // Make a request
    // Pass params to markup

    let post_list = get_post_list().await?;

    Ok(html!{})
}

async fn lvl1(path: web::Path<(String, String)>) -> Result<Markup> {
    Ok(html!{})
}

async fn lvl2(path: web::Path<(String, String, String)>) -> Result<Markup> {
    Ok(html!{})
}

async fn lvl3(path: web::Path<(String, String, String, String)>) -> Result<Markup> {
    Ok(html!{})
}

async fn lvl4(path: web::Path<(String, String, String, String, String)>) -> Result<Markup> {
    Ok(html!{})
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
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
    .run().await
}