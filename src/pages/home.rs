use crate::components::quote::Quote;
use crate::components::sections::profile::Profile;
use crate::components::sections::projects::Projects;
use crate::components::sections::publications::Publications;
use crate::components::sidebar::Sidebar;
use crate::components::siteinfo::SiteInfo;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>{"I messed something up :/ sorry"}</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>

            }
        }>

        <div class="pagecontainer">
           <div class="sectioncontainer">
               <div class="top-spacer"/>
               <div class="section" id="bio">
                   <Profile />
               </div>

               // add when pub lol
               //<div class="section-spacer"/>
               //<div class="section" id="publications">
               //    <Publications />
               //</div>

               <div class="section-spacer"/>
               <div class="section" id="projects">
                   <Projects />
               </div>

            <Quote />

            <br/>
            <SiteInfo />

            </div>
            <Sidebar/>
        </div>
        </ErrorBoundary>
    }
}
