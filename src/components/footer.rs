use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    pub is_playing: bool,
    pub on_toggle: Callback<()>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let handle_click = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_| on_toggle.emit(()))
    };

    html! {
        <footer>
            <button
                onclick={handle_click}
            >
                {
                    if props.is_playing {
                        "Pause"
                    } else {
                        "Resume"
                    }
                }
            </button>
        </footer>
    }
}
