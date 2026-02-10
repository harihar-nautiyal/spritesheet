use leptos::prelude::*;

pub mod components;
pub mod utils;
pub mod models;
pub mod modules;

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
