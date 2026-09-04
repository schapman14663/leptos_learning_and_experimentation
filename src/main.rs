use leptos::{
    ev::SubmitEvent,
    html::{self, button},
    prelude::*,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
    mount_to_body(Inputs);
    mount_to_body(Uncontrolled);
    mount_to_body(Select);
    mount_to_body(Enums);
}

// Marks a function as a re-usable component
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let values = vec![0, 1, 2];
    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();
    // TODO: Possible use case for the available plates Vector for PL8M8
    let (name, set_name) = signal("Controlled".to_string());

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

        //Iteration
            // this will just render "012"
        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()} //Can also use .collect_view()
        </ul>

        <ul>{counter_buttons}</ul>

        <input type="text"
            on:input:target=move |ev| {
                set_name.set(ev.target().value());
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>

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
        <br/>
    }
}

#[component]
fn Inputs() -> impl IntoView {
    let (name, set_name) = signal("Serve".to_string());
    let email = RwSignal::new("".to_string());
    let favorite_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <input type="text"
            bind:value=(name, set_name)
        />
        <br/>
        <input type="email"
            bind:value=email
        />
        <br/>
        <label>
            "Please send me lots of spam email"
            <input type="checkbox"
                bind:checked=spam_me
            />
        </label>
        <br/>
        <fieldset>
            <legend>"Favorite_color"</legend>
            <label>
                "Red"
                <input
                    type="radio"
                    name="color"
                    value="red"
                    bind:group=favorite_color
                />
            </label>
             <label>
                "Green"
                <input
                    type="radio"
                    name="color"
                    value="green"
                    bind:group=favorite_color
                />
            </label>
             <label>
                "Blue"
                <input
                    type="radio"
                    name="color"
                    value="blue"
                    bind:group=favorite_color
                />
            </label>
        </fieldset>
        <p>"Your favorite color is " {favorite_color} "."</p>
        <p>"Your name is " {name}</p>
        <p>"Yor email is " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You'll receive cool bonus content!"</p>
        </Show>
    }
}

#[component]
fn Uncontrolled() -> impl IntoView {
    let (name, set_name) = signal("Uncontrolled".to_string());
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        // stop page from reloading
        ev.prevent_default();

        //extract value from input
        let value = input_element
            .get()
            //event handlers only fire after the view is mounted to the DOM so 'NodeRef' must be
            //'Some' type
            .expect("<input> should be mounted")
            //leptos::HtmlElement<html::Input> implements 'Deref'
            //to a 'web_sys::HtmlInputElement' so we can call
            //'HtmlInputElement::value()' to get the current value of the input
            .value();
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn Select() -> impl IntoView {
    let (value, set_value) = signal(0i32);
    view! {
        <select
            on:change:target=move |ev| {
            set_value.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || value.get().to_string()
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
        // a button that will cycle through the options
        <button on:click=move |_| set_value.update(|n| {
            if *n == 2 {
            *n = 0;
            } else {
            *n += 1;
            }
        })>
            "Next Option"
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

#[component]
fn Enums() -> impl IntoView {
    enum TestKind {
        TestKind1,
        TestKind2,
    }

    let (value, set_value) = signal(0);
    let mut enum_value = TestKind::TestKind1;
    view! {
        <button on:click=move |_| enum_value = TestKind::TestKind2>

        </button>
    }
}
