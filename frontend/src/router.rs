use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{AboutPage, BlendProtocol};
use crate::pages::LoginPage;
use crate::state::AppState;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/about")]
    About,
    #[at("/blend")]
    Blend,
}

/// Route switching logic with state
pub fn switch_with_state(state: yew::UseReducerHandle<AppState>) -> impl Fn(Route) -> Html {
    move |routes: Route| match routes {
        Route::Home => {
            html! { <crate::pages::HomePage state={state.clone()} /> }
        },
        Route::Login => {
            html! { <LoginPage state={state.clone()} /> }
        },
        Route::About => {
            html! { <AboutPage /> }
        },
        Route::Blend => {
            html! { <BlendProtocol /> }
        },
    }
}