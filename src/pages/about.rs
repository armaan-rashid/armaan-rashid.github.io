use crate::components::navbar::*;
use leptos::*;

static SIDEBAR_LINKS: [(&'static str, &'static str); 3] =
    [("About", "/about"), ("Code", "/code"), ("Home", "/")];

static ABOUT: [&str; 4] = [
    "Hey, I’m Armaan. I program sometimes. This website hosts what I’m working on or have finished. I plan to write on here about this work also.",
    "I studied a bunch of different things and then I studied computer science when I was an undergrad at Stanford. Right now I’m at Chalmers in Sweden.",
    "If you look at the Code page you might notice I like working in different programming languages in different paradigms. There are enough (wonderful)
    hedgehogs in this line of work and I’m happy to be a fox.",
    "If you have questions about anything on this site please feel free to contact me, my email is armaan [at] chalmers [dot] se."];

#[component]
pub fn About() -> impl IntoView {
    let (sidebar, side_on) = create_signal(false);

    view! {
        <div class="grid md:grid-cols-3">
            <div class="p-6 flex flex-col flex-wrap gap-6 w-full md:col-span-2">
                <PageName name="About"/>
                {ABOUT.into_iter()
                    .map(move |s| view! {<p class="copy"> {s} </p>})
                    .collect_view()}
            </div>
            <div class="p-6 justify-self-end md:col-span-1 md:col-start-3 row-span-full">
                <MenuIcon on:click=move |_| side_on(true)/>
            </div>
            <div class="fixed right-0 transition-transform"
                 class=("translate-x-0", {sidebar})
                 class=("translate-x-full", {move || !sidebar.get()})
            >
                <SideBar links={SIDEBAR_LINKS.to_vec()} side_on=side_on selected=0/>
            </div>
        </div>
    }
}
