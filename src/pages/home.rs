use crate::components::counter_btn::Button;
use crate::components::sections::profile::Profile;
use crate::components::sidebar::Sidebar;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"I messed something up :/ sorry"</h1>

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
                    <div class="section">
                        <Profile />
                    </div>
                    <div class="section">
                        <Profile />
                    </div>
                    <div class="section">
                        <Profile />
                    </div>
                    <div class="section">
                        <Profile />
                    </div>

                </div>
                <Sidebar/>
            </div>
        </ErrorBoundary>
    }
}
