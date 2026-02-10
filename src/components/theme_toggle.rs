use crate::utils::theme::set_theme;
use leptos::prelude::*;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    view! {
        <div class="theme-toggle">
            <button on:click=move |_| set_theme("light")>"Light"</button>
            <button on:click=move |_| set_theme("dark")>"Dark"</button>
            <button on:click=move |_| set_theme("dracula")>"Dracula"</button>
        </div>
    }
}
