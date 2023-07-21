/*
DOM ids
#w  = content Wrapper
#n  = Navbar
#f  = Footer

DOM classes
.o  = Overflow
.e  = align right (End)
.r  = Row
.p  = content type Preview
.s  = Score
.u  = Username
.l  = Link
.b  = Badge
.pb = PageBar
.h  = Highlight
.m  = Mute
.ch = Comment Header details
.c  = Collapsible
.br = Border Root
.b? = Border 0-5
*/

use crate::lemmy_api::{
    CommentData, CommunityData, CommunityDetailData, InstancePageParam, PersonPageData,
    PersonSummaryData, PostData, PostDetailData, SearchParams, SearchResponseData,
};
use chrono::naive::NaiveDateTime;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use pulldown_cmark::{html as pchtml, CowStr, Event, Parser, Tag};

const MEDIA_EXT: &[&str] = &[".png", "jpg", ".jpeg", ".gif", ".svg", ".webm", ".mp4"];
const STYLESHEET: &str = "/s.css";
const LINK_IMG: &str = "/l.svg";
const MEDIA_IMG: &str = "/m.svg";
const TEXT_IMG: &str = "/t.svg";

// Pure HTML redirect
pub fn redirect_page(instance: &str) -> Markup {
    html! {
        (headers_markup())
        meta content={"0;URL='/i/" (instance) "'"} http-equiv="refresh";
    }
}

pub fn index_page() -> Markup {
    html! {
        (headers_markup())
        #w  {
                h2 { "Enter a lemmy domain or URL" };
                form id="w" action="/goto" method="GET" {
                input type="text" name="domain";
                input type="submit" value="GO";
            }
        }
    }
}

pub fn create_link(
    instance: Option<&str>,
    community: Option<&str>,
    post: Option<i32>,
    username: Option<&str>,
    limit: Option<i32>,
    page: Option<i32>,
    sort: Option<&str>,
) -> String {
    let mut result = String::default();

    if let Some(instance) = instance {
        result.push_str(&format!("/i/{}", instance));
    }

    if let Some(community) = community {
        result.push_str(&format!("/c/{}", community));
    }

    if let Some(post) = post {
        result.push_str(&format!("/p/{}", post));
    }

    if let Some(username) = username {
        result.push_str(&format!("/u/{}", username));
    }

    let to_add = match (limit, page, sort) {
        (None, None, Some(sort)) => format!("?sort={}", sort),
        (None, Some(page), None) => format!("?page={}", page),
        (None, Some(page), Some(sort)) => format!("?page={}&sort={}", page, sort),
        (Some(limit), None, None) => format!("?limit={}", limit),
        (Some(limit), None, Some(sort)) => todo!("?limit={}&sort={}", limit, sort),
        (Some(limit), Some(page), None) => format!("?limit={}&page={}", limit, page),
        (Some(limit), Some(page), Some(sort)) => {
            format!("?limit={}&page={}&sort={}", limit, page, sort)
        }
        _ => String::default(),
    };

    result.push_str(&to_add);

    return result;
}

pub fn communities_page(
    instance: &str,
    community_list: &[CommunityData],
    paging_params: Option<&InstancePageParam>,
) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance, Some(html!{
            a.l href=(create_link(Some(instance),None,None,None,None,None,None));
        }), None))
        #w {
            (pagebar_markup(paging_params))
            .o {
                table {
                    tr {
                        th {"Name"}
                        th {"Title"}
                        th {"Category"}
                        th {"Subscribers"}
                        th {"Posts"}
                        th {"Comments"}
                    }
                    @for community in community_list {
                        (community_markup(instance, community))
                    }
                }
            }
            (pagebar_markup(paging_params))
        }
    }
}

pub fn post_list_page(
    instance: &str,
    post_list: &[PostData],
    now: &NaiveDateTime,
    community: Option<&str>,
    paging_params: Option<&InstancePageParam>,
) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(
            instance,
            community.map(|c| html!{
                a.l href=(create_link(Some(instance),Some(c),None,None,None,None,None));
            }),
            community.map(|c| SearchParams {
                query: None,
                content_type: None,
                community_name: Some(c.to_string()),
                sort: None,
                page: None,
                limit: None
            }).as_ref()
        ))
        #w {
            (pagebar_markup(paging_params))
            @for post in post_list {
                div { (post_markup(instance, post, now)) }
                hr;
            }
            (pagebar_markup(paging_params))
            @if let Some(c) = community {
                #f href={(create_link(Some(instance),Some(c),None,None,None,None,None)) "/info"} {
                    "More info on /c/" (c)
                }
            }
        }
    }
}

