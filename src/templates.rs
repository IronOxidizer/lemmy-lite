use chrono::naive::NaiveDateTime;
use maud::{html, Markup, PreEscaped};
use pulldown_cmark::{
    Parser,
    html as pchtml,
    Event::{Start, End},
    Tag::{Image, Link}
};
use crate::lemmy_api::{PostView, PostList, PostDetail, CommentView, CommunityView, CommunityList, UserView, UserDetail, PagingParams, SearchParams, SearchResponse};

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
        (navbar_markup(instance, Some(html!{
            a.l href={"/" (instance) "/communities"} {"/communities"}
        }), None))
        #cw {
            (pagebar_markup(paging_params))
            .tw {
                table {
                    tr {
                        th {"Name"}
                        th {"Title"}
                        th {"Category"}
                        th {"Subscribers"}
                        th {"Posts"}
                        th {"Comments"}
                    }
                    @for community in &community_list.communities {
                        (community_markup(instance, community))
                    }
                }
            }
            (pagebar_markup(paging_params))
        }
    }
}

pub fn post_list_page(instance: &String, post_list: PostList, now: &NaiveDateTime, community: Option<&String>, paging_params: Option<&PagingParams>) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(
            instance,
            community.map(|c| html!{
                a.l href=(c) {"/c/" (c)}
            }), 
            community.map(|c| SearchParams {
                q: None,
                t: None,
                c: Some(c.clone()),
                s: None,
                p: None,
                l: None
            }).as_ref()
        ))
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
        (navbar_markup(instance, None, None))
        #cw {
            (post_markup(instance, &post_detail.post, now))

            @if let Some(body) = &post_detail.post.body {
                p { (mdstr_to_html(body))}
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
        (navbar_markup(instance, None, None))
        #cw {
            (post_markup(instance, &post_detail.post, now))

            @if let Some(body) = &post_detail.post.body {
                p {(mdstr_to_html(body))}
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
        (navbar_markup(instance, Some(html!{
            a.username href={"/" (instance) "/u/" (user.user.name)} {"/u/" (user.user.name)}
        }), None))
        #cw {
            div { (pagebar_markup(paging_params)) }
            @for post in user.posts {
                (post_markup(instance, &post, now))
                hr;
            }
            @for comment in user.comments {
                (comment_markup(instance, &comment, None, None, now, None))
                hr;
            }
            (pagebar_markup(paging_params))
        }
    }
}

pub fn search_page(instance: &String, now: &NaiveDateTime, search_res: Option<&SearchResponse>, search_params: &SearchParams) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance, Some(html!{
            a.l href={"/" (instance) "/search"} {"/search"}
        }), Some(search_params)))
        #cw {
            (searchbar_markup(search_params))
            @if let Some(results) = search_res {
                @if !results.communities.is_empty() {
                    .tw {
                        table {
                            tr {
                                th {"Community"}
                                th {"Title"}
                                th {"Category"}
                                th {"Subscribers"}
                                th {"Posts"}
                                th {"Comments"}
                            }
                            @for community in &results.communities {
                                (community_markup(instance, community))
                            }
                        }
                    }
                    hr;
                }

                @if !results.users.is_empty() {
                    .tw {
                        table {
                            tr {
                                th {"User"}
                                th {"Post Score"}
                                th {"Posts"}
                                th {"Comment Score"}
                                th {"Comments"}
                            }
                            @for user in &results.users {
                                (user_markup(instance, user))
                            }
                        }
                    }
                    hr;
                }
                @for post in &results.posts {
                    (post_markup(instance, post, now))
                    hr;
                }
                @for comment in &results.comments {
                    (comment_markup(instance, comment, None, None, now, None))
                    hr;
                }
                (searchbar_markup(search_params))
            } @else {
                "Empty search"
            }
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

fn navbar_markup(instance: &String, embed: Option<Markup>, search_params: Option<&SearchParams>) -> Markup {
    let paging_params = search_params.map(|s| s.to_paging_params());
    html! {
        #navbar {
            a href={"/" (instance) "/communities"} {"Communities"}
        
            div {
                a href={"/" (instance)} {(instance)}
                @if let Some(e) = embed {(e)}
            }
        
            form action={"/" (instance) "/search"} {
                @if let Some(SearchParams {q: Some(query), ..}) = search_params {
                    input name="q" placeholder="Search" value=((query));
                    (default_sort_markup(paging_params.as_ref()))
                    (default_page_markup(paging_params.as_ref()))
                    (default_limit_markup(paging_params.as_ref()))
                    (default_type_markup(search_params))
                } @else {
                    input name="q" placeholder="Search";
                }
                (default_community_markup(search_params))
                input type="submit" value="Go";
            }
        }
    }
}

