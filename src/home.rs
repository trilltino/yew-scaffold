use crate::freighter::connect_and_get_address;
use yew::prelude::*;

#[function_component(Home)]
pub fn test_freighter() -> Html {
    let address = use_state(|| None::<String>);

    let onclick = {
        let address = address.clone();
        Callback::from(move |_| {
            yew::platform::spawn_local({
                let address = address.clone();
                async move {
                    match connect_and_get_address().await {
                        Ok(a) => address.set(Some(a)),
                        Err(e) => web_sys::console::error_1(&e),
                    }
                }
            });
        })
    };

    html! {
        <div>
            <button onclick={onclick}>{"Connect Freighter"}</button>
            {
                if let Some(a) = &*address {
                    html! { <p>{format!("Address: {}", a)}</p> }
                } else { html!{} }
            }
        </div>
    }
}
