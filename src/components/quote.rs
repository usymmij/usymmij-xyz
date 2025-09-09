use leptos::prelude::*;

#[component]
pub fn Quote() -> impl IntoView {
    view! {
        <div class="quotecontainer">
            <div class="quotespacer"/>
            <div class="quote">
                {"«Malheureusement, ce qui est peu reconnu, c'est que les livres scientifiques les plus valables sont ceux où l'auteur indique clairement ce qu'il ne sait pas.»"}
            <br/>
                {"\"Unfortunately, what is little recognized is that the most valuable
                scientific books are those in which the author clearly states what he does
                not know.\""} 

            <div class="quotename">
                {"- Evariste Galois"}
            </div>
            </div>
            <div class="quotespacer"/>
        </div>
    }
}
