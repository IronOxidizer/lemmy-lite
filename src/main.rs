use chrono::offset::Utc;
use actix_web::{App, HttpServer, Result, error, middleware, client::Client, 
    web::{Data, Query, ServiceConfig, Path, scope, get}};
use actix_files::Files;
use maud::Markup;

mod templates;
mod lemmy_api;
use templates::*;
use lemmy_api::*;

const BARE_ROOT: &str = "";
const LITE_ROOT: &str = "/lite";


/*
const FAVICON_ICO: &str = "favicon.ico.gz";
const LINK_IMG: &str = "l.svg.gz";
const MEDIA_IMG: &str = "m.svg.gz";
const TEXT_IMG: &str = "t.svg.gz";
const STYLE_CSS: &str = "s.css.gz";

let res = HttpResponse::Ok()
    .content_type("application/x-protobuf")
    //.content_encoding(ContentEncoding::Gzip) // <======= this does not work
    .header("content-encoding", "gzip")  // <======== this works
    .content_encoding(ContentEncoding::Identity)
    .body(data);
Ok(res)
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let instance_name = get_site_detail(&Client::default())
        .await.map_err(|_| std::io::ErrorKind::NotConnected)?.site.name;
    println!("Successfully connected to {}", instance_name);

    HttpServer::new(move || { App::new()
        .wrap(middleware::NormalizePath::default())
        .data((Client::default(), instance_name.clone()))
        .service(
            // Alias /lite paths to / while preserving knowledge of /lite, needed to produce links that work on bare and lite root
            scope(LITE_ROOT).data(LITE_ROOT).configure(config)
        )
        .data(BARE_ROOT).configure(config)
        
    }).bind("127.0.0.1:1131")?.run().await
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg
        .route("/", get().to(root))
        .route("/communities/", get().to(communities))
        .route("/search/", get().to(search))
        .route("/u/{user_id}/", get().to(user))
        .route("/c/{community_id}/", get().to(community))
        .route("/c/{community_id}/info/", get().to(community_info))
        .route("/post/{post_id}/",  get().to(post))
        .route("/post/{post_id}/comment/{comment_id}",  get().to(post_comment))

        // Using actix-files for the sake of simplicity. NginX can be way faster and compressed.
        .service(Files::new("/", "static"));
}

async fn root(data: Data<(Client, String)>, data_root: Data<&'static str>, query_paging_params: Query<PagingParams>) -> Result<Markup> {
    let (client, inst) = &**data; 
    let paging_params = &query_paging_params.into_inner();

    let post_list = get_post_list(client, None, None, Some(paging_params)).await?;
    Ok(post_list_page(inst, **data_root, post_list, &Utc::now().naive_utc(), None, Some(paging_params)))
}

async fn communities(data: Data<(Client, String)>, data_root: Data<&'static str>, query_paging_params: Query<PagingParams>) -> Result<Markup> {
    let (client, inst) = &**data; 
    let mut paging_params = query_paging_params.into_inner();
    // TODO: Can be done better, maybe map_or + replace
    if paging_params.s.is_none() {
        paging_params.s = Some("TopAll".to_string());
    }

    let communities = get_community_list(client, Some(&paging_params)).await?;
    Ok(communities_page(inst, **data_root, communities, Some(&paging_params)))
}

async fn search (data: Data<(Client, String)>, data_root: Data<&'static str>, query_search_params: Query<SearchParams>) -> Result<Markup> {
    let (client, inst) = data.get_ref(); 
    let search_params = &query_search_params.into_inner();
    let now = &Utc::now().naive_utc();

    let search_res = match search_params.q {
        Some(ref query) if !query.is_empty() => Some(get_search(client, search_params).await?),
        _ => None
    };

    Ok(search_page(inst, **data_root, now, search_res, search_params))
}

async fn user(data: Data<(Client, String)>, data_root: Data<&'static str>, path: Path<String>, query_paging_params: Query<PagingParams>) -> Result<Markup> {
    let (client, inst) = data.get_ref();
    let paging_params = &query_paging_params.into_inner();
    let now = &Utc::now().naive_utc();

    let user_detail = get_user(client, &path.to_string(), Some(paging_params)).await?;
    Ok(user_page(inst, **data_root, user_detail, now, Some(paging_params)))
}

async fn community(data: Data<(Client, String)>, data_root: Data<&'static str>, path: Path<String>, query_paging_params: Query<PagingParams>) -> Result<Markup> {
    let (client, inst) = data.get_ref();
    let paging_params = &query_paging_params.into_inner();
    let com_id = &path.to_string();
    let now = &Utc::now().naive_utc();

    let post_list = get_post_list(client, None,
        Some(com_id), Some(paging_params)).await?;
    Ok(post_list_page(inst, **data_root, post_list, now, Some(com_id), Some(paging_params)))
}

async fn community_info(data: Data<(Client, String)>, data_root: Data<&'static str>, path: Path<String>) -> Result<Markup> {
    let (client, inst) = data.get_ref();
    let com_id = &path.to_string();

    let community = get_community(client, com_id).await?;
    Ok(community_info_page(inst, **data_root, community))
}

async fn post(data: Data<(Client, String)>, data_root: Data<&'static str>, path: Path<String>) -> Result<Markup> {
    let (client, inst) = data.get_ref(); 
    let now = &Utc::now().naive_utc();

    let post_detail = get_post(client, &path.to_string()).await?;
    Ok(post_page(inst, **data_root, post_detail, now))
}

async fn post_comment(data: Data<(Client, String)>, data_root: Data<&'static str>, path: Path<(String, i32)>) -> Result<Markup> {
    let (client, inst) = data.get_ref(); 
    let now = &Utc::now().naive_utc();
    let (post_id, comment_id) = &*path;
    let post_detail = get_post(client, &post_id).await?;

    // Find a comment in the comment vec with a matching id
    let comment = match post_detail.comments.iter().find(|c| c.id == *comment_id) {
        Some(c) => c.clone(),
        None => return Err(error::ErrorExpectationFailed("Comment doesn't belong to this post"))
    };
    Ok(comment_page(inst, **data_root, comment, post_detail, now))
}