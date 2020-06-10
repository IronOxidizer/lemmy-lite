/*
rustup toolchain install nightly
cargo +nightly run --release

Fastest templating options: askama, horrowshow, maud

TODO:
- Make page dynamic using css. Collapsable example: https://jsfiddle.net/gSPqX/1/

Endpoints:
mydomain.com/test/ # Post list view for /c/all
mydomain.com/test/post/51234 # Post view
mydomain.com/test/community/rust # Post list view
mydomain.com/test/post/51234/comment/1523 # Comment view
mydomain.com/test/user/anon123 # User view
mydomain.com/test/communities # Community list

Test domains:
dev.lemmy.ml
enterprise.lemmy.ml
voyager.lemmy.ml
ds9.lemmy.ml

CSS Table is ~10% faster than CSS FlexBox
https://benfrain.com/css-performance-test-flexbox-v-css-table-fight/
*/

use maud::{html, Markup};
use actix_web::{web, App, HttpServer, Result, error};
use actix_web::client::Client;
use serde::Deserialize;

mod templates;
mod lemmy_api;

use crate::templates::{redirect_page, post_list_page, post_page, comment_page, communities_page};
use crate::lemmy_api::{get_post_list, get_post, get_community_list, get_community};

#[derive(Deserialize)]
struct RedirForm {
    i: Option<String>,
}

#[derive(Deserialize)]
struct ListParams {
    s: Option<String>, // Sort
    p: Option<i32> // Page
    // Page size?
    // Enable preview? API expensive, multiple API calls per page
}

async fn index(web::Query(query): web::Query<RedirForm>) -> Result<Markup> {
    Ok(redirect_page(
        query.i.ok_or(error::ErrorExpectationFailed("i parameter missing. Is Nginx running?"))?
    ))
}

async fn lvl0(path: web::Path<String>, query: web::Query<ListParams>) -> Result<Markup> {
    let inst = &path.to_string();
    let client = &Client::default();

    let post_list = get_post_list(client, inst, None).await?;
    Ok(post_list_page(inst, post_list))
}

async fn lvl1(path: web::Path<(String, String)>) -> Result<Markup> {
    let inst = &path.0.to_string();
    let command = &path.1.to_string();
    let client = &Client::default();

    if command == "communities" {
        let communities = get_community_list(client, inst).await?;
        Ok(communities_page(inst, communities))
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

async fn lvl2(path: web::Path<(String, String, String)>) -> Result<Markup> {
    let inst = &path.0.to_string();
    let command = &path.1.to_string();
    let id =  &path.2.to_string();
    let client = &Client::default();

    if command == "post" {
        let post_detail = get_post(client, inst, id).await?;
        Ok(post_page(inst, post_detail))
    } else if command == "c" {
        let communities = get_community(client, inst, id).await?;

        let post_list = get_post_list(client, inst, Some(&communities.community.id)).await?;
        Ok(post_list_page(inst, post_list))
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

async fn lvl3(path: web::Path<(String, String, String, String)>) -> Result<Markup> {
    Ok(html!{})
}

async fn lvl4(path: web::Path<(String, String, String, String, String)>) -> Result<Markup> {
    let inst = &path.0.to_string();
    let command = &path.1.to_string();
    let id =  &path.2.to_string();
    let sub_command = &path.3.to_string();
    let sub_id =  &path.4.to_string();
    let client = &Client::default();

    if command == "post" && sub_command == "comment" {
        let post_detail = get_post(client, inst, id).await?;
        Ok(comment_page(inst, sub_id, post_detail))
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new()
        .service(
            web::resource("/").route(web::get().to(index))
        ).route(
            "/{lvl0}", web::get().to(lvl0)
        ).route(
            "/{lvl0}/{lvl1}", web::get().to(lvl1)
        ).route(
            "/{lvl0}/{lvl1}/{lvl2}", web::get().to(lvl2)
        ).route(
            "/{lvl0}/{lvl1}/{lvl2}/{lvl3}", web::get().to(lvl3)
        ).route(
            "/{lvl0}/{lvl1}/{lvl2}/{lvl3}/{lvl4}", web::get().to(lvl4)
        )
    })
    .bind("127.0.0.1:1131")?
    .run().await
}