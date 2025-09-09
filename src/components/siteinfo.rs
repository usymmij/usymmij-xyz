use leptos::prelude::*;

#[component]
pub fn SiteInfo() -> impl IntoView {
    view! {

    <div class="site-info">
        {"made with "}
        <a href="https://www.rust-lang.org/" target="_blank">
            <img src="./public/rustacean-flat-happy.svg"/>
        </a>
        {" + "}
        <a href="https://leptos.dev/" target="_blank">
            <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"/>
        </a>
    </div>
    }
}