pub fn community_info_page(instance: &str, community_detail: CommunityDetailData) -> Markup {
    let community = &community_detail.community;
    html! {
        (headers_markup())
        (navbar_markup(instance, Some(html! {
            a.l href={(create_link(Some(instance),Some(&community.name),None,None,None,None,None))} {
                "/c/" (community_detail.community.name)
            }
            a href={(create_link(Some(instance),Some(&community.name),None,None,None,None,None)) "/info"} {
                "/info"
            }
        }), None))
        #w {
            h1 {(community.name)}
            h2 {(community.title)}
            h3 {"Number of online: " (community_detail.online)}
            h3 {"Number of subscribers: " (community.number_of_subscribers)}
            h3 {"Number of posts: " (community.number_of_posts)}
            h3 {"Number of comments: " (community.number_of_comments)}
            @if community.hot_rank > 0 {
                h3 {"Hot rank: " (community.hot_rank)}
            }
            @if let Some(ref d) = community.description {
                h3 {"Description:"}
                p {(mdstr_to_html(d))}
            }

            @if let Some(a) = community_detail.admins {
                h3 {"Admins: "}
                @if !a.is_empty() {
                    .w {
                        table {
                            tr {
                                th {"User"}
                                th {"Post Score"}
                                th {"Posts"}
                                th {"Comment Score"}
                                th {"Comments"}
                            }
                            @for user in &a {
                                (user_markup(instance, user))
                            }
                        }
                    }
                    hr;
                }
            }

            @if !community_detail.moderators.is_empty() {
                h3 {"Moderators: "}
                .w {
                    table {
                        tr {
                            th {"User"}
                        }
                        @for moderator in &community_detail.moderators {
                            (moderator_markup(instance, moderator))
                        }
                    }
                }
                hr;
            }
        }
    }
}

pub fn post_page(instance: &str, post_detail: PostDetailData, now: &NaiveDateTime) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance, None, None))
        #w {
            (post_markup(instance, &post_detail.post, now))

            @if let Some(body) = &post_detail.post.body {
                p {(mdstr_to_html(body))}
            }
            hr;

            (comment_tree_markup(instance, &post_detail.comments, post_detail.post.creator_id, None, 0, None, now))
        }
    }
}

pub fn user_page(
    instance: &str,
    user: PersonPageData,
    now: &NaiveDateTime,
    paging_params: Option<&InstancePageParam>,
) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance, Some(html!{
            a.u href=(create_link(Some(instance),None,None,Some(&user.user.name),None,None,None)) {"/u/" (user.user.name)}
        }), None))
        #w {
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

