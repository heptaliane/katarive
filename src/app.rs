use yew::prelude::*;

use crate::components::{error_bar, footer, header};
use crate::tauri::fetch_document;

#[function_component(App)]
pub fn app() -> Html {
    let is_playing = use_state(|| false);
    let lines = use_state(Vec::<String>::new);
    let errors = use_state(Vec::<AttrValue>::new);

    let handle_fetch = {
        let lines = lines.clone();
        let errors = errors.clone();
        Callback::from(move |url: String| {
            if !url.is_empty() {
                let lines = lines.clone();
                let errors = errors.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_document(url).await {
                        Ok(docs) => {
                            log::info!("Current document: {:?}", docs.title);
                            lines.set(docs.body);
                        }
                        Err(err) => {
                            log::error!("Failed to fetch document: {:?}", err);
                            let mut new_errors = (*errors).clone();
                            new_errors.push(AttrValue::from(err));
                            errors.set(new_errors);
                        }
                    }
                });
            }
        })
    };
    let handle_errors_change = {
        let errors = errors.clone();
        Callback::from(move |new_errors: Vec<AttrValue>| errors.set(new_errors))
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
            <error_bar::ErrorBar
                messages={(*errors).clone()}
                onchange={handle_errors_change}
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
