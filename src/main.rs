use leptos::{html::Progress, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

// Marks a function as a re-usable component
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        // Define Event Listener With on
        <button
            on:click=move |_| { *set_count.write() += 1; }
            // class: syntax reactively updates a single class
            // Set 'red' class when count is odd
            class:red=move || count.get() % 2 == 1
        >
        // Text nodes wrapped in quotations
        "Click Me:"
        // Blocks include Rust code
        {count}
        <br/>
        <ProgressBar max=50 progress=count/>
        <br/>
        <ProgressBar progress=Signal::derive(double_count)/>
        <br/>
        <ProgressBar progress=count/>
        </button>
        <button
            on:click=move |_| { *set_count.write() += 10 }
            // Individual CSS properties can be updated using 'style:'
            // set the style attribute
            //style="position: absolute"
            style:left=move || format!("{}px", count.get() + 100)
            //style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for the stylesheet to use
            // Sometimes you'll need tuple notation to set something due to
            // symbols in the CSS name
            style=("--columns", move || count.get().to_string())
        >
        "Click to Move"
        <ProgressBar progress=Signal::derive(double_count)/>
        </button>
        <p>
        "Double Count:"
        {double_count}
        </p>
        <br/>
        // Okay, lets try and emulate the buttons for PL8M8
        <button
            on:click=move |_| { *set_count.write() += 1 }
            style="position: left; width: 200px; height: 30px"
        >
        "Round Up"
        </button>
        <button
            on:click=move |_| { *set_count.write() += 1 }
            style="width: 200px; height: 30px"
        >
        "Round Down"
        </button>
        <button
            on:click=move |_| { *set_count.write() += 1 }
            style="position: right; width: 200px; height: 30px"
        >
        "Smart Round"
        </button>
        <br/>
        <button
            on:click=move |_| { *set_count.write() -= 10 }
            style="position: left; width: 300px; height: 30px"
        >
        "Metric"
        </button>
        <button
            on:click=move |_| { *set_count.write() -= 10 }
            style="position: right; width: 300px; height: 30px"
        >
        "Imperial"
        </button>
    }
}

#[component]
fn ProgressBar(
    //#[prop(optional)]
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=progress
        />
    }
}
