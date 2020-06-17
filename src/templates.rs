use maud::{html, Markup};
use crate::lemmy_api::{PostView, PostList, PostDetail, CommentView, CommunityView, CommunityList, UserDetail};

const MEDIA_EXT: &[&str] = &[".png", "jpg", ".jpeg", ".gif"];

pub fn redirect_page(instance: String) -> Markup {
    html! {
        (headers_markup())
        meta content={"0;URL='" "/" (instance) "'"} http-equiv="refresh" {}
    }
}

pub fn communities_page(instance: &String, community_list: CommunityList) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance))
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
    }
}

pub fn post_list_page(instance: &String, post_list: PostList) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance))
        @for post in &post_list.posts {
            div { (post_markup(instance, post)) }
            hr;
        }
    }
}

pub fn post_page(instance: &String, post_detail: PostDetail) -> Markup {
    html! {
        (headers_markup())
        (navbar_markup(instance))
        (post_markup(instance, &post_detail.post))

        @if let Some(body) = &post_detail.post.body {
            p { (body)}
        }
        hr;
        
        (comment_tree_markup(instance, &post_detail.comments, post_detail.post.creator_id, None, 0, None))
    }
}

pub fn comment_page(instance: &String, comment: CommentView, post_detail: PostDetail) -> Markup {
    let mut comments = post_detail.comments;
    let comment_id = comment.id;
    comments.retain(|c| Some(c.id) == comment.parent_id ||
        c.id == comment_id ||
        c.parent_id == Some(comment.id));
    let parent = comments.iter().find(|c| Some(c.id) == comment.parent_id);

    html! {
        (headers_markup())
        (navbar_markup(instance))
        (post_markup(instance, &post_detail.post))

        @if let Some(body) = &post_detail.post.body {
            p { (body)}
        }
        hr;
        
        @match parent {
            Some(p) => (comment_tree_markup(instance, &comments, post_detail.post.creator_id, p.parent_id, 0, Some(comment_id))),
            None => (comment_tree_markup(instance, &comments, post_detail.post.creator_id, None, 0, Some(comment_id)))
        }
    }
}

pub fn user_page(instance: &String, user: UserDetail) -> Markup {
    html!{
        (headers_markup())
        (navbar_markup(instance))
        @for post in user.posts {
            div {(post_markup(instance, &post))}
        }
        @for comment in user.comments {
            (highlight_comment_markup(instance, &comment, None, None));
        }
    }
}

#[inline(always)]
fn headers_markup() -> Markup {
    html! {
        meta charset="utf8";
        meta name="viewport" content="width=480px, user-scalable=no";
        meta name="theme-color" content="#222";
        link rel="stylesheet" href="/style.css";
    }
}

fn navbar_markup(instance: &String) -> Markup {
    html! {
        #navbar {
            a href= {".."} {"Back"}
        
            a href= {"/" (instance) } {(instance)}
        
            a href= {"/" (instance) "/communities"} {"Communities"}
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

fn post_markup(instance: &String, post: &PostView) -> Markup {
    html!{
        p.cell.score { (post.score) }
        @match &post.url {
            Some(url) => {
                a.cell href={ (url) } {
                    img.preview src={
                        @if ends_with_any(url.clone(), MEDIA_EXT) {
                            "/media.svg"
                        } @else {
                            "/link.svg"
                        }
                    };
                }
            }, None => {
                a.cell href={"/" (instance) "/post/" (post.id )} {
                    img.preview src={"/text.svg"};
                }
            }
        }
        .cell {
            a.title href={"/" (instance) "/post/" (post.id )} {
                (post.name)
            }
            .mute{
                "by "
                a.username href={"/" (instance) "/u/" (post.creator_name) }{
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
            }
        }
    }
}

fn comment_markup(instance: &String, comment: &CommentView, post_creator_id: Option<i32>) -> Markup {
    return html! {
        .highlight {
            p.mute {
                a.username href={"/" (instance) "/u/" (comment.creator_name)} {
                    (comment.creator_name)
                }
                @if let Some(pcid) = post_creator_id {
                    @if pcid== comment.creator_id {
                        " " span.badge { ("creator") }
                    }
                }

                " • ϟ " (comment.score) 
                a href={"/" (instance) "/post/" (comment.post_id) "/comment/" (comment.id)} {
                    " • ⚓"
                }
            }
            
            div {
                (comment.content)
            }
        }
    }
}

pub fn highlight_comment_markup(instance: &String, comment: &CommentView, post_creator_id: Option<i32>, highlight_id: Option<i32>) -> Markup {
    if let Some(hid) = highlight_id {
        if comment.id == hid {
            return comment_markup(instance, comment, post_creator_id)
        }
    }

    comment_markup(instance, comment, post_creator_id)
}

// zstewart#2487@discord.rust-community-server
fn comment_tree_markup(instance: &String, comments: &[CommentView],
    post_creator_id: i32, comment_parent_id: Option<i32>, depth: i32, highlight_id: Option<i32>) -> Markup{

    html! {
        @if depth == 0 {
            @for comment in comments.iter().filter(|c| c.parent_id == comment_parent_id) {
                (highlight_comment_markup(instance, comment, Some(post_creator_id), highlight_id))
                (comment_tree_markup(instance, comments, post_creator_id, Some(comment.id), depth+1, highlight_id))
            }
        } @else {
            .branch style={"border-left:2px solid rgb(" ({172-64*((depth-1)%3)}) ",83,83)"}{
                @for comment in comments.iter().filter(|c| c.parent_id == comment_parent_id) {
                    (highlight_comment_markup(instance, comment, Some(post_creator_id), highlight_id))
                    (comment_tree_markup(instance, comments, post_creator_id, Some(comment.id), depth+1, highlight_id))
                }
            }
        }
    }
}

fn ends_with_any(s: String, suffixes: &'static [&'static str]) -> bool {
    return suffixes.iter().any(|&suffix| s.to_lowercase().ends_with(suffix));
}