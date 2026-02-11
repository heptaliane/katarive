use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub on_fetch: Callback<String>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let url = use_state(String::new);

    let handle_change = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            let inp: HtmlInputElement = e.target_unchecked_into();
            url.set(inp.value());
        })
    };
    let handle_click = {
        let url = url.clone();
        let on_fetch = props.on_fetch.clone();
        Callback::from(move |_| on_fetch.emit((*url).clone()))
    };

    html! {
        <header>
            <div class="header-inner">
                <input
                    type="text"
                    placeholder="Enter document URL ..."
                    oninput={handle_change}
                    value={(*url).clone()}
                />
                <button
                    onclick={handle_click}
                >
                    {"Fetch"}
                </button>
            </div>
        </header>
    }
}
