use yew::prelude::*;
use yew_router::prelude::*;
use routes::{Route,switch};
mod freighter;
mod home;
mod routes;

#[function_component(App)]
pub fn app() -> Html {
    html! {
     <HashRouter>
     <Switch<Route> render = {switch} />
     </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
