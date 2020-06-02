use maud::{html, Markup};

// This is static and would perform better if served statically by Nginx
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