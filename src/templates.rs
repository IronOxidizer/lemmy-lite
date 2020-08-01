use chrono::naive::NaiveDateTime;
use maud::{html, Markup};
use crate::lemmy_api::{PostView, PostList, PostDetail, CommentView, CommunityView, CommunityList, UserDetail, PagingParams};

const MEDIA_EXT: &[&str] = &[".png", "jpg", ".jpeg", ".gif"];
const STYLESHEET: &str = "/style.css";
const LINK_IMG: &str = "/link.svg";
const MEDIA_IMG: &str = "/media.svg";
const TEXT_IMG: &str = "/text.svg";

// Pure HTML redirect
pub fn redirect_page(instance: String) -> Markup {
    html! {
        (headers_markup())
        meta content={"0;URL='/" (instance) "'"} http-equiv="refresh";
    }
}

pub fn communities_page(instance: &String, community_list: CommunityList, paging_params: Option<&PagingParams>) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance))
        #cw {
            (pagebar_markup(paging_params))
            table {
                tr {
                    th {"Name"}
                    th {"Title"}
                    th {"Category"}
                    th {"Subscribers"}
                    th {"Posts"}
                    th {"Comments"}
                }
                @for community in community_list.communities {
                    (community_markup(instance, community))
                }
            }
            (pagebar_markup(paging_params))
        }
    }
}

pub fn post_list_page(instance: &String, post_list: PostList, now: &NaiveDateTime, paging_params: Option<&PagingParams>) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance))
        #cw {
            (pagebar_markup(paging_params))
            @for post in &post_list.posts {
                div { (post_markup(instance, post, now)) }
                hr;
            }
            (pagebar_markup(paging_params))
        }
    }
}

pub fn post_page(instance: &String, post_detail: PostDetail, now: &NaiveDateTime) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance))
        #cw {
            (post_markup(instance, &post_detail.post, now))

            @if let Some(body) = &post_detail.post.body {
                p { (body)}
            }
            hr;
            
            (comment_tree_markup(instance, &post_detail.comments, post_detail.post.creator_id, None, 0, None, now))
        }
    }
}

pub fn comment_page(instance: &String, comment: CommentView, post_detail: PostDetail, now: &NaiveDateTime) -> Markup {
    let mut comments = post_detail.comments;
    let comment_id = comment.id;
    comments.retain(|c| Some(c.id) == comment.parent_id ||
        c.id == comment_id ||
        c.parent_id == Some(comment.id));
    let parent = comments.iter().find(|c| Some(c.id) == comment.parent_id);

    html! {
        (headers_markup())
        (navbar_markup(instance))
        #cw {
            (post_markup(instance, &post_detail.post, now))

            @if let Some(body) = &post_detail.post.body {
                p {(body)}
            }
            hr;
            
            @match parent {
                Some(p) => (comment_tree_markup(instance, &comments, post_detail.post.creator_id, p.parent_id, 0, Some(comment_id), now)),
                None => (comment_tree_markup(instance, &comments, post_detail.post.creator_id, None, 0, Some(comment_id), now))
            }
        }
    }
}

pub fn user_page(instance: &String, user: UserDetail, now: &NaiveDateTime, paging_params: Option<&PagingParams>) -> Markup {
    html!{
        (headers_markup())
        (navbar_markup(instance))
        #cw {
            div { (pagebar_markup(paging_params)) }
            @for comment in user.comments {
                (comment_markup(instance, &comment, None, None, now, None))
                hr;
            }
            @for post in user.posts {
                (post_markup(instance, &post, now))
                hr;
            }
            (pagebar_markup(paging_params))
        }
    }
}

fn headers_markup() -> Markup {
    html! {
        meta charset="utf8" name="viewport" content="width=device-width,user-scalable=no,initial-scale=1";
        meta name="theme-color" content="#222";
        link rel="stylesheet" href=(STYLESHEET);
    }
}

fn navbar_markup(instance: &String) -> Markup {
    html! {
        #navbar {
            a href=".." {"Directory Up"}
        
            a href={"/" (instance) } {(instance)}
        
            a href={"/" (instance) "/communities"} {"Communities"}
        }
    }
}

