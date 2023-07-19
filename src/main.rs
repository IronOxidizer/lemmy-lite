use actix_web::{client::Client, http::StatusCode, web, App, HttpResponse, HttpServer, Result};
use chrono::offset::Utc;
use lemmy_api::{get_community_info, RedirectToInstanceParam};
use maud::Markup;
use serde::Deserialize;
use templates::{community_info_page, index_page, redirect_page};

mod lemmy_api;
mod templates;

use crate::lemmy_api::{
    get_community_list, get_post, get_post_list, get_user, search, InstancePageParam, SearchParams,
};
use crate::templates::{communities_page, post_list_page, post_page, search_page, user_page};

#[derive(Deserialize)]
struct SearchPageParam {
    inst: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Client::default())
            .route("/", web::get().to(get_index_page))
            .route("/goto", web::get().to(redirect_to_instance_page))
            .route("/i/{inst}", web::get().to(get_instance_page))
            .route("/i/{inst}/search", web::get().to(get_search_page))
            .route(
                "/i/{inst}/communities",
                web::get().to(get_instance_community_page),
            )
            .route("/i/{inst}/c/{community}", web::get().to(get_community_page))
            .route(
                "/i/{inst}/c/{community}/info",
                web::get().to(get_community_info_page),
            )
            .route("/i/{inst}/c/{command}/p/{id}", web::get().to(get_post_page))
            .route("/i/{inst}/u/{id}", web::get().to(get_user_page))
            .service(actix_files::Files::new("/", "./static").show_files_listing())
    })
    .bind("127.0.0.1:1131")?
    .run()
    .await
}

async fn get_index_page() -> Result<HttpResponse> {
    html_res(index_page())
}

async fn redirect_to_instance_page(
    query: web::Query<RedirectToInstanceParam>,
) -> Result<HttpResponse> {
    let query = query.into_inner();
    html_res(redirect_page(&query.domain))
}

async fn get_instance_page(
    path: web::Path<String>,
    query: web::Query<InstancePageParam>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let inst = &path.to_string();
    let client = &data_client.into_inner();

    let now = &Utc::now().naive_utc();
    let paging_params = &query.into_inner();

    let post_list = get_post_list(client, inst, None).await?;

    html_res(post_list_page(
        inst,
        &post_list,
        now,
        None,
        Some(paging_params),
    ))
}

async fn get_instance_community_page(
    path: web::Path<String>,
    query: web::Query<SearchParams>,
    client: web::Data<Client>,
) -> Result<HttpResponse> {
    let instance_name = path.into_inner();
    let search_params = &query.into_inner();
    let client = &client.into_inner();

    let paging_params = &InstancePageParam {
        sort: search_params
            .sort
            .clone()
            .or(Some(lemmy_api_common::lemmy_db_schema::SortType::TopAll)),
        page: search_params.page,
        limit: search_params.limit,
    };
    let communities = get_community_list(client, &instance_name).await?;

    html_res(communities_page(
        &instance_name,
        &communities,
        Some(paging_params),
    ))
}

async fn get_community_page(
    path: web::Path<(String, String)>,
    query: web::Query<InstancePageParam>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let (instance_name, community_name) = path.into_inner();
    let client = &data_client.into_inner();

    let now = &Utc::now().naive_utc();
    let paging_params = &query.into_inner();

    let post_list = get_post_list(client, &instance_name, Some(&community_name)).await?;

    html_res(post_list_page(
        &instance_name,
        &post_list,
        now,
        None,
        Some(paging_params),
    ))
}

async fn get_community_info_page(
    path: web::Path<(String, String)>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let (instance_name, community_name) = path.into_inner();
    let client = &data_client.into_inner();
    let community_info = get_community_info(client, &instance_name, &community_name).await?;
    html_res(community_info_page(&instance_name, community_info))
}

async fn get_search_page(
    p: web::Path<SearchPageParam>,
    query: web::Query<SearchParams>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let client = &data_client.into_inner();
    let search_params = &query.into_inner();

    let now = &Utc::now().naive_utc();
    let search_res = match search_params.query {
        Some(ref query) if !query.is_empty() => Some(search(client, &p.inst, search_params).await?),
        _ => None,
    };

    html_res(search_page(&p.inst, now, search_res, search_params))
}

async fn get_post_page(
    path: web::Path<(String, String, u32)>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let (instance_name, community_name, post_id) = path.into_inner();
    let client = &data_client.into_inner();
    let now = &Utc::now().naive_utc();
    let post_detail = get_post(client, &instance_name, Some(&community_name), post_id).await?;
    html_res(post_page(&instance_name, post_detail, now))
}

async fn get_user_page(
    path: web::Path<(String, String)>,
    query: web::Query<InstancePageParam>,
    data_client: web::Data<Client>,
) -> Result<HttpResponse> {
    let (instance_name, user_name) = path.into_inner();
    let client = &data_client.into_inner();
    let now = &Utc::now().naive_utc();
    let paging_params = &query.into_inner();
    let user = get_user(client, &instance_name, &user_name).await?;
    html_res(user_page(&instance_name, user, now, Some(paging_params)))
}

fn html_res(markup: Markup) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(markup.into_string()))
}
