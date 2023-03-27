use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <div class="columns is-mobile is-centered">
            <div id="title" class="column is-three-quarters">
                <p class="title is-3 has-text-centered">{"Astrological Calendar Events"}</p>
                <p class="subtitle is-5 has-text-centered">{"Generate calendar events so you never miss a moonrise"}</p>
            </div>
        </div>
    }
}