fn community_markup(instance: &String, community: &CommunityView) -> Markup {
    html! {
        tr {
            td {a.l href= {"/" (instance) "/c/" (community.name)} {
                (community.name)
            }}
            td {(community.title)}
            td {(community.category_name)}
            td.ar {(community.number_of_subscribers)}
            td.ar {(community.number_of_posts)}
            td.ar {(community.number_of_comments)}
        }
    }
}

fn user_markup(instance: &String, user: &UserView) -> Markup {
    html! {
        tr {
            td {a.username href= {"/" (instance) "/u/" (user.name)} {
                (user.name)
            }}
            td.ar {(user.post_score)}
            td.ar {(user.number_of_posts)}
            td.ar {(user.comment_score)}
            td.ar {(user.number_of_comments)}
        }
    }
}

fn post_markup(instance: &String, post: &PostView, now: &NaiveDateTime) -> Markup {
    html!{
        .row {
            p.score {(post.score)}
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
                    a.l href= {"/" (instance) "/c/" (post.community_name)} {
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

fn comment_header_markup(instance: &String, comment: &CommentView, post_creator_id: Option<i32>, highlight_id: Option<i32>, now: &NaiveDateTime) -> Markup {
    return html! {
        p.ch.highlight[Some(comment.id) == highlight_id] {
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
        (comment_header_markup(instance, comment, post_creator_id, highlight_id, now))
        
        @if children.is_some() {
            input.cc type="checkbox";
        }
        
        div {
            (mdstr_to_html(comment.content.as_str()))
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

fn pagebar_markup(paging_params: Option<&PagingParams>) -> Markup {
    html! {
        .pb {
            form {
                (sort_markup(paging_params))
                @if let Some(PagingParams {p: Some(page), ..}) = paging_params {
                    input type="hidden" name="p" value=(page);
                }
                (limit_size_markup(paging_params))
                input type="submit" value="Apply";
            }

            div {
                @if let Some(PagingParams {p: Some(page), ..}) = paging_params {
                    @if page > &1 {
                        form {
                            (default_sort_markup(paging_params))
                            input type="hidden" name="p" value=((page-1));
                            (default_limit_markup(paging_params))
                            input type="submit" value="Prev";
                        }
                        " " (page) " "
                    }
                    form {
                        (default_sort_markup(paging_params))
                        input type="hidden" name="p" value=((page+1));
                        (default_limit_markup(paging_params))
                        input type="submit" value="Next";
                    }
                } @else {
                    form {
                        (default_sort_markup(paging_params))
                        input type="hidden" name="p" value=(2);
                        (default_limit_markup(paging_params))
                        input type="submit" value="Next";
                    }
                }
            }
        }
    }
}

fn searchbar_markup(search_params: &SearchParams) -> Markup {
    let paging_params_bare = &(search_params.to_paging_params());
    let paging_params = Some(paging_params_bare);
    html! {
        .pb {
            form {
                (default_query_markup(Some(search_params)))
                (sort_markup(paging_params))
                (default_page_markup(paging_params))
                (limit_size_markup(paging_params))

                select name="t" {
                    @if let Some(ref type_) = search_params.t {
                        option selected?[type_==&"All".to_string()] value="All" {"All"}
                        option selected?[type_==&"Comments".to_string()] value="Comments" {"Comments"}
                        option selected?[type_==&"Posts".to_string()] value="Posts" {"Posts"}
                        option selected?[type_==&"Communities".to_string()] value="Communities" {"Communities"}
                        option selected?[type_==&"Users".to_string()] value="Users" {"Users"}
                        option selected?[type_==&"Url".to_string()] value="Url" {"URLs"}

                    } @else {
                        option value="All" {"All"}
                        option value="Comments" {"Comments"}
                        option value="Posts" {"Posts"}
                        option value="Communities" {"Communities"}
                        option value="Users" {"Users"}
                        option value="Url" {"URLs"}
                    }
                }

                @if let Some(ref community) = search_params.c {
                    input type="text" name="c" placeholder="Community" value=((community));
                } @else {
                    input type="text" name="c" placeholder="Community";
                }

                input type="submit" value="Apply";
            }

            div {
                @if let Some(PagingParams {p: Some(page), ..}) = paging_params {
                    @if page > &1 {
                        form {
                            (default_query_markup(Some(search_params)))
                            (default_sort_markup(paging_params))
                            input type="hidden" name="p" value=((page-1));
                            (default_limit_markup(paging_params))
                            (default_type_markup(Some(search_params)))
                            (default_community_markup(Some(search_params)))
                            input type="submit" value="Prev";
                        }
                        " " (page) " "
                    }
                    form {
                        (default_query_markup(Some(search_params)))
                        (default_sort_markup(paging_params))
                        input type="hidden" name="p" value=((page+1));
                        (default_limit_markup(paging_params))
                        (default_type_markup(Some(search_params)))
                        (default_community_markup(Some(search_params)))
                        input type="submit" value="Next";
                    }
                } @else {
                    form {
                        (default_query_markup(Some(search_params)))
                        (default_sort_markup(paging_params))
                        input type="hidden" name="p" value=(2);
                        (default_limit_markup(paging_params))
                        (default_type_markup(Some(search_params)))
                        (default_community_markup(Some(search_params)))
                        input type="submit" value="Next";
                    }
                }
            }
        }
    }
}

fn sort_markup(paging_params: Option<&PagingParams>) -> Markup {
    html! {
        select name="s" {
            @if let Some(PagingParams {s: Some(sort), ..}) = paging_params {
                option selected?[sort==&"Active".to_string()] value="Active" {"Active"}
                option selected?[sort==&"Hot".to_string()] value="Hot" {"Hot"}
                option selected?[sort==&"New".to_string()] value="New" {"New"}
                option selected?[sort==&"TopDay".to_string()] value="TopDay" {"Day"}
                option selected?[sort==&"TopWeek".to_string()] value="TopWeek" {"Week"}
                option selected?[sort==&"TopMonth".to_string()] value="TopMonth" {"Month"}
                option selected?[sort==&"TopYear".to_string()] value="TopYear" {"Year"}
                option selected?[sort==&"TopAll".to_string()] value="TopAll" {"All"}
            } @else {
                option value="Active" {"Active"}
                option value="Hot" {"Hot"}
                option value="New" {"New"}
                option value="TopDay" {"Day"}
                option value="TopWeek" {"Week"}
                option value="TopMonth" {"Month"}
                option value="TopYear" {"Year"}
                option value="TopAll" {"All"}
            }
        }
    }
}

fn limit_size_markup(paging_params: Option<&PagingParams>) -> Markup {
    html! {
        select name="l" {
            @if let Some(PagingParams {l: Some(limit), ..}) = paging_params {
                option selected?[limit==&10] value="10" {"10"}
                option selected?[limit==&25] value="25" {"25"}
                option selected?[limit==&50] value="50" {"50"}
                option selected?[limit==&100] value="100" {"100"}
            } @else {
                option value="10" {"10"}
                option value="25" {"25"}
                option value="50" {"50"}
                option value="100" {"100"}
            }
        }
    }
}

fn default_sort_markup(paging_params: Option<&PagingParams>) -> Markup {
    html! {
        @if let Some(PagingParams {s: Some(sort), ..}) = paging_params {
            input type="hidden" name="s" value=((sort));
        }
    }
}

fn default_page_markup(paging_params: Option<&PagingParams>) -> Markup {
    html! {
        @if let Some(PagingParams {p: Some(page), ..}) = paging_params {
            input type="hidden" name="p" value=((page));
        }
    }
}

fn default_limit_markup(paging_params: Option<&PagingParams>) -> Markup {
    html! {
        @if let Some(PagingParams {l: Some(limit), ..}) = paging_params {
            input type="hidden" name="l" value=((limit));
        }
    }
}

fn default_query_markup(search_params: Option<&SearchParams>) -> Markup {
    html! {
        @if let Some(SearchParams {q: Some(query), ..}) = search_params {
            input type="hidden" name="q" value=((query));
        }
    }
}

fn default_type_markup(search_params: Option<&SearchParams>) -> Markup {
    html! {
        @if let Some(SearchParams {t: Some(type_), ..}) = search_params {
            input type="hidden" name="t" value=((type_));
        }
    }
}

fn default_community_markup(search_params: Option<&SearchParams>) -> Markup {
    html! {
        @if let Some(SearchParams {c: Some(community), ..}) = search_params {
            input type="hidden" name="c" value=((community));
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

fn mdstr_to_html(text: &str) -> Markup {
    let parser = Parser::new(text)
        .map(|event| match event { // Remove image rendering by default to save user data usage
            Start(Image(linktype, url, title)) if title.is_empty() =>
                Start(Link(linktype, url.clone(), url)),
            Start(Image(linktype, url, title)) =>
                Start(Link(linktype, url, title)),
            End(Image(linktype, url, title)) if title.is_empty() =>
                End(Link(linktype, url.clone(), url)),
            End(Image(linktype, url, title)) =>
                End(Link(linktype, url, title)),
            _ => event,
        });
    let mut html_output = String::new();
    pchtml::push_html(&mut html_output, parser);
    PreEscaped(html_output)
}