fn pagebar_markup(paging_params: Option<&PagingParams>) -> Markup {
    html! {
        .pb {
            form {
                select name="s" {
                    @if let Some(PagingParams {s: Some(sort), ..}) = paging_params {
                        option selected?[sort==&"Hot".to_string()] value="Hot" {"Hot"}
                        option selected?[sort==&"New".to_string()] value="New" {"New"}
                        option selected?[sort==&"TopDay".to_string()] value="TopDay" {"Day"}
                        option selected?[sort==&"TopWeek".to_string()] value="TopWeek" {"Week"}
                        option selected?[sort==&"TopMonth".to_string()] value="TopMonth" {"Month"}
                        option selected?[sort==&"TopYear".to_string()] value="TopYear" {"Year"}
                        option selected?[sort==&"TopAll".to_string()] value="TopAll" {"All"}
                    } @else {
                        option selected? value="Hot" {"Hot"}
                        option value="New" {"New"}
                        option value="TopDay" {"Day"}
                        option value="TopWeek" {"Week"}
                        option value="TopMonth" {"Month"}
                        option value="TopYear" {"Year"}
                        option value="TopAll" {"All"}
                    }
                }
                @if let Some(PagingParams {p: Some(page), ..}) = paging_params {
                    input type="hidden" name="p" value=(page);
                }
                @if let Some(PagingParams {l: Some(limit), ..}) = paging_params {
                    input type="hidden" name="l" value=(limit);
                }
                input type="submit" value="Sort";
            }

            div {
                @if let Some(PagingParams {p: Some(page), ..}) = paging_params {
                    @if page > &1 {
                        a href=(format!("?{}p={}{}",
                            default_sort_string(paging_params),
                            page-1,
                            default_limit_string(paging_params)))
                            {"Prev"}
                    }
                    a href=(format!("?{}p={}{}",
                        default_sort_string(paging_params),
                        page+1,
                        default_limit_string(paging_params)))
                        {"Next"}
                } @else {
                    a href=(format!("?{}p=2{}",
                    default_sort_string(paging_params),
                    default_limit_string(paging_params)))
                    {"Next"}
                }
            }

            form {
                select name="l" {
                    @if let Some(PagingParams {l: Some(limit), ..}) = paging_params {
                        option selected?[limit==&10] value="10" {"10"}
                        option selected?[limit==&25] value="25" {"25"}
                        option selected?[limit==&50] value="50" {"50"}
                        option selected?[limit==&100] value="100" {"100"}
                    } @else {
                        option selected? value="10" {"10"}
                        option value="25" {"25"}
                        option value="50" {"50"}
                        option value="100" {"100"}
                    }
                }
                @if let Some(PagingParams {p: Some(page), ..}) = paging_params {
                    input type="hidden" name="p" value=(page);
                }
                @if let Some(PagingParams {s: Some(sort), ..}) = paging_params {
                    input type="hidden" name="p" value=(sort);
                }
                input type="submit" value="Sort";
            }
        }
    }
}

fn community_markup(instance: &String, community: CommunityView) -> Markup {
    html! {
        tr {
            td {a.community href= {"/" (instance) "/c/" (community.name)} {
                (community.name)
            }}
            td {(community.title)}
            td {(community.category_name)}
            td {(community.number_of_subscribers)}
            td {(community.number_of_posts)}
            td {(community.number_of_comments)}
        }
    }
}

