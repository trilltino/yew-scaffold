use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub dark_mode: bool,
    pub on_toggle_dark_mode: Callback<()>,
    pub connected_wallet: Option<String>,
}

#[function_component(Navigation)]
pub fn navigation(props: &NavigationProps) -> Html {
    let location = use_location().unwrap();
    let current_route = &location.path();

    html! {
        <nav class="nav">
            <div class="nav-content">
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
                    <li>
                        <Link<Route>
                            to={Route::Blend}
                            classes={if *current_route == "/blend" { "nav-link active" } else { "nav-link" }}
                        >
                            {"Blend"}
                        </Link<Route>>
                    </li>
                    {
                        if let Some(wallet) = &props.connected_wallet {
                            html! {
                                <>
                                    <li class="nav-user-info">
                                        <span class="user-badge">
                                            {&wallet[..6]}{"..."}{&wallet[wallet.len()-4..]}
                                        </span>
                                    </li>
                                    <li>
                                        <button
                                            class="logout-btn"
                                            onclick={Callback::from(move |_| {
                                                web_sys::window().unwrap().location().reload().ok();
                                            })}
                                        >
                                            {"Disconnect"}
                                        </button>
                                    </li>
                                </>
                            }
                        } else {
                            html! {
                                <li>
                                    <Link<Route>
                                        to={Route::Login}
                                        classes={if *current_route == "/login" { "nav-link login-btn active" } else { "nav-link login-btn" }}
                                    >
                                        {"Login / Sign Up"}
                                    </Link<Route>>
                                </li>
                            }
                        }
                    }
                    <li class="nav-dark-mode-item">
                        <DarkModeToggle
                            dark_mode={props.dark_mode}
                            on_toggle={props.on_toggle_dark_mode.clone()}
                        />
                    </li>
                </ul>
            </div>
        </nav>
    }
}

/// Dark mode toggle button component
#[derive(Properties, PartialEq)]
pub struct DarkModeToggleProps {
    pub dark_mode: bool,
    pub on_toggle: Callback<()>,
}

#[function_component(DarkModeToggle)]
pub fn dark_mode_toggle(props: &DarkModeToggleProps) -> Html {
    let on_click = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_| {
            on_toggle.emit(());
        })
    };

    let icon = if props.dark_mode { "Light" } else { "Dark" };
    let title = if props.dark_mode { "Switch to light mode" } else { "Switch to dark mode" };

    html! {
        <button
            class="dark-mode-toggle"
            onclick={on_click}
            title={title}
        >
            {icon}
        </button>
    }
}

