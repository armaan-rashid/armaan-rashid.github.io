use crate::pages::code::haskell::Haskell;
use crate::pages::code::python::Python;
use crate::pages::code::rust::Rust;
use crate::pages::code::swift::Swift;
use leptos::*;

#[component]
pub fn Portfolio() -> impl IntoView {
    view! {
        <div id="rust">
            <Rust />
        </div>
        <div id="python">
            <Python />
        </div>
        <div id="haskell">
            <Haskell />
        </div>
        <div id="swift">
            <Swift />
        </div>
    }
}
