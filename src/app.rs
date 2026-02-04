use yew::prelude::*;

use crate::components::{footer, header};

#[function_component(App)]
pub fn app() -> Html {
    let is_playing = use_state(|| false);
    let lines = use_state(Vec::<String>::new);

    let handle_fetch = {
        let lines = lines.clone();
        Callback::from(move |url: String| {
            // TODO: Set fetched data to lines
            lines.set(vec![url]);
        })
    };
    let handle_playing_toggle = {
        let is_playing = is_playing.clone();

        Callback::from(move |_| {
            is_playing.set(!*is_playing);
        })
    };

    html! {
        <div class="container">
            <header::Header
                on_fetch={handle_fetch}
            />
            <main>
                <article>
                {for (*lines).iter().map(|line| html!{<p>{line}</p>})}
                </article>
            </main>
            <footer::Footer
                is_playing={*is_playing}
                on_toggle={handle_playing_toggle}
            />
        </div>
    }
}
