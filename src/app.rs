use yew::prelude::*;

use crate::components::{footer, header};
use crate::tauri::fetch_document;

#[function_component(App)]
pub fn app() -> Html {
    let is_playing = use_state(|| false);
    let lines = use_state(Vec::<String>::new);

    let handle_fetch = {
        let lines = lines.clone();
        Callback::from(move |url: String| {
            let lines = lines.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_document(&url).await {
                    Ok(docs) => {
                        lines.set(docs.body);
                    }
                    Err(err) => {
                        log::error!("[Error] {:?}", err);
                    }
                }
            });
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
