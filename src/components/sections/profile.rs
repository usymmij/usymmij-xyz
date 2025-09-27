use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <h3>Hi there!</h3>
        <div class="text">
            {"I'm Jimmy, and I'm currently a student at the "} // apostrophes :(
            <a href="https://www.ece.utoronto.ca/" target="_blank">
            {"University of Toronto"}
            </a>
            {" pursuing a MASc in ECE under the supervision of "}
            <a href="https://www.eecg.utoronto.ca/~mcj/" target="_blank">
            {"Mark C. Jeffrey"}
            </a>
            {". My current research focus is on speculative techniques for transformer inference. I previously also worked on closed form optimization approaches, and led teams in competitive robotics as a participant and later on as a mentor."}
            <br/>

            <br/><br/><br/>
            {"Academic Interests: Computer Systems/Architecture, Parallel Computing, and their applications in Machine Learning and Scientific Workloads"}

        </div>
    }
}
