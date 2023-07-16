/*
rustup toolchain install nightly
cargo +nightly run --release

Test instances:
dev.lemmy.ml
enterprise.lemmy.ml
voyager.lemmy.ml
ds9.lemmy.ml
*/

use actix_web::{
    client::Client, error, http::StatusCode, web, App, HttpResponse, HttpServer, Result,
};
use chrono::offset::Utc;
use maud::Markup;
use serde::Deserialize;
mod lemmy_api;
mod templates;

use crate::lemmy_api::{
    get_community, get_community_list, get_post, get_post_list, get_user, search, PagingParams,
    SearchParams,
};
use crate::templates::{
    comment_page, communities_page, community_info_page, post_list_page, post_page, search_page,
    user_page,
};

#[derive(Deserialize)]
struct PathParams2 {
    inst: String,
    command: String,
}
#[derive(Deserialize)]
struct PathParams3 {
    inst: String,
    command: String,
    id: String,
}

#[derive(Deserialize)]
struct PathParams4 {
    inst: String,
    command: String,
    id: String,
    sub_command: String,
}

#[derive(Deserialize)]
struct PathParams5 {
    inst: String,
    command: String,
    id: String,
    sub_command: String,
    sub_id: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Client::default())
            .route("/i/{inst}", web::get().to(lvl1))
            .route("/i/{inst}/{command}", web::get().to(lvl2))
            .route("/i/{inst}/{command}/{id}", web::get().to(lvl3))
            .route(
                "/i/{inst}/{command}/{id}/{sub_command}",
                web::get().to(lvl4),
            )
            .route(
                "/{inst}/{command}/{id}/{sub_command}/{sub_id}",
                web::get().to(lvl5),
            )
            .service(actix_files::Files::new("/", "./static").show_files_listing())
    })
    .bind("127.0.0.1:1131")?
    .run()
    .await
}

async fn lvl1(
    path: web::Path<String>,
    query: web::Query<PagingParams>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let inst = &path.to_string();
    let client = &data_client.into_inner();

    let now = &Utc::now().naive_utc();
    let paging_params = &query.into_inner();

    let post_list = get_post_list(client, inst).await?;

    html_res(post_list_page(
        inst,
        post_list,
        now,
        None,
        Some(paging_params),
    ))
}

async fn lvl2(
    p: web::Path<PathParams2>,
    query: web::Query<SearchParams>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let client = &data_client.into_inner();
    let search_params = &query.into_inner();

    if p.command == "communities" {
        let paging_params = &PagingParams {
            s: search_params.s.clone().or(Some("TopAll".to_string())),
            p: search_params.p,
            l: search_params.l,
        };
        let communities = get_community_list(client, &p.inst).await?;
        html_res(communities_page(&p.inst, communities, Some(paging_params)))
    } else if p.command == "search" {
        let now = &Utc::now().naive_utc();
        let search_res = match search_params.q {
            Some(ref query) if !query.is_empty() => {
                Some(search(client, &p.inst, search_params).await?)
            }
            _ => None,
        };

        html_res(search_page(&p.inst, now, search_res, search_params))
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

async fn lvl3(
    p: web::Path<PathParams3>,
    query: web::Query<PagingParams>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let client = &data_client.into_inner();
    let now = &Utc::now().naive_utc();
    let paging_params = &query.into_inner();

    if p.command == "post" {
        let post_detail = get_post(client, &p.inst, &p.id).await?;
        html_res(post_page(&p.inst, post_detail, now))
    } else if p.command == "c" {
        let post_list = get_post_list(client, &p.inst).await?;
        html_res(post_list_page(
            &p.inst,
            post_list,
            now,
            Some(&p.id),
            Some(paging_params),
        ))
    } else if p.command == "u" {
        let user = get_user(client, &p.inst, &p.id).await?;
        html_res(user_page(&p.inst, user, now, Some(paging_params)))
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

async fn lvl4(p: web::Path<PathParams4>, data_client: web::Data<Client>) -> Result<HttpResponse> {
    let client = &data_client.into_inner();
    if p.command == "c" && p.sub_command == "info" {
        let community = get_community(client, &p.inst, &p.id).await?;
        html_res(community_info_page(&p.inst, community))
    } else {
        Err(error::ErrorExpectationFailed("Invalid path"))
    }
}

async fn lvl5(p: web::Path<PathParams5>, data_client: web::Data<Client>) -> Result<HttpResponse> {
    let client = &data_client.into_inner();
    let now = &Utc::now().naive_utc();

    if p.command == "post" && p.sub_command == "comment" {
        let post_detail = get_post(client, &p.inst, &p.id).await?;
        let comment_id = match p.sub_id.parse::<i32>() {
            Ok(cid) => cid,
            Err(_) => return Err(error::ErrorExpectationFailed("Comment ID is invalid")),
        };
        let comment = match post_detail.comments.iter().find(|c| c.id == comment_id) {
            Some(c) => c.clone(),
            None => {
                return Err(error::ErrorExpectationFailed(
                    "Comment doesn't belong to this post",
                ))
            }
        };
        html_res(comment_page(&p.inst, comment, post_detail, now))
    } else {
        Err(error::ErrorExpectationFailed("Invalid parameters"))
    }
}

fn html_res(markup: Markup) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}
