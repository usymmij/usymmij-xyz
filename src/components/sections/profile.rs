use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <h3>Hi there!</h3>
        <div class="text">
            {"I'm currently a student at the "} // apostrophes :(
            <a href="https://www.ece.utoronto.ca/" target="_blank">
            University of Toronto
            </a>
            {" pursuing a MASc in ECE under the supervision of "}
            <a href="https://www.eecg.utoronto.ca/~mcj/" target="_blank">
            Mark C. Jeffrey
            </a>,
            working on ML accelerators.{" "}I previously also worked on closed form approaches for optimizers,
            and led teams in competitive robotics as a participant and later on as a mentor.
            <br/>
            {"
                
            "}



            <br/><br/><br/>
            {"Academic Interests: Computer Architecture, Parallel Computing, ML Accelerators, and Applications in Scientific Computing"}


        </div>
    }
}
