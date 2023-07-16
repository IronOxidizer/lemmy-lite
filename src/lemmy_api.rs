use actix_web::{client::Client, error::ErrorBadRequest, Result};
use chrono::naive::NaiveDateTime;
use lemmy_api_common::{
    comment::GetCommentsResponse,
    community::ListCommunitiesResponse,
    post::{GetPostResponse, GetPostsResponse},
};
use serde::Deserialize;
use url::{ParseError, Url};

const REQ_MAX_SIZE: usize = 8388608; // 8MB limit

#[derive(Deserialize, Clone)]
pub struct PagingParams {
    pub s: Option<String>, // Sort
    pub p: Option<i32>,    // Page
    pub l: Option<i32>,    // Limit size
}

#[derive(Deserialize, Clone)]
pub struct SearchParams {
    pub q: Option<String>, // Query
    pub t: Option<String>, // Content type
    pub c: Option<String>, // Community name
    pub s: Option<String>, // Sort
    pub p: Option<i32>,    // Page
    pub l: Option<i32>,    // Limit size
}

impl SearchParams {
    pub fn to_paging_params(&self) -> PagingParams {
        PagingParams {
            s: self.s.clone(),
            p: self.p,
            l: self.l,
        }
    }
}

#[derive(Deserialize)]
pub struct CommunityView {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub category_name: String,
    pub number_of_subscribers: i64,
    pub number_of_posts: i64,
    pub number_of_comments: i64,
    pub hot_rank: i32,
}

impl CommunityView {
    pub fn from_lemmy(cv: lemmy_api_common::lemmy_db_views_actor::structs::CommunityView) -> Self {
        Self {
            id: cv.community.id.0,
            name: cv.community.name,
            title: cv.community.title,
            description: cv.community.description,
            category_name: "Some Category Name".to_string(),
            number_of_subscribers: cv.counts.subscribers,
            number_of_posts: cv.counts.posts,
            number_of_comments: cv.counts.comments,
            hot_rank: cv.counts.hot_rank,
        }
    }
}

#[derive(Deserialize)]
pub struct CommunityList {
    pub communities: Vec<CommunityView>,
}

