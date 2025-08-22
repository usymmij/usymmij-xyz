use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar">
            <div class="name">
                <h3>"Jimmy Su"</h3>
            </div>
            <div class="links">
                <div class="extspacer"/>
                <div><a href="https://github.com/usymmij" target="_blank">
                   <img src="public/github-mark-white.svg"/>
                </a></div>
                <div class="inspacer"/>
                <div>
                   <img src="public/email.svg"/>
                </div>: usymmij
                <div>
                   <img class="emdomain" src="public/addr.png"/>
                </div>
                <div class="extspacer"/>
            </div>
        </div>

    }
}
