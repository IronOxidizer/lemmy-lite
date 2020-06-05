use maud::{html, Markup};

use crate::lemmy_api::{PostList};

pub fn redirect(instance: String) -> Markup {
    html! {
        meta content={"0;URL='" (instance) "'"} http-equiv="refresh" charset="utf8" {}
        link rel="stylesheet" href="/style.css" {}
    }
}

pub fn post_list_view(instance: &String, post_list: PostList) -> Markup {
    html! {
        meta charset="utf8" {}
        link rel="stylesheet" href="/style.css" {}
        @for post in post_list.posts {
            div {
                div class="cell score" { (post.score) }
                div class="cell" {
                    @match post.url {
                        Some(url) => {
                            a href={ (url) } {
                                img class="preview" src={"/link.svg"} {}
                            }
                        }, None => {
                            a {
                                img class="preview" src={"/text.svg"} {}
                            }
                        }
                    }
                }
                div class="cell" {
                    div {
                        a href={(instance) "/post/" (post.id )} {
                            h4 {
                                (post.name)
                            }
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