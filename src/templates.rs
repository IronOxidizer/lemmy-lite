use maud::{html, Markup};

use crate::lemmy_api::{PostList};

pub fn root() -> Markup {
    html! {
        form {
            input name="i" {}
            input type="submit" {}
        }
    }
}

pub fn redirect(instance: String) -> Markup {
    html! {
        meta content={"0;URL='" (instance) "'"} http-equiv="refresh" {}
    }
}

pub fn post_list_view(post_list: PostList) -> Markup {
    html! {
        @for post in post_list.posts {
            div {
                @match post.url {
                    Some(url) => {
                        a href={ (url) } { (post.name) }
                    }, None => {
                        a { (post.name) }
                    }
                }

                " by "

                (post.creator_name)
            }
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