fn post_markup(instance: &String, post: &PostView, now: &NaiveDateTime) -> Markup {
    html!{
        .row {
            p.score { (post.score) }
            @match &post.url {
                Some(url) => {
                    a href=(url) {
                        img.preview src={
                            @if ends_with_any(url.clone(), MEDIA_EXT) {
                                (MEDIA_IMG)
                            } @else {
                                (LINK_IMG)
                            }
                        };
                    }
                }, None => {
                    a href={"/" (instance) "/post/" (post.id )} {
                        img.preview src=(TEXT_IMG);
                    }
                }
            }
            div {
                a href={"/" (instance) "/post/" (post.id )} {
                    (post.name)
                }
                .mute{
                    "by "
                    a.username href={"/" (instance) "/u/" (post.creator_name) } {
                        (post.creator_name)
                    }
                    " to "
                    a.community href= {"/" (instance) "/c/" (post.community_name)} {
                        (post.community_name)
                    }
                    " • ˄ " (post.upvotes) " ˅ " (post.downvotes)
                    a href={"/" (instance) "/post/" (post.id )} {
                        " • ✉ " (post.number_of_comments)
                    }
                    " • " (simple_duration(now, post.published))
                }
            }
        }
    }
}

fn comment_details_markup(instance: &String, comment: &CommentView, post_creator_id: Option<i32>, now: &NaiveDateTime) -> Markup {
    return html! {
        p.ch {
            a.username href={"/" (instance) "/u/" (comment.creator_name)} {
                (comment.creator_name)
            }
            @if let Some(pcid) = post_creator_id {
                @if pcid == comment.creator_id {
                    span.badge { ("creator") }
                }
            }

            " ϟ" (comment.score) 
            a href={"/" (instance) "/post/" (comment.post_id) "/comment/" (comment.id)} {
                " ⚓ "
            }
            
            (simple_duration(now, comment.published))

        }
    }
}

fn comment_markup(instance: &String, comment: &CommentView, post_creator_id: Option<i32>, highlight_id: Option<i32>, now: &NaiveDateTime, children: Option<Markup>) -> Markup {
    html! {
        @if let Some(hid) = highlight_id {
            @if comment.id == hid {
                .highlight {
                    (comment_details_markup(instance, comment, post_creator_id, now))
                }
            } @else {
                (comment_details_markup(instance, comment, post_creator_id, now))
            }
        } @else {
            (comment_details_markup(instance, comment, post_creator_id, now))
        }
        
        @if children.is_some() {
            input.cc type="checkbox";
        }
        
        div {
            (comment.content)
            @if let Some(c) = children {
                (c);
            }
        }
    }
}

// zstewart#2487@discord.rust-community-server
fn comment_tree_markup(instance: &String, comments: &[CommentView],
    post_creator_id: i32, comment_parent_id: Option<i32>, depth: i32, highlight_id: Option<i32>, now: &NaiveDateTime) -> Markup {

    html! {
        @for comment in comments.iter().filter(|c| c.parent_id == comment_parent_id) {
            .{"b" (
                if depth == 0 {"r".to_string()} else {((depth - 1)%6).to_string()}
                )} {
                (comment_markup(instance, comment, Some(post_creator_id), highlight_id, now,
                    Some(comment_tree_markup(instance, comments, post_creator_id, Some(comment.id), depth+1, highlight_id, now))))
            }
        }
    }
}

fn ends_with_any(s: String, suffixes: &'static [&'static str]) -> bool {
    return suffixes.iter().any(|&suffix| s.to_lowercase().ends_with(suffix));
}

fn simple_duration(now: &NaiveDateTime, record: NaiveDateTime) -> String {
    let seconds = now.signed_duration_since(record).num_seconds();
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m",
            now.signed_duration_since(record).num_minutes())
    } else if seconds < 86400 {
        format!("{}h",
            now.signed_duration_since(record).num_hours())
    } else if seconds < 2629746 {
        format!("{}d",
            now.signed_duration_since(record).num_days())
    } else if seconds < 31556952 {
        format!("{}mo",
            now.signed_duration_since(record).num_weeks() / 4)
    } else {
        format!("{}y",
            now.signed_duration_since(record).num_weeks() / 52)
    }
}

fn default_sort_string(paging_params: Option<&PagingParams>) -> String {
    if let Some(PagingParams {s: Some(sort), ..}) = paging_params {
        format!("s={}&", sort)
    } else {
        String::new()
    }
}

fn default_limit_string(paging_params: Option<&PagingParams>) -> String {
    if let Some(PagingParams {l: Some(limit), ..}) = paging_params {
        format!("&l={}", limit)
    } else {
        String::new()
    }
}