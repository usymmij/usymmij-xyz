use leptos::prelude::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    // TODO: SAD FACE public/sad.png
    view! {
        <h1>{"???"}
            <br /> {"Page not found :("}
        </h1>
        <br />

    }
}
