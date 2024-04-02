use crate::components::navbar::NavBarMain;
use leptos::*;

static PAGE_LINKS: [(&'static str, &'static str); 2] = [("About", "/about"), ("Code", "/code")];

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
        <NavBarMain links={PAGE_LINKS.to_vec()}/>
        </ErrorBoundary>
    }
}
