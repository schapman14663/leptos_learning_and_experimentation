use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

// Marks a function as a re-usable component
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        // Define Event Listener With on
        <button on:click=move |_| { *set_count.write() += 1; }>
        // Text nodes wrapped in quotations
        "Click Me:"
        // Blocks include Rust code
        {count}
        </button>
        <p>
        "Double Count:"
        {move || count.get() * 2}
        </p>
    }
}
