use maud::{html, Markup};
use crate::lemmy_api::{PostView, PostList, PostDetail, CommentView};

const MEDIA_EXT: &[&str] = &[".png", "jpg", ".jpeg", ".gif"];

#[inline(always)]
fn headers_markup() -> Markup {
    html! {
        meta charset="utf8";
        meta name="viewport" content="width=480px, user-scalable=no";
        meta name="theme-color" content="#222";
        link rel="stylesheet" href="/style.css";
    }
}

pub fn redirect_page(instance: String) -> Markup {
    html! {
        (headers_markup())
        meta content={"0;URL='" "/" (instance) "'"} http-equiv="refresh" {}
    }
}

pub fn post_markup(instance: &String, post: &PostView) -> Markup {
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

pub fn post_list_page(instance: &String, post_list: PostList) -> Markup {
    html! {
        (headers_markup())
        @for post in &post_list.posts {
            div { (post_markup(instance, post)) }
            hr;
        }
    }
}

pub fn comment_markup(instance: &String, comment: &CommentView, post_creator_id: i32) -> Markup {
    html! {
        p.mute {
            a.username href={"/" (instance) "/u/" (comment.creator_name)} {
                (comment.creator_name)
            }
            @if post_creator_id == comment.creator_id {
                " " span.badge { ("creator") }
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

// zstewart#2487@discord.rust-community-server
fn comment_tree_markup(instance: &String, comments: &[CommentView],
    post_creator_id: i32, comment_parent_id: Option<i32>, depth: i32) -> Markup{

    html! {
        @if depth == 0 {
            @for comment in comments.iter().filter(|c| c.parent_id == comment_parent_id) {
                (comment_markup(instance, comment, post_creator_id))
                (comment_tree_markup(instance, comments, post_creator_id, Some(comment.id), depth+1))
            }
        } @else {
            .branch style={"border-left:2px solid rgb(" ({(172-32*depth)%256}) ",83,83)"}{
                @for comment in comments.iter().filter(|c| c.parent_id == comment_parent_id) {
                    (comment_markup(instance, comment, post_creator_id))
                    (comment_tree_markup(instance, comments, post_creator_id, Some(comment.id), depth+1))
                }
            }
        }
    }
}

pub fn post_page(instance: &String, post_detail: PostDetail) -> Markup {
    html! {
        (headers_markup())
        (post_markup(instance, &post_detail.post))

        @if let Some(body) = &post_detail.post.body {
            p { (body)}
        }
        hr;
        
        (comment_tree_markup(instance, &post_detail.comments, post_detail.post.creator_id, None, 0))
    }
}
fn ends_with_any(s: String, suffixes: &'static [&'static str]) -> bool {
    return suffixes.iter().any(|&suffix| s.to_lowercase().ends_with(suffix));
}