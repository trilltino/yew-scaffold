use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Navigation)]
pub fn navigation() -> Html {
    let location = use_location().unwrap();
    let current_route = &location.path();

    html! {
        <nav class="nav">
            <div class="nav-content">
                <Link<Route> to={Route::Home} classes="nav-logo">
                    {"🚀 Stellar dApp"}
                </Link<Route>>

                <ul class="nav-links">
                    <li>
                        <Link<Route>
                            to={Route::Home}
                            classes={if *current_route == "/" { "nav-link active" } else { "nav-link" }}
                        >
                            {"Home"}
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route>
                            to={Route::About}
                            classes={if *current_route == "/about" { "nav-link active" } else { "nav-link" }}
                        >
                            {"About"}
                        </Link<Route>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}