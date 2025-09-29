/// Template for new Yew components
///
/// To create a new component:
/// 1. Copy this file to `frontend/src/components/newcomponent.rs`
/// 2. Replace COMPONENT_NAME with actual component name
/// 3. Define the Props structure
/// 4. Implement the component logic
/// 5. Add to `components/mod.rs`

use yew::prelude::*;

/// Props for COMPONENT_NAME component
#[derive(Properties, PartialEq)]
pub struct ComponentNameProps {
    /// Example: Text to display
    pub text: String,

    /// Example: Click handler
    pub on_click: Option<Callback<MouseEvent>>,

    /// Example: Optional CSS classes
    pub classes: Option<Classes>,

    /// Example: Whether component is disabled
    pub disabled: bool,
}

/// COMPONENT_NAME component
#[function_component(ComponentName)]
pub fn component_name(props: &ComponentNameProps) -> Html {
    // State management example
    let counter = use_state(|| 0);

    // Event handler example
    let on_increment = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    // Effect example
    {
        let counter = counter.clone();
        use_effect_with(*counter, move |count| {
            web_sys::console::log_1(&format!("Counter updated: {}", count).into());
            || {}
        });
    }

    // CSS classes
    let mut classes = classes!("component-name");
    if let Some(extra_classes) = &props.classes {
        classes.extend(extra_classes.clone());
    }
    if props.disabled {
        classes.push("disabled");
    }

    html! {
        <div class={classes}>
            <h3>{"Component Name"}</h3>
            <p>{&props.text}</p>

            <div class="component-controls">
                <button
                    onclick={on_increment}
                    disabled={props.disabled}
                    class="btn btn-primary"
                >
                    {format!("Count: {}", *counter)}
                </button>

                {if let Some(on_click) = &props.on_click {
                    html! {
                        <button
                            onclick={on_click.clone()}
                            disabled={props.disabled}
                            class="btn btn-secondary"
                        >
                            {"Action"}
                        </button>
                    }
                } else {
                    html! {}
                }}
            </div>

            // Conditional rendering example
            {if *counter > 5 {
                html! {
                    <div class="alert">
                        {"Counter is high!"}
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

/// Default props implementation
impl Default for ComponentNameProps {
    fn default() -> Self {
        Self {
            text: "Default text".to_string(),
            on_click: None,
            classes: None,
            disabled: false,
        }
    }
}

// CSS styles for this component (add to styles.css)
/*
.component-name {
    background: #fafafa;
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    border: 1px solid #e0e0e0;
}

.component-name.disabled {
    opacity: 0.6;
    pointer-events: none;
}

.component-controls {
    display: flex;
    gap: 0.5rem;
    margin-top: 1rem;
}

.component-name .btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s ease;
}

.component-name .btn-primary {
    background: #8b4513;
    color: white;
}

.component-name .btn-secondary {
    background: #6c757d;
    color: white;
}

.component-name .btn:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.component-name .alert {
    background: #fff3cd;
    border: 1px solid #ffeaa7;
    color: #856404;
    padding: 0.5rem;
    border-radius: 4px;
    margin-top: 0.5rem;
}
*/