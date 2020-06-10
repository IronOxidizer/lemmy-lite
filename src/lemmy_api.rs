use serde::Deserialize;
use actix_web::client::Client;
use actix_web::{Result};

#[derive(Deserialize)]
pub struct CommunityView {
    pub id: i32,
    pub name: String,
    pub title: String,
    description: Option<String>,
    category_id: i32,
    creator_id: i32,
    removed: bool,
    published: String,
    updated: Option<String>,
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


#[derive(Deserialize, Clone)]
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
    fedi_name: String,
    admin: bool,
    banned: bool,
    show_avatars: bool,
    send_notifications_to_email: bool,
    published: String,
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

pub async fn get_community_list(client: &Client, instance: &String) -> Result<CommunityList> {
    let url = format!("https://{}/api/v1/community/list?sort=TopAll", instance);
    println!("Making request: {}", url);

    Ok(CommunityList::from(
        client.get(url).send().await?.json().await?
    ))
}

pub async fn get_community(client: &Client, instance: &String, community: &String) -> Result<CommunityDetail> {
    let url = format!("https://{}/api/v1/community?name={}", instance, community);
    println!("Making request: {}", url);

    Ok(CommunityDetail::from(
        client.get(url).send().await?.json().await?
    ))
}

pub async fn get_post_list(client: &Client, instance: &String, community: Option<&i32>) -> Result<PostList> {
    let url = match community {
        Some(c) => format!("https://{}/api/v1/post/list?type_=All&sort=Hot&community_id={}", instance, c),
        None => format!("https://{}/api/v1/post/list?type_=All&sort=Hot", instance)
    };
    println!("Making request: {}", url);

    Ok(PostList::from(
        client.get(url).send().await?.json().await?
    ))
}

pub async fn get_post(client: &Client, instance: &String, post_id: &String) -> Result<PostDetail> {
    let url = format!("https://{}/api/v1/post?id={}", instance, post_id);
    println!("Making request: {}", url);
    let mut a = client.get(url).send().await?;
    //println!("Request successfully made"); // Breaks on msgs more than 256K https://docs.rs/actix-web/2.0.0/actix_web/client/struct.ClientResponse.html#method.json
    Ok(PostDetail::from(a.json().await?))
}