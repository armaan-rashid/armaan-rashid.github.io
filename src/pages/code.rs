use crate::components::code_icons::*;
use crate::components::navbar::*;
use leptos::*;

pub mod haskell;
pub mod python;
pub mod rust;
pub mod swift;

static SIDEBAR_LINKS: [(&'static str, &'static str); 3] =
    [("About", "/about"), ("Code", "/code"), ("Home", "/")];

#[component]
pub fn Code() -> impl IntoView {
    let (sidebar, side_on) = create_signal(false);
    let (get_lang_name, lang_name) = create_signal("");
    view! {
        <div class="grid md:grid-cols-3">
            <LanguageStack lang_name=lang_name />
            <p class="font-lexend text-center text-3xl font-normal fixed top-1/2 right-1/3">
                {get_lang_name}
            </p>
            <div class="p-6 justify-self-end md:col-span-1 md:col-start-3 row-span-full">
                <MenuIcon on:click=move |_| side_on(true)/>
            </div>
            <div class="fixed right-0 transition-transform"
                 class=("translate-x-0", {sidebar})
                 class=("translate-x-full", {move || !sidebar.get()})
            >
                <SideBar links={SIDEBAR_LINKS.to_vec()} side_on=side_on selected=1/>
            </div>

        </div>
    }
}
