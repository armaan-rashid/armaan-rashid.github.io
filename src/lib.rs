use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::about::About;
use crate::pages::code::haskell::Haskell;
use crate::pages::code::python::Python;
use crate::pages::code::rust::Rust;
use crate::pages::code::swift::Swift;
use crate::pages::code::Code;
use crate::pages::dummy::*;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        <Html lang="en" dir="ltr"/>
        <Stylesheet href="public/tailwind.css"/>
        // sets the document title
        <Title text="Armaan Rashid"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/home" view=Home/>
                <Route path="/about" view=About/>
                <Route path="/code" view=Code/>
                    <Route path="/code/haskell" view=Haskell/>
                    <Route path="/code/rust" view=Rust/>
                    <Route path="/code/swift" view=Swift/>
                    <Route path="/code/python" view=Python/>
                <Route path="/dummy" view=Dummy/>
                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}
