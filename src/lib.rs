use leptos::prelude::*;

pub mod components;
pub mod utils;

use crate::components::theme_toggle::ThemeToggle;

#[allow(non_snake_case)]
pub fn App() -> impl IntoView {
    view! {
        <div class="dracula">
            <ThemeToggle />
            <p>"Tool coming soon."</p>
        </div>
    }
}
