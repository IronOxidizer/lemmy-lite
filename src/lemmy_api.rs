use chrono::naive::NaiveDateTime;
use serde::Deserialize;
use actix_web::{Result, client::Client, error::ErrorBadRequest};
use url::{Url, ParseError};

const REQ_MAX_SIZE: usize = 8388608; // 8MB limit
const DEFAULT_SORT: &str = "Hot";

#[derive(Deserialize, Clone)]
pub struct PagingParams {
    pub s: Option<String>,  // Sort
    pub p: Option<i32>,     // Page
    pub l: Option<i32>      // Limit size
}

#[derive(Deserialize, Clone)]
pub struct SearchParams {
    pub q: Option<String>,  // Query
    pub t: Option<String>,  // Content type
    pub c: Option<String>,  // Community name
    pub s: Option<String>,  // Sort
    pub p: Option<i32>,     // Page
    pub l: Option<i32>      // Limit size
}

impl SearchParams {
    pub fn to_paging_params(&self) -> PagingParams {
        PagingParams {
            s: self.s.clone(),
            p: self.p,
            l: self.l
        }
    }
}

#[derive(Deserialize)]
pub struct SiteView {
    // id: i32,
    pub name: String,
//     description: Option<String>,
//     creator_id: i32,
//     published: NaiveDateTime,
//     updated: Option<NaiveDateTime>,
//     enable_downvotes: bool,
//     open_registration: bool,
//     enable_nsfw: bool,
//     icon: Option<String>,
//     banner: Option<String>,
//     creator_name: String,
//     creator_preferred_username: Option<String>,
//     creator_avatar: Option<String>,
//     number_of_users: i32,
//     number_of_posts: i32,
//     number_of_comments: i32,
//     number_of_communities: i32,
}

#[derive(Deserialize)]
pub struct SiteDetail {
    pub site: SiteView,
    // admins: Vec<UserView>,
    // banned: Vec<UserView>,
    // online: i32,
    // version: String,
    // my_user: Option<UserView>,
    // federated_instances: Vec<String>
}

#[derive(Deserialize)]
pub struct CommunityView {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    // category_id: i32,
    // creator_id: i32,
    // removed: bool,
    // published: NaiveDateTime,
    // updated: Option<NaiveDateTime>,
    // deleted: bool,
    // nsfw: bool,
    // creator_name: String,
    // creator_avatar: Option<String>,
    pub category_name: String,
    pub number_of_subscribers: i32,
    pub number_of_posts: i32,
    pub number_of_comments: i32,
    pub hot_rank: i32,
    // user_id: Option<i32>,
    // subscribed: Option<bool>
}

#[derive(Deserialize)]
pub struct CommunityList {
    pub communities: Vec<CommunityView>
}

#[derive(Deserialize)]
pub struct PostView {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub body: Option<String>,
    pub creator_id: i32,
    // community_id: i32,
    // removed: bool,
    // locked: bool,
    pub published: NaiveDateTime,
    // updated: Option<String>,
    // deleted: bool,
    // nsfw: bool,
    pub stickied: bool,
    // embed_title: Option<String>,
    // embed_description: Option<String>,
    // embed_html: Option<String>,
    // thumbnail_url: Option<String>,
    // banned: bool,
    // banned_from_community: bool,
    pub creator_name: String,
    // creator_avatar: Option<String>,
    pub community_name: String,
    // community_removed: bool,
    // community_deleted: bool,
    // community_nsfw: bool,
    pub number_of_comments: i32,
    pub score: i32,
    pub upvotes: i32,
    pub downvotes: i32,
    // hot_rank: i32,
    // newest_activity_time: String,
    pub user_id: Option<i32>,
    // my_vote: Option<i32>,
    // subscribed: Option<bool>,
    // read: Option<bool>,
    // saved: Option<bool>
}

#[derive(Deserialize)]
pub struct PostList {
    pub posts: Vec<PostView>
}

#[derive(Deserialize, Clone)]
pub struct CommentView {
    pub id: i32,
    pub creator_id: i32,
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    // removed: bool,
    // read: Option<bool>,
    pub published: NaiveDateTime,
    // updated: Option<NaiveDateTime>,
    // deleted: Option<bool>,
    // community_id: i32,
    // community_name: String,
    // banned: bool,
    // banned_from_community: bool,
    pub creator_name: String,
    // creator_avatar: Option<String>,
    pub score: i32,
    pub upvotes: i32,
    // pub downvotes: i32,
    // hot_rank: i32,
    // user_id: Option<i32>,
    // my_vote: Option<i32>,
    // subscribed: Option<bool>,
    // saved: Option<bool>,
}

#[derive(Deserialize)]
pub struct PostDetail {
    pub post: PostView,
    pub comments: Vec<CommentView>
}

#[derive(Deserialize)]
pub struct CommunityModeratorView {
    // id: i32,
    // community_id: i32,
    // user_id: i32,
    // published: String,
    pub user_name: String,
    // avatar: Option<String>,
    // community_name: String,
}

