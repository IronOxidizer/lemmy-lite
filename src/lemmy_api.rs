use actix_web::{error::ErrorBadRequest, Result as ActixResult};
use awc::Client as AwcClient;
use chrono::naive::NaiveDateTime;
use lemmy_api_common::{
    comment::GetCommentsResponse,
    community::{GetCommunityResponse, ListCommunitiesResponse},
    lemmy_db_schema::{SearchType, SortType},
    lemmy_db_views::structs::CommentView,
    lemmy_db_views_actor::structs::PersonView,
    person::GetPersonDetailsResponse,
    post::{GetPostResponse, GetPostsResponse},
    site::SearchResponse,
};
use serde::Deserialize;
use url::{ParseError, Url};

const REQ_MAX_SIZE: usize = 80 * 1024 * 1024; // 80MB limit

#[derive(Deserialize, Clone)]
pub struct RedirectToInstanceParam {
    pub domain: String,
}

#[derive(Deserialize, Clone)]
pub struct InstancePageParam {
    pub sort: Option<SortType>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Deserialize, Clone)]
pub struct SearchParams {
    pub query: Option<String>,            // Query
    pub content_type: Option<SearchType>, // Content type
    pub community_name: Option<String>,   // Community name
    pub sort: Option<SortType>,           // Sort
    pub page: Option<i32>,                // Page
    pub limit: Option<i32>,               // Limit size
}

impl SearchParams {
    pub fn to_paging_params(&self) -> InstancePageParam {
        InstancePageParam {
            sort: self.sort,
            page: self.page,
            limit: self.limit,
        }
    }
}

#[derive(Deserialize)]
pub struct CommunityData {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub number_of_subscribers: i64,
    pub number_of_posts: i64,
    pub number_of_comments: i64,
    pub hot_rank: i32,
}

