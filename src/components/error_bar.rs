use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorBarProps {
    pub messages: Vec<AttrValue>,
    pub onchange: Callback<Vec<AttrValue>>,
}

#[function_component(ErrorBar)]
pub fn error_bar(props: &ErrorBarProps) -> Html {
    html! {
        <div>
        {
            props.messages.iter().enumerate().map(|(i, msg)| {
                let handle_click = {
                    let onchange = props.onchange.clone();
                    let messages = props.messages.clone();
                    Callback::from(move |_| {
                        let mut messages = messages.clone();
                        messages.remove(i);
                        onchange.emit(messages);
                    })
                };

                html! {
                    <div class="error-banner">
                        <button
                            class="close-btn"
                            onclick={handle_click}
                        >
                            {"x"}
                        </button>
                        <span>{msg}</span>
                    </div>
                }
            }).collect::<Html>()
        }
        </div>
    }
}
