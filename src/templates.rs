use maud::{html, Markup};

use crate::lemmy_api::{PostView, PostList, PostDetail, CommentView};

const MEDIA_EXT: &[&str] = &[".png", "jpg", ".jpeg", ".gif"];

pub fn redirect_page(instance: String) -> Markup {
    html! {
        meta charset="utf8" {}
        meta content={"0;URL='" "/" (instance) "'"} http-equiv="refresh" {}
        link rel="stylesheet" href="/style.css" {}
    }
}


pub fn post_markup(instance: &String, post: &PostView) -> Markup {
    html!{
        p class="cell score" { (post.score) }
        @match &post.url {
            Some(url) => {
                a class="cell" href={ (url) } {
                    img class="preview" src={
                        @if ends_with_any(url.clone(), MEDIA_EXT) {
                            "/media.svg"
                        } @else {
                            "/link.svg"
                        }
                    } {}
                }
            }, None => {
                a class="cell" href={"/" (instance) "/post/" (post.id )} {
                    img class="preview" src={"/text.svg"} {}
                }
            }
        }
        div class="cell" {
            a class="title" href={"/" (instance) "/post/" (post.id )} {
                (post.name)
            }
            div class="mute"{
                "by "
                a class="username" href={"/" (instance) "/u/" (post.creator_name) }{
                    (post.creator_name)
                }
                " to "
                a class="community" href= {"/" (instance) "/c/" (post.community_name)} {
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
        meta charset="utf8" {}
        meta name="viewport" content="width=480px, user-scalable=no" {}
        meta name="theme-color" content="#222" {}
        link rel="stylesheet" href="/style.css" {}
        @for post in &post_list.posts {
            div { (post_markup(instance, post)) }
            hr {}
        }
    }
}


pub fn comment_markup(instance: &String, comment: &CommentView, post_creator_id: i32) -> Markup {
    html! {
        p class="mute" {
            a class="username" href={"/" (instance) "/u/" (comment.creator_name)} {
                (comment.creator_name)
            }
            @if post_creator_id == comment.creator_id {
                " " span class="badge" { ("creator") }
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

/*
Build comment chains from no parent, look for children, loop until no children, loop until all comments added

Make a mutable copy of comment vec and delete
*/
pub fn post_page(instance: &String, mut post_detail: &PostDetail) -> Markup {
    html! {
        meta charset="utf8" {}
        meta name="viewport" content="width=480px, user-scalable=no" {}
        meta name="theme-color" content="#222" {}
        link rel="stylesheet" href="/style.css" {}
        (post_markup(instance, &post_detail.post))

        @if let Some(body) = &post_detail.post.body {
            p { (body)}
        }
        
        @for mut comment in post_detail.comments.iter() {
            (comment_markup(instance, comment, post_detail.post.creator_id))
            // Build from root to children, remove_item as you go
        }
        
    }
}

pub fn user_view() -> Markup {
    html!{}
}

pub fn communities_view() -> Markup {
    html!{}
}

fn ends_with_any(s: String, suffixes: &'static [&'static str]) -> bool {
    return suffixes.iter().any(|&suffix| s.to_lowercase().ends_with(suffix));
}