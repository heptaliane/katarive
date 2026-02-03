use yew::prelude::*;

use crate::components::{footer, header};

#[function_component(App)]
pub fn app() -> Html {
    let is_playing = use_state(|| false);

    let handle_fetch = {
        Callback::from(move |_url: String| {
            // TODO: Implement this
        })
    };
    let handle_playing_toggle = {
        let is_playing = is_playing.clone();

        Callback::from(move |_| {
            is_playing.set(!*is_playing);
        })
    };

    html! {
        <main>
            <header::Header
                on_fetch={handle_fetch}
            />
            <footer::Footer
                is_playing={*is_playing}
                on_toggle={handle_playing_toggle}
            />
        </main>
    }
}
