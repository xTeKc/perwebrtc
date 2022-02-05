use perseus::Template;
use sycamore::prelude::{component, view, Html, SsrNode, View};

#[perseus::template(ChatPage)]
#[component(ChatPage<G>)]
pub fn chat_page() -> View<G> {
    view! {
        p { "Chat." }
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "Chat Page | Basic" }
    }
}



pub fn get_template<G: Html>() -> Template<G> {
    Template::new("chat").template(chat_page).head(head)
}