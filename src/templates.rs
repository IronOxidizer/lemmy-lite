use maud::{html, Markup};

use crate::lemmy_api::{PostList};

const MEDIA_EXT: &[&str] = &[".png", "jpg", ".jpeg", ".gif"];

pub fn redirect(instance: String) -> Markup {
    html! {
        meta content={"0;URL='" (instance) "'"} http-equiv="refresh" charset="utf8" {}
        link rel="stylesheet" href="/style.css" {}
    }
}

pub fn post_list_view(instance: &String, post_list: PostList) -> Markup {
    html! {
        meta charset="utf8" {}
        meta name="viewport" content="width=480px, user-scalable=no" {}
        meta name="theme-color" content="#222" {}
        link rel="stylesheet" href="/style.css" {}
        @for post in post_list.posts {
            div {
                div class="cell score" { (post.score) }
                div class="cell" {
                    @match post.url {
                        Some(url) => {
                            a href={ (url) } {
                                img class="preview" src={
                                    @if ends_with_any(url, MEDIA_EXT) {
                                        "/media.svg"
                                    } @else {
                                        "/link.svg"
                                    }
                                } {}
                            }
                        }, None => {
                            a href={(instance) "/post/" (post.id )} {
                                img class="preview" src={"/text.svg"} {}
                            }
                        }
                    }
                }
                div class="cell" {
                    div {
                        a class="title" href={(instance) "/post/" (post.id )} {
                            (post.name)
                        }
                    }
                    div class="post_details"{
                        "by "
                        a class="username" href= {(instance) "/u/" (post.creator_name) }{
                            (post.creator_name)
                        }
                        " to "
                        a class="community" href= {(instance) "/c/" (post.community_name)} {
                            (post.community_name)
                        }
                        " • ˄ " (post.upvotes) " ˅ " (post.downvotes)
                    }
                }
            }
            hr {}
        }
    }
}

pub fn post_view() -> Markup {
    html!{}
}

pub fn comment_view() -> Markup {
    html!{}
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