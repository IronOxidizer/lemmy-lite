use actix_web::client::Client;
use actix_web::{Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    body: Option<String>,
    creator_id: i32,
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
    number_of_comments: i32,
    pub score: i32,
    pub upvotes: i32,
    pub downvotes: i32,
    hot_rank: i32,
    newest_activity_time: String,
    user_id: Option<i32>,
    my_vote: Option<i32>,
    subscribed: Option<bool>,
    read: Option<bool>,
    saved: Option<bool>
}

#[derive(Deserialize)]
pub struct PostList {
    pub posts: Vec<Post>
}

pub async fn get_post_list(instance: &String, client: Client) -> Result<PostList> {
    let url = format!("https://{}/api/v1/post/list?type_=All&sort=Hot", instance);
    println!("Making request: {}", url);

    Ok(PostList::from(
        client.get(url)
            .header("User-Agent", "Actix-web")
            .send().await?.json().await?
    ))
}