impl CommunityList {
    pub fn from_lemmy(cv: ListCommunitiesResponse) -> Self {
        Self {
            communities: cv
                .communities
                .into_iter()
                .map(CommunityView::from_lemmy)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostView {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub body: Option<String>,
    pub creator_id: i32,
    pub published: NaiveDateTime,
    pub stickied: bool,
    pub creator_name: String,
    pub community_name: String,
    pub number_of_comments: i64,
    pub score: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub user_id: Option<i32>,
}

impl PostView {
    pub fn from_lemmy_pv(pv: lemmy_api_common::lemmy_db_views::structs::PostView) -> Self {
        Self {
            id: pv.post.id.0,
            name: pv.post.name,
            url: pv.post.url.map(|url| url.to_string()),
            body: pv.post.body,
            creator_id: pv.post.creator_id.0,
            published: pv.post.published,
            stickied: false,
            creator_name: pv.creator.name,
            community_name: "Some Community Name".to_string(),
            number_of_comments: pv.counts.comments,
            score: pv.counts.score,
            upvotes: pv.counts.upvotes,
            downvotes: pv.counts.downvotes,
            user_id: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostList {
    pub posts: Vec<PostView>,
}

impl PostList {
    pub fn from(post: GetPostsResponse) -> Self {
        Self {
            posts: post
                .posts
                .into_iter()
                .map(PostView::from_lemmy_pv)
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct CommentView {
    pub id: i32,
    pub creator_id: i32,
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub published: NaiveDateTime,
    pub creator_name: String,
    pub score: i64,
    pub upvotes: i64,
    pub downvotes: i64,
}

impl CommentView {
    pub fn from_lemmy(cv: lemmy_api_common::lemmy_db_views::structs::CommentView) -> Self {
        Self {
            id: cv.comment.id.0,
            creator_id: cv.creator.id.0,
            post_id: cv.post.id.0,
            parent_id: None,
            content: cv.comment.content,
            published: cv.comment.published,
            creator_name: cv.creator.name,
            score: cv.counts.score,
            upvotes: cv.counts.upvotes,
            downvotes: cv.counts.downvotes,
        }
    }
}

#[derive(Deserialize)]
pub struct PostDetail {
    pub post: PostView,
    pub comments: Vec<CommentView>,
}

impl PostDetail {
    pub fn from_lemmy(resp: GetPostResponse, comments: GetCommentsResponse) -> Self {
        Self {
            post: PostView::from_lemmy_pv(resp.post_view),
            comments: comments
                .comments
                .into_iter()
                .map(CommentView::from_lemmy)
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct CommunityModeratorView {
    pub user_name: String,
}

#[derive(Deserialize)]
pub struct UserView {
    pub name: String,
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
    pub online: i32,
}

#[derive(Deserialize)]
struct CommunityFollowerView {}

#[derive(Deserialize)]
pub struct UserDetail {
    pub user: UserView,
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

pub async fn get_community_list(client: &Client, instance: &str) -> Result<CommunityList> {
    let url = build_url(instance, "community/list")
        .map_err(|e| ErrorBadRequest(e.to_string()))
        .unwrap()
        .to_string();

    println!("Making request: {}", url);

    let result = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<ListCommunitiesResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    Ok(CommunityList::from_lemmy(result))
}

pub async fn get_community(
    client: &Client,
    instance: &str,
    community: &str,
) -> Result<CommunityDetail> {
    let mut base_url = build_url(instance, "community")
        .map_err(|e| ErrorBadRequest(e.to_string()))
        .unwrap();
    let mut url_builder = base_url.query_pairs_mut();
    let url = url_builder
        .append_pair("name", community)
        .finish()
        .to_string();

    println!("Making request: {}", url);
    Ok(client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<CommunityDetail>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap())
}

pub async fn get_post_list(client: &Client, instance: &str) -> Result<PostList> {
    let base_url = build_url(instance, "post/list")
        .map_err(|e| ErrorBadRequest(e.to_string()))
        .unwrap();
    let url = base_url.to_string();
    println!("Making request: {}", url);
    let result = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<GetPostsResponse>()
        .await
        .unwrap();

    Ok(PostList::from(result))
}

pub async fn get_post(client: &Client, instance: &str, post_id: &str) -> Result<PostDetail> {
    let post_url = build_url(instance, "post")
        .map_err(|e| ErrorBadRequest(e.to_string()))
        .unwrap()
        .query_pairs_mut()
        .append_pair("id", post_id)
        .finish()
        .to_string();

    println!("getting post from {}", post_url);

    let post = client
        .get(post_url)
        .send()
        .await
        .unwrap()
        .json::<GetPostResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    let comment_url = build_url(instance, "comment/list")
        .map_err(|e| ErrorBadRequest(e.to_string()))
        .unwrap()
        .query_pairs_mut()
        .append_pair("post_id", post_id)
        .finish()
        .to_string();

    println!("getting comments from {}", comment_url);

    let comments = client
        .get(comment_url)
        .send()
        .await
        .unwrap()
        .json::<GetCommentsResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    Ok(PostDetail::from_lemmy(post, comments))
}

pub async fn get_user(client: &Client, instance: &str, username: &str) -> Result<UserDetail> {
    let url = build_url(instance, "user")
        .map_err(|e| ErrorBadRequest(e.to_string()))
        .unwrap()
        .query_pairs_mut()
        .append_pair("saved_only", "false")
        .append_pair("username", username)
        .finish()
        .to_string();

    println!("Making request: {}", url);
    Ok(client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<UserDetail>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap())
}

pub async fn search(
    client: &Client,
    instance: &str,
    search_params: &SearchParams,
) -> Result<SearchResponse> {
    let query = search_params
        .q
        .as_ref()
        .ok_or(ErrorBadRequest("Query cannot be empty"))
        .unwrap();

    let mut base_url = build_url(instance, "search")
        .map_err(|e| ErrorBadRequest(e.to_string()))
        .unwrap();
    let mut url_builder = base_url.query_pairs_mut();
    url_builder.append_pair("q", query.as_str());
    url_builder.append_pair("type_", search_params.t.as_ref().map_or("All", |t| &**t));
    search_params
        .c
        .as_ref()
        .map(|c| url_builder.append_pair("community_name", c.as_str()));
    let url = url_builder.finish().to_string();

    println!("Making request: {}", url);
    Ok(client
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<SearchResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap())
}

fn build_url(instance: &str, endpoint: &str) -> Result<Url, ParseError> {
    let url = Url::parse(format!("https://{}/api/v3/{}", instance, endpoint).as_str()).unwrap();
    Ok(url)
}
