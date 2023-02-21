use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <DarkModeToggle />
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

fn get_stored_pref() -> Option<bool> {
    let storage = window().local_storage().ok().flatten()?;
    let value = storage.get_item("dark").ok().flatten()?;
    Some(value=="true")
}

fn set_stored_pref(prefers_dark: bool) -> Option<()> {
    let storage = window().local_storage().ok().flatten()?;
    let _ = storage.set_item("dark", &prefers_dark.to_string());
    Some(())
}

#[component]
fn DarkModeToggle(cx: Scope) -> impl IntoView {
    let (prefers_dark, set_prefers_dark) = create_signal(cx, get_stored_pref().unwrap_or_default());

    let color_scheme = move || {
        if prefers_dark.get() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };

    let toggle_dark_mode = move |_| set_prefers_dark.update(|dark| *dark = ! *dark);
    
    create_effect(cx, move |_| {
        set_stored_pref(prefers_dark.get());
    });

    view ! { cx, 
        <Meta name="color-scheme" content=color_scheme />
        <button on:click=toggle_dark_mode>"Toggle Dark Mode"</button>
    }
}