#[derive(Deserialize)]
pub struct UserView {
    // id: i32,
    pub name: String,
    // avatar: Option<String>,
    // email: Option<String>,
    // matrix_user_id: Option<String>,
    // fedi_name: Option<String>,
    // admin: bool,
    // banned: bool,
    // show_avatars: bool,
    // send_notifications_to_email: bool,
    // published: NaiveDateTime,
    pub number_of_posts: i32,
    pub post_score: i32,
    pub number_of_comments: i32,
    pub comment_score: i32,
}

#[derive(Deserialize)]
pub struct CommunityDetail {
    pub community: CommunityView,
    pub moderators: Vec<CommunityModeratorView>,
    pub admins: Option<Vec<UserView>>,
    pub online: i32
}

#[derive(Deserialize)]
struct CommunityFollowerView {	
    // id: i32,
    // community_id: i32,
    // user_id: i32,
    // published: NaiveDateTime,
    // user_name: String,
    // avatar: Option<String>,
    // community_name: String,
}

#[derive(Deserialize)]
pub struct UserDetail {
    pub user: UserView,
    // follows: Vec<CommunityFollowerView>,
    // moderates: Vec<CommunityModeratorView>,
    pub comments: Vec<CommentView>,
    pub posts: Vec<PostView>,
}

#[derive(Deserialize)]
pub struct SearchResponse {
    pub type_: String,
    pub comments: Vec<CommentView>,
    pub posts: Vec<PostView>,
    pub communities: Vec<CommunityView>,
    pub users: Vec<UserView>,
}

pub async fn get_site_detail(client: &Client) -> Result<SiteDetail> {
    let url = build_url("site", None)
        .map_err(|e| ErrorBadRequest(e.to_string()))?.to_string();

    Ok(SiteDetail::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

pub async fn get_community_list(client: &Client, paging_params: Option<&PagingParams>) -> Result<CommunityList> {
    let url = build_url("community/list", paging_params)
        .map_err(|e| ErrorBadRequest(e.to_string()))?.to_string();

    Ok(CommunityList::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

pub async fn get_community(client: &Client, community: &String) -> Result<CommunityDetail> {
    let mut base_url = build_url("community", None)
        .map_err(|e| ErrorBadRequest(e.to_string()))?;
    let mut url_builder = base_url.query_pairs_mut();
    let url = url_builder.append_pair("name", community.as_str()).finish().to_string();

    Ok(CommunityDetail::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

pub async fn get_post_list(client: &Client, community: Option<&i32>, community_name: Option<&String>,
    paging_params: Option<&PagingParams>) -> Result<PostList> {
    let mut base_url = build_url("post/list", paging_params)
        .map_err(|e| ErrorBadRequest(e.to_string()))?;
    let mut url_builder = base_url.query_pairs_mut();

    url_builder.append_pair("type_", "All");
    if let Some(cid) = community {
        url_builder.append_pair("community_id", cid.to_string().as_str());
    } else if let Some(cn) = community_name {
        if !cn.is_empty() {
            url_builder.append_pair("community_name", cn);
        }
    }
    let url = url_builder.finish().to_string();

    Ok(PostList::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

pub async fn get_post(client: &Client, post_id: &String) -> Result<PostDetail> {
    let url = build_url("post", None)
        .map_err(|e| ErrorBadRequest(e.to_string()))?.query_pairs_mut()
            .append_pair("id", post_id)
        .finish().to_string();

    Ok(PostDetail::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

pub async fn get_user(client: &Client, username: &String, paging_params: Option<&PagingParams>) -> Result<UserDetail> {
    let url = build_url( "user", paging_params)
        .map_err(|e| ErrorBadRequest(e.to_string()))?.query_pairs_mut()
            .append_pair("saved_only", "false")
            .append_pair("username", username)
        .finish().to_string();

    Ok(UserDetail::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

pub async fn get_search(client: &Client, search_params: &SearchParams) -> Result<SearchResponse> {
    let query = search_params.q.as_ref().ok_or(ErrorBadRequest("Query cannot be empty"))?;

    let mut base_url = build_url( "search", Some(&search_params.to_paging_params()))
        .map_err(|e| ErrorBadRequest(e.to_string()))?;
    let mut url_builder = base_url.query_pairs_mut();
    url_builder.append_pair("q", query.as_str());
    url_builder.append_pair("type_", search_params.t.as_ref().map_or("All", |t| &**t));
    search_params.c.as_ref().map(|c| url_builder.append_pair("community_name", c.as_str()));
    let url = url_builder.finish().to_string();

    Ok(SearchResponse::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

fn build_url(endpoint: &str, paging_params: Option<&PagingParams>) -> Result<Url, ParseError> {
    let mut url = Url::parse(format!("http://{}/api/v1/{}", env!("LEMMY_INTERNAL_HOST"), endpoint).as_str())?;
    let mut url_queries = url.query_pairs_mut();
    
    match paging_params {
        Some(params) => {
            url_queries.append_pair("sort", params.s.as_ref().map_or(DEFAULT_SORT, |s| s.as_str()));
            params.p.map(|p| url_queries.append_pair("page", p.to_string().as_str()));
            params.l.map(|l| url_queries.append_pair("limit", l.to_string().as_str()));
        }, None => {
            url_queries.append_pair("sort", DEFAULT_SORT);
        }
    }
    Ok(url_queries.finish().to_owned())
}