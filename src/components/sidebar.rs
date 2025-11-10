use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar">
            <div class="name">
                <h3>{"Jimmy Su"}</h3>
                <div class="emaddr">
                    {"jimmy.su"}
                    <img class="emdomain" src="public/email.png"/>
                     /*
                    email domain photo generated with https://opentype.js.org/
                    1. Type in "@ email.domain" with desired TTF
                    2. save the image
                    3. use gimp or any other image editor
                        - crop to content
                        - Invert colour for white
                    4. export and use png
                    */
                </div>

            </div>
            <div class="links">
                <div class="extspacer"/>
                <div><a href="https://github.com/usymmij" target="_blank">
                   <img src="public/github-mark-white.svg"/>
                </a></div>

                <div class="inspacer"/>
                <div><a href="https://discuss.systems/@usymmij" target="_blank">
                   <img src="public/mastodon.svg"/>
                </a></div>

                // <div class="inspacer"/>
                // <div><a href="https://bsky.app/profile/usymmij.bsky.social" target="_blank">
                //    <img src="public/bluesky.png"/>
                // </a></div>

                <div class="inspacer"/>
                <div><a href="https://medium.usymmij.xyz" target="_blank">
                   <img src="public/medium.png"/>
                </a></div>


                <div class="inspacer"/>
                //<div><a href="https://typst.app/project/rqsaAJj7QQVKuC8nY4Y7e8" target="_blank">
                <div><a href="public/full-cv.pdf" target="_blank">
                    <img style="height: 1em" src="public/CV.png"/>
                </a></div>

                <div class="extspacer"/>
            </div>

            // TODO: Add section headers with animations
        </div>

    }
}
