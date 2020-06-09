use serde::Deserialize;
use actix_web::client::Client;
use actix_web::{Result};

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
    published: String,
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


#[derive(Deserialize)]
pub struct CommentView {
    pub id: i32,
    pub creator_id: i32,
    pub post_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    removed: bool,
    read: Option<bool>,
    published: Option<String>,
    updated: Option<String>,
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

pub async fn get_post_list(client: Client, instance: &String) -> Result<PostList> {
    let url = format!("https://{}/api/v1/post/list?type_=All&sort=Hot", instance);
    println!("Making request: {}", url);

    Ok(PostList::from(
        client.get(url).send().await?.json().await?
    ))
}

pub async fn get_post_detail(client: Client, instance: &String, post_id: &String) -> Result<PostDetail> {
    let url = format!("https://{}/api/v1/post?id={}", instance, post_id);
    println!("Making request: {}", url);

    Ok(PostDetail::from(
        client.get(url).send().await?.json().await?
    ))
}

// zstewart#2487@discord.rust-community-server