pub fn search_page(
    instance: &str,
    now: &NaiveDateTime,
    search_res: Option<SearchResponseData>,
    search_params: &SearchParams,
) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance, Some(html!{
            a.l href={(create_link(Some(instance),None,None,None,None,None,None)) "/search"} {"/search"}
        }), Some(search_params)))
        #w {
            (searchbar_markup(search_params))
            @if let Some(results) = search_res {
                @if !results.communities.is_empty() {
                    .o {
                        table {
                            tr {
                                th {"Community"}
                                th {"Title"}
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
                    .o {
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
        (DOCTYPE)
        meta charset="utf8" name="mobile-web-app-capable" content="yes";
        meta name="apple-mobile-web-app-capable" content="yes";
        meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
        meta name="viewport" content="width=device-width,user-scalable=no,initial-scale=1";
        meta name="theme-color" content="#222";
        meta name="description" content="Lemmy";
        title { "Lemmy" }
        link rel="stylesheet" href=(STYLESHEET);
    }
}

fn navbar_markup(
    instance: &str,
    embed: Option<Markup>,
    search_params: Option<&SearchParams>,
) -> Markup {
    let paging_params = search_params.map(|s| s.to_paging_params());
    html! {
        #n {
            a href={"/i/" (instance) "/c"} {"Communities"}

            div {
                a href={"/i/" (instance)} {(instance)}
                @if let Some(e) = embed {(e)}
            }

            form action={"/i/" (instance) "/search"} {
                @if let Some(SearchParams {query: Some(query), ..}) = search_params {
                    input name="q" placeholder="Search" value=((query));
                    (default_sort_markup(paging_params.as_ref()))
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

fn community_markup(instance: &str, community: &CommunityData) -> Markup {
    html! {
        tr {
            td {a.l href= {(create_link(Some(instance),None,None,None,None,None,None)) "/c/" (community.name)} {
                (community.name)
            }}
            td {(community.title)}
            td.e {(community.number_of_subscribers)}
            td.e {(community.number_of_posts)}
            td.e {(community.number_of_comments)}
        }
    }
}

fn user_markup(instance: &str, user: &PersonSummaryData) -> Markup {
    html! {
        tr {
            td {a.u href=(create_link(Some(instance),None,None,Some(&user.name),None,None,None)) {
                (user.name)
            }}
            td.e {(user.post_score)}
            td.e {(user.post_count)}
            td.e {(user.comment_score)}
            td.e {(user.comment_count)}
        }
    }
}

fn moderator_markup(instance: &str, moderator: &str) -> Markup {
    html! {
        tr {
            td {a.u href=(create_link(Some(instance),None,None,Some(moderator),None,None,None)) {
                (moderator)
            }}
        }
    }
}

fn post_markup(instance: &str, post: &PostData, now: &NaiveDateTime) -> Markup {
    let post_link = create_link(
        Some(instance),
        Some(&post.community_name),
        Some(post.id),
        None,
        None,
        None,
        None,
    );
    let creator_link = create_link(
        Some(instance),
        None,
        None,
        Some(&post.creator_name),
        None,
        None,
        None,
    );
    let community_link = create_link(
        Some(instance),
        Some(&post.community_name),
        None,
        None,
        None,
        None,
        None,
    );
    html! {
        .r {
            p.s {(post.score)}
            @match &post.url {
                Some(url) => {
                    a href=(url) {
                        img.p src={
                            @if ends_with_any(url.clone(), MEDIA_EXT) {
                                (MEDIA_IMG)
                            } @else {
                                (LINK_IMG)
                            }
                        };
                    }
                }, None => {
                    a href=(post_link) {
                        img.p src=(TEXT_IMG);
                    }
                }
            }
            div {
                a.s[post.stickied] href=(post_link) {
                    @if post.stickied {"📌 "} (post.name)
                }
                .m{
                    "by "
                    a.u href=(creator_link) {
                        (post.creator_name)
                    }
                    " to "
                    a.l href=(community_link) {
                        (post.community_name)
                    }
                    div {
                        "up " 
                        (post.upvotes) 
                        " down " 
                        (post.downvotes)
                        " " 
                        a href={(post_link) " ✉ " (post.number_of_comments)}
                        {" "} 
                        (simple_duration(now, post.published))
                    }
                }
            }
        }
    }
}

fn comment_header_markup(
    instance: &str,
    comment: &CommentData,
    post_creator_id: Option<i32>,
    highlight_id: Option<i32>,
    now: &NaiveDateTime,
) -> Markup {
    html! {
        p.ch.h[Some(comment.id) == highlight_id] {
            a.u href={"/i/" (instance) "/u/" (comment.creator_name)} {
                (comment.creator_name)
            }
            @if let Some(pcid) = post_creator_id {
                @if pcid == comment.creator_id {
                    span.b { ("creator") }
                }
            }

            " ϟ" (comment.score)
            a href={"/i/" (instance) "/post/" (comment.post_id) "/comment/" (comment.id)} {
                " ⚓ "
            }

            (simple_duration(now, comment.published))
        }
    }
}

fn comment_markup(
    instance: &str,
    comment: &CommentData,
    post_creator_id: Option<i32>,
    highlight_id: Option<i32>,
    now: &NaiveDateTime,
    children: Option<Markup>,
) -> Markup {
    html! {
        (comment_header_markup(instance, comment, post_creator_id, highlight_id, now))

        @if children.is_some() {
            input.c type="checkbox";
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
fn comment_tree_markup(
    instance: &str,
    comments: &[CommentData],
    post_creator_id: i32,
    comment_parent_id: Option<i32>,
    depth: i32,
    highlight_id: Option<i32>,
    now: &NaiveDateTime,
) -> Markup {
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

fn pagebar_markup(paging_params: Option<&InstancePageParam>) -> Markup {
    html! {
        .pb {
            form {
                (sort_markup(paging_params))
                // @if let Some(PagingParams {p: Some(page), ..}) = paging_params {
                //     input type="hidden" name="p" value=(page);
                // }
                (limit_size_markup(paging_params))
                input type="submit" value="Apply";
            }

            div {
                @if let Some(InstancePageParam {page: Some(page), ..}) = paging_params {
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
                (limit_size_markup(paging_params))

                select name="t" {
                    @if let Some(ref type_) = search_params.content_type {
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

                @if let Some(ref community) = search_params.community_name {
                    input type="text" name="c" placeholder="Community" value=((community));
                } @else {
                    input type="text" name="c" placeholder="Community";
                }

                input type="submit" value="Apply";
            }

            div {
                @if let Some(InstancePageParam {page: Some(page), ..}) = paging_params {
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

fn sort_markup(paging_params: Option<&InstancePageParam>) -> Markup {
    html! {
        select name="sort" {
            @if let Some(InstancePageParam {sort: Some(sort), ..}) = paging_params {
                option selected?[sort==&lemmy_api_common::lemmy_db_schema::SortType::Hot] value="Hot" {"Hot"}
                option selected?[sort==&lemmy_api_common::lemmy_db_schema::SortType::Active] value="Active" {"Active"}
                option selected?[sort==&lemmy_api_common::lemmy_db_schema::SortType::New] value="New" {"New"}
                option selected?[sort==&lemmy_api_common::lemmy_db_schema::SortType::TopDay] value="TopDay" {"Day"}
                option selected?[sort==&lemmy_api_common::lemmy_db_schema::SortType::TopWeek] value="TopWeek" {"Week"}
                option selected?[sort==&lemmy_api_common::lemmy_db_schema::SortType::TopMonth] value="TopMonth" {"Month"}
                option selected?[sort==&lemmy_api_common::lemmy_db_schema::SortType::TopYear] value="TopYear" {"Year"}
                option selected?[sort==&lemmy_api_common::lemmy_db_schema::SortType::TopAll] value="TopAll" {"All"}
            } @else {
                option value="Hot" {"Hot"}
                option value="Active" {"Active"}
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

fn limit_size_markup(paging_params: Option<&InstancePageParam>) -> Markup {
    html! {
        select name="limit" {
            @if let Some(InstancePageParam {limit: Some(limit), ..}) = paging_params {
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

fn default_sort_markup(paging_params: Option<&InstancePageParam>) -> Markup {
    html! {
        @if let Some(InstancePageParam {sort: Some(sort), ..}) = paging_params {
            input type="hidden" name="sort" value=((sort));
        }
    }
}

fn default_limit_markup(paging_params: Option<&InstancePageParam>) -> Markup {
    html! {
        @if let Some(InstancePageParam {limit: Some(limit), ..}) = paging_params {
            input type="hidden" name="limit" value=((limit));
        }
    }
}

fn default_query_markup(search_params: Option<&SearchParams>) -> Markup {
    html! {
        @if let Some(SearchParams {query: Some(query), ..}) = search_params {
            input type="hidden" name="q" value=((query));
        }
    }
}

fn default_type_markup(search_params: Option<&SearchParams>) -> Markup {
    html! {
        @if let Some(SearchParams {content_type: Some(type_), ..}) = search_params {
            input type="hidden" name="t" value=((type_));
        }
    }
}

fn default_community_markup(search_params: Option<&SearchParams>) -> Markup {
    html! {
        @if let Some(SearchParams {community_name: Some(community), ..}) = search_params {
            input type="hidden" name="c" value=((community));
        }
    }
}

fn ends_with_any(s: String, suffixes: &'static [&'static str]) -> bool {
    return suffixes
        .iter()
        .any(|&suffix| s.to_lowercase().ends_with(suffix));
}

fn simple_duration(now: &NaiveDateTime, record: NaiveDateTime) -> String {
    let seconds = now.signed_duration_since(record).num_seconds();
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m", now.signed_duration_since(record).num_minutes())
    } else if seconds < 86400 {
        format!("{}h", now.signed_duration_since(record).num_hours())
    } else if seconds < 2629746 {
        format!("{}d", now.signed_duration_since(record).num_days())
    } else if seconds < 31556952 {
        format!("{}M", now.signed_duration_since(record).num_weeks() / 4)
    } else {
        format!("{}Y", now.signed_duration_since(record).num_weeks() / 52)
    }
}

// Custom markdown to HTML
fn mdstr_to_html(text: &str) -> Markup {
    let parser = ImageSwapper::new(Parser::new(text));
    let mut html_output = String::new();
    pchtml::push_html(&mut html_output, parser);
    PreEscaped(html_output)
}

struct ImageSwapper<'a, I> {
    iter: I,
    image_title: Option<CowStr<'a>>,
}

impl<'a, I> ImageSwapper<'a, I> {
    fn new(iter: I) -> Self {
        ImageSwapper {
            iter,
            image_title: None,
        }
    }
}

impl<'a, I> Iterator for ImageSwapper<'a, I>
where
    I: ::std::iter::Iterator<Item = Event<'a>>,
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut title = None;
        ::std::mem::swap(&mut self.image_title, &mut title);

        match title {
            None => self.iter.next().map(|event| match event {
                Event::Start(Tag::Image(linktype, url, title)) if title.is_empty() => {
                    self.image_title = Some(url.clone());
                    Event::Start(Tag::Link(linktype, url, title))
                }
                Event::Start(Tag::Image(linktype, url, title)) => {
                    self.image_title = Some(title.clone());
                    Event::Start(Tag::Link(linktype, url, title))
                }
                Event::End(Tag::Image(linktype, url, title)) => {
                    Event::End(Tag::Link(linktype, url, title))
                }
                _ => event,
            }),
            Some(title) => {
                self.image_title = None;
                Some(Event::Text(title))
            }
        }
    }
}