impl CommunityData {
    pub fn from_lemmy(
        lemmy: lemmy_api_common::lemmy_db_views_actor::structs::CommunityView,
    ) -> Self {
        Self {
            id: lemmy.community.id.0,
            name: lemmy.community.name,
            title: lemmy.community.title,
            description: lemmy.community.description,
            number_of_subscribers: lemmy.counts.subscribers,
            number_of_posts: lemmy.counts.posts,
            number_of_comments: lemmy.counts.comments,
            hot_rank: lemmy.counts.hot_rank,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PostData {
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

impl PostData {
    pub fn from_lemmy(lemmy: lemmy_api_common::lemmy_db_views::structs::PostView) -> Self {
        Self {
            id: lemmy.post.id.0,
            name: lemmy.post.name,
            url: lemmy.post.url.map(|url| url.to_string()),
            body: lemmy.post.body,
            creator_id: lemmy.post.creator_id.0,
            published: lemmy.post.published,
            stickied: false,
            creator_name: lemmy.creator.name,
            community_name: lemmy.community.name,
            number_of_comments: lemmy.counts.comments,
            score: lemmy.counts.score,
            upvotes: lemmy.counts.upvotes,
            downvotes: lemmy.counts.downvotes,
            user_id: None,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct CommentData {
    pub id: i32,
    pub creator_id: i32,
    pub post_id: i32,
    pub content: String,
    pub published: NaiveDateTime,
    pub creator_name: String,
    pub score: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub chain: Vec<i32>,
}

impl CommentData {
    pub fn from_lemmy(lemmy: CommentView) -> Self {
        Self {
            id: lemmy.comment.id.0,
            creator_id: lemmy.creator.id.0,
            post_id: lemmy.post.id.0,
            content: lemmy.comment.content,
            published: lemmy.comment.published,
            creator_name: lemmy.creator.name,
            score: lemmy.counts.score,
            upvotes: lemmy.counts.upvotes,
            downvotes: lemmy.counts.downvotes,
            chain: lemmy
                .comment
                .path
                .split(".")
                .skip(1)
                .map(|s| {
                    i32::from_str_radix(s, 10).expect("should only contain valid integer strings")
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Deserialize)]
pub struct PostDetailData {
    pub post: PostData,
    pub comments: Vec<CommentData>,
}

impl PostDetailData {
    pub fn from_lemmy(resp: GetPostResponse, comments: GetCommentsResponse) -> Self {
        Self {
            post: PostData::from_lemmy(resp.post_view),
            comments: comments
                .comments
                .into_iter()
                .map(CommentData::from_lemmy)
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct PersonSummaryData {
    pub name: String,
    pub post_count: i64,
    pub post_score: i64,
    pub comment_count: i64,
    pub comment_score: i64,
}

impl PersonSummaryData {
    pub fn from_lemmy(lemmy: PersonView) -> Self {
        Self {
            name: lemmy.person.name,
            post_count: lemmy.counts.post_count,
            post_score: lemmy.counts.post_score,
            comment_count: lemmy.counts.comment_count,
            comment_score: lemmy.counts.comment_score,
        }
    }
}

#[derive(Deserialize)]
pub struct CommunityDetailData {
    pub community: CommunityData,
    pub moderators: Vec<String>,
    pub admins: Option<Vec<PersonSummaryData>>,
    pub online: i64,
}

impl CommunityDetailData {
    pub fn from_lemmy(lemmy: GetCommunityResponse) -> Self {
        let online_count = lemmy.community_view.counts.users_active_day;
        Self {
            community: CommunityData::from_lemmy(lemmy.community_view),
            moderators: lemmy
                .moderators
                .into_iter()
                .map(|m| m.moderator.name)
                .collect::<Vec<_>>(),
            admins: None,
            online: online_count,
        }
    }
}

#[derive(Deserialize)]
struct CommunityFollowerView {}

#[derive(Deserialize)]
pub struct PersonPageData {
    pub user: PersonSummaryData,
    pub comments: Vec<CommentData>,
    pub posts: Vec<PostData>,
}

impl PersonPageData {
    pub fn from_lemmy(lemmy: GetPersonDetailsResponse) -> Self {
        Self {
            user: PersonSummaryData::from_lemmy(lemmy.person_view),
            comments: lemmy
                .comments
                .into_iter()
                .map(|l| CommentData::from_lemmy(l))
                .collect::<Vec<_>>(),
            posts: lemmy
                .posts
                .into_iter()
                .map(|p| PostData::from_lemmy(p))
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Deserialize)]
pub struct SearchResponseData {
    pub type_: String,
    pub comments: Vec<CommentData>,
    pub posts: Vec<PostData>,
    pub communities: Vec<CommunityData>,
    pub users: Vec<PersonSummaryData>,
}

impl SearchResponseData {
    pub fn from_lemmy(lemmy: SearchResponse) -> Self {
        Self {
            type_: lemmy.type_.to_string(),
            comments: lemmy
                .comments
                .into_iter()
                .map(CommentData::from_lemmy)
                .collect::<Vec<_>>(),
            posts: lemmy
                .posts
                .into_iter()
                .map(PostData::from_lemmy)
                .collect::<Vec<_>>(),
            communities: lemmy
                .communities
                .into_iter()
                .map(CommunityData::from_lemmy)
                .collect::<Vec<_>>(),
            users: lemmy
                .users
                .into_iter()
                .map(PersonSummaryData::from_lemmy)
                .collect::<Vec<_>>(),
        }
    }
}

pub async fn get_community_list(
    client: &AwcClient,
    instance_name: &str,
) -> ActixResult<Vec<CommunityData>> {
    let url = build_url(instance_name, "community/list")
        .map_err(|e| ErrorBadRequest(e.to_string()))?
        .to_string();

    println!("getting communities from {}", url);

    let url_result = client
        .get(url)
        .insert_header(("User-Agent", "lemmy-lite"))
        .send()
        .await
        .unwrap()
        .json::<ListCommunitiesResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    let result = url_result
        .communities
        .into_iter()
        .map(CommunityData::from_lemmy)
        .collect::<Vec<_>>();

    Ok(result)
}

pub async fn get_community_info(
    client: &AwcClient,
    instance: &str,
    community: &str,
) -> ActixResult<CommunityDetailData> {
    let mut base_url =
        build_url(instance, "community").map_err(|e| ErrorBadRequest(e.to_string()))?;
    let mut url_builder = base_url.query_pairs_mut();
    let url = url_builder
        .append_pair("name", community)
        .finish()
        .to_string();

    println!("getting community info from {}", url);

    let result = client
        .get(url)
        .insert_header(("User-Agent", "lemmy-lite"))
        .send()
        .await
        .unwrap()
        .json::<GetCommunityResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    Ok(CommunityDetailData::from_lemmy(result))
}

pub async fn get_post_list(
    client: &AwcClient,
    instance_name: &str,
    community_name: Option<&str>,
    paging_params: Option<&InstancePageParam>,
) -> ActixResult<Vec<PostData>> {
    let mut base_url =
        build_url(instance_name, "post/list").map_err(|e| ErrorBadRequest(e.to_string()))?;

    let mut base_url_builder = base_url.query_pairs_mut();

    if let Some(community_name) = community_name {
        base_url_builder.append_pair("community_name", community_name);
    }

    if let Some(paging_params) = paging_params {
        if let Some(sort) = paging_params.sort {
            base_url_builder.append_pair("sort", from_sort_type_to_str(sort));
        }

        if let Some(limit) = paging_params.limit {
            base_url_builder.append_pair("limit", &format!("{}", limit));
        }

        if let Some(page) = paging_params.page {
            base_url_builder.append_pair("page", &format!("{}", page));
        }
    }

    let base_url_str = base_url_builder.finish().to_string();

    println!("getting posts from {}", base_url_str);

    let url_result = client
        .get(base_url_str)
        .insert_header(("User-Agent", "lemmy-lite"))
        .send()
        .await
        .unwrap()
        .json::<GetPostsResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    let result = url_result
        .posts
        .into_iter()
        .map(PostData::from_lemmy)
        .collect::<Vec<_>>();

    Ok(result)
}

pub async fn get_post(
    client: &AwcClient,
    instance_name: &str,
    community_name: Option<&str>,
    post_id: u32,
) -> ActixResult<PostDetailData> {
    let post_url = build_url(instance_name, "post")
        .map_err(|e| ErrorBadRequest(e.to_string()))?
        .query_pairs_mut()
        .append_pair("id", post_id.to_string().as_str())
        .finish()
        .to_string();

    println!("getting post from {}", post_url);

    let post = client
        .get(post_url)
        .insert_header(("User-Agent", "lemmy-lite"))
        .send()
        .await
        .unwrap()
        .json::<GetPostResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    let mut comment_url =
        build_url(instance_name, "comment/list").map_err(|e| ErrorBadRequest(e.to_string()))?;

    let mut comment_url_builder = comment_url.query_pairs_mut();
    comment_url_builder.append_pair("post_id", post_id.to_string().as_str());

    if let Some(community_name) = community_name {
        comment_url_builder.append_pair("community_name", community_name);
    }

    comment_url_builder.append_pair("limit", "50");

    let comment_url_str = comment_url_builder.finish().to_string();

    println!("getting comments from {}", comment_url_str);

    let comments = client
        .get(comment_url_str)
        .insert_header(("User-Agent", "lemmy-lite"))
        .send()
        .await
        .unwrap()
        .json::<GetCommentsResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    Ok(PostDetailData::from_lemmy(post, comments))
}

pub async fn get_user(
    client: &AwcClient,
    instance: &str,
    username: &str,
) -> ActixResult<PersonPageData> {
    let url = build_url(instance, "user")
        .map_err(|e| ErrorBadRequest(e.to_string()))?
        .query_pairs_mut()
        .append_pair("saved_only", "false")
        .append_pair("username", username)
        .finish()
        .to_string();

    println!("getting user from {}", url);

    let post_info = client
        .get(url)
        .insert_header(("User-Agent", "lemmy-lite"))
        .send()
        .await
        .unwrap()
        .json::<GetPersonDetailsResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    Ok(PersonPageData::from_lemmy(post_info))
}

pub async fn search(
    client: &AwcClient,
    instance: &str,
    search_params: &SearchParams,
) -> ActixResult<SearchResponseData> {
    let mut base_url = build_url(instance, "search").map_err(|e| ErrorBadRequest(e.to_string()))?;
    let mut url_builder = base_url.query_pairs_mut();

    if let Some(query) = &search_params.query {
        url_builder.append_pair("q", query.as_str());
    } else {
        url_builder.append_pair("q", "");
    }
    if let Some(content_type) = &search_params.content_type {
        url_builder.append_pair("type_", from_search_type_to_str(*content_type));
    } else {
        url_builder.append_pair("type_", from_search_type_to_str(SearchType::All));
    }
    if let Some(community_name) = &search_params.community_name {
        url_builder.append_pair("community_name", community_name.as_str());
    }
    if let Some(sort) = &search_params.sort {
        url_builder.append_pair("sort", from_sort_type_to_str(*sort));
    }
    if let Some(page) = &search_params.page {
        url_builder.append_pair("page", &format!("{}", page));
    }
    if let Some(limit) = &search_params.limit {
        url_builder.append_pair("limit", &format!("{}", limit));
    }
    let url = url_builder.finish().to_string();

    println!("getting search from {}", url);

    let search_response = client
        .get(url)
        .insert_header(("User-Agent", "lemmy-lite"))
        // .insert_header("Accept-Encoding", "gzip, deflate, br")
        .send()
        .await
        .unwrap()
        .json::<SearchResponse>()
        .limit(REQ_MAX_SIZE)
        .await
        .unwrap();

    let result = SearchResponseData::from_lemmy(search_response);

    Ok(result)
}

fn build_url(instance: &str, endpoint: &str) -> ActixResult<Url, ParseError> {
    let url = Url::parse(format!("https://{}/api/v3/{}", instance, endpoint).as_str())?;
    Ok(url)
}

fn from_sort_type_to_str(sort: SortType) -> &'static str {
    match sort {
        SortType::Active => "Active",
        SortType::Hot => "Hot",
        SortType::New => "New",
        SortType::Old => "Old",
        SortType::TopDay => "TopDay",
        SortType::TopWeek => "TopWeek",
        SortType::TopMonth => "TopMonth",
        SortType::TopYear => "TopYear",
        SortType::TopAll => "TopAll",
        SortType::MostComments => "MostComments",
        SortType::NewComments => "NewComments",
        SortType::TopHour => "TopHour",
        SortType::TopSixHour => "TopSixHour",
        SortType::TopTwelveHour => "TopTwelveHour",
        SortType::TopThreeMonths => "TopThreeMonths",
        SortType::TopSixMonths => "TopSixMonths",
        SortType::TopNineMonths => "TopNineMonths",
    }
}

pub fn from_search_type_to_str(search_type: SearchType) -> &'static str {
    match search_type {
        SearchType::All => "All",
        SearchType::Comments => "Comments",
        SearchType::Posts => "Posts",
        SearchType::Communities => "Communities",
        SearchType::Users => "Users",
        SearchType::Url => "Url",
    }
}
