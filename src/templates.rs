use maud::{html, Markup};

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

pub fn list_view() -> Markup {
    html!{}
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