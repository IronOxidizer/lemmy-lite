use chrono::naive::NaiveDateTime;
use serde::Deserialize;
use actix_web::client::Client;
use actix_web::Result;

const REQ_MAX_SIZE: usize = 8388608; // 8MB limit

#[derive(Deserialize, Clone)]
pub struct PagingParams {
    pub s: Option<String>, // Sort
    pub p: Option<i32>, // Page
    pub l: Option<i32> // Limit size
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

#[derive(Deserialize)]
pub struct CommunityView {
    pub id: i32,
    pub name: String,
    pub title: String,
    description: Option<String>,
    category_id: i32,
    creator_id: i32,
    removed: bool,
    published: NaiveDateTime,
    updated: Option<NaiveDateTime>,
    deleted: bool,
    nsfw: bool,
    creator_name: String,
    creator_avatar: Option<String>,
    pub category_name: String,
    pub number_of_subscribers: i32,
    pub number_of_posts: i32,
    pub number_of_comments: i32,
    hot_rank: i32,
    user_id: Option<i32>,
    subscribed: Option<bool>
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
    community_id: i32,
    removed: bool,
    locked: bool,
    pub published: NaiveDateTime,
    updated: Option<String>,
    deleted: bool,
    nsfw: bool,
    stickied: bool,
    embed_title: Option<String>,
    embed_description: Option<String>,
    embed_html: Option<String>,
    thumbnail_url: Option<String>,
    banned: bool,
    banned_from_community: bool,
    pub creator_name: String,
    creator_avatar: Option<String>,
    pub community_name: String,
    community_removed: bool,
    community_deleted: bool,
    community_nsfw: bool,
    pub number_of_comments: i32,
    pub score: i32,
    pub upvotes: i32,
    pub downvotes: i32,
    hot_rank: i32,
    newest_activity_time: String,
    pub user_id: Option<i32>,
    my_vote: Option<i32>,
    subscribed: Option<bool>,
    read: Option<bool>,
    saved: Option<bool>
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
    removed: bool,
    read: Option<bool>,
    pub published: NaiveDateTime,
    updated: Option<NaiveDateTime>,
    deleted: Option<bool>,
    community_id: i32,
    community_name: String,
    banned: bool,
    banned_from_community: bool,
    pub creator_name: String,
    creator_avatar: Option<String>,
    pub score: i32,
    pub upvotes: i32,
    pub downvotes: i32,
    hot_rank: i32,
    user_id: Option<i32>,
    my_vote: Option<i32>,
    subscribed: Option<bool>,
    saved: Option<bool>,
}

#[derive(Deserialize)]
pub struct PostDetail {
    pub post: PostView,
    pub comments: Vec<CommentView>
}

#[derive(Deserialize)]
pub struct CommunityModeratorView {
    id: i32,
    community_id: i32,
    user_id: i32,
    published: String,
    user_name: String,
    avatar: Option<String>,
    community_name: String,
}

#[derive(Deserialize)]
pub struct UserView {
    id: i32,
    name: String,
    avatar: Option<String>,
    email: Option<String>,
    matrix_user_id: Option<String>,
    fedi_name: Option<String>,
    admin: bool,
    banned: bool,
    show_avatars: bool,
    send_notifications_to_email: bool,
    published: NaiveDateTime,
    number_of_posts: i32,
    post_score: i32,
    number_of_comments: i32,
    comment_score: i32,
}

#[derive(Deserialize)]
pub struct CommunityDetail {
    pub community: CommunityView,
    moderators: Vec<CommunityModeratorView>,
    admins: Vec<UserView>,
    online: i32
}

#[derive(Deserialize)]
struct CommunityFollowerView {	
    id: i32,
    community_id: i32,
    user_id: i32,
    published: NaiveDateTime,
    user_name: String,
    avatar: Option<String>,
    community_name: String,
}

#[derive(Deserialize)]
pub struct UserDetail {
    user: UserView,
    follows: Vec<CommunityFollowerView>,
    moderates: Vec<CommunityModeratorView>,
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

pub async fn get_community_list(client: &Client, instance: &String, paging_params: Option<&PagingParams>) -> Result<CommunityList> {
    let url = format_url(instance, "v1/community/list", paging_params, None);
    println!("Making request: {}", url);

    Ok(CommunityList::from(
        client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?
    ))
}

pub async fn get_community(client: &Client, instance: &String, community: &String) -> Result<CommunityDetail> {
    let url = format_url(instance,"v1/community",
        None, Some(format!("name={}", community)));
    println!("Making request: {}", url);

    Ok(CommunityDetail::from(
        client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?
    ))
}

pub async fn get_post_list(client: &Client, instance: &String, community: Option<&i32>, community_name: Option<&String>,
    paging_params: Option<&PagingParams>) -> Result<PostList> {
        
    let url = format_url(instance, "v1/post/list", paging_params, 
        match community {
            Some(c) => Some(format!("type_=All&community_id={}", c)),
            None => {
                match community_name {
                    Some(c_n) => Some(format!("type_=All&community_name={}", c_n)),
                    None => Some("type_=All".to_string())
                }
            }
        });
    println!("Making request: {}", url);

    Ok(PostList::from(
        client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?
    ))
}

pub async fn get_post(client: &Client, instance: &String, post_id: &String) -> Result<PostDetail> {
    let url = format_url(instance, "v1/post", None, Some(format!("id={}", post_id)));
    println!("Making request: {}", url);
    Ok(PostDetail::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

pub async fn get_user(client: &Client, instance: &String, username: &String, paging_params: Option<&PagingParams>) -> Result<UserDetail> {
    let url = format_url(instance, "v1/user", paging_params, 
        Some(format!("saved_only=false&username={}", username)));
    println!("Making request: {}", url);
    Ok(UserDetail::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

pub async fn search(client: &Client, instance: &String, search_params: &SearchParams) -> Result<SearchResponse> {
    let url = format_url(instance, "v1/search", None, Some(String::new())); // TODO: Replace string new with params
    Ok(SearchResponse::from(client.get(url).send().await?.json().limit(REQ_MAX_SIZE).await?))
}

// Benchmark faster solutions, too many allocations
// Preallocate buffer and push
fn format_url(instance: &String, endpoint: &str, paging_params: Option<&PagingParams>, extra_params: Option<String>) -> String{
    format!("https://{}/api/{}?{}{}", instance, endpoint,
        match paging_params {
            Some(params) => format!("{}{}{}",
                match &params.s {
                    Some(sort) => format!("sort={}&", sort),
                    None => "sort=Hot&".to_string()
                },
                match &params.p {
                    Some(page) => format!("page={}&", page),
                    None => String::new()
                },
                match &params.l {
                    Some(limit) => format!("limit={}&", limit),
                    None => String::new()
                }),
            None => "sort=Hot&".to_string()
        },
        match extra_params {
            Some(x) => x,
            _ => String::new()
        }
    )
}