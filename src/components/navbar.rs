use leptos::*;

#[component]
pub fn MenuIcon() -> impl IntoView {
    view! {
        <svg class="w-8 h-6">
            <g class="hover:fill-black fill-stone-400">
            {[0, 2, 4].into_iter()
                .map(|i| view! {<MenuLine cy=i * 4/>})
                .collect_view()}
            </g>
        </svg>
    }
}

#[component]
pub fn MenuLine(#[prop(optional)] cx: i32, #[prop(optional)] cy: i32) -> impl IntoView {
    view! {
        <rect width="2rem"
              height="0.25rem"
              rx="2.5" x={cx} y={cy}></rect>
    }
}

#[component]
pub fn SideBar(
    links: Vec<(&'static str, &'static str)>,
    side_on: WriteSignal<bool>,
    selected: usize,
) -> impl IntoView {
    view! {
        <div class="flex flex-col bg-black sidebar">
        {links.into_iter()
              .enumerate()
              .map(|(i, (name, route))| view! {
                  <div class="flex flex-row gap-6">
                  <SideBarSelector side_on=side_on filled={i == selected}/>
                  <SideBarLink name=name route=route/>
                  </div>
              })
              .collect_view()}
        </div>
    }
}

#[component]
pub fn SideBarSelector(side_on: WriteSignal<bool>, filled: bool) -> impl IntoView {
    view! {
        <svg width="40" height="40"
             class=format!("hover:fill-black {}",
                            if filled {"fill-background"} else {"fill-black"})
             on:click=move |_| side_on(false)
        >
            <circle r="5" cx="20" cy="25"></circle>
        </svg>
    }
}

#[component]
pub fn SideBarLink(name: &'static str, route: &'static str) -> impl IntoView {
    view! {
        <div class="text-background text-lg font-light font-lexend p-3 hover:underline">
            <a href={route}>{name}</a>
        </div>
    }
}

#[component]
pub fn NavBarMain(links: Vec<(&'static str, &'static str)>) -> impl IntoView {
    view! {
        <div class="p-6 flex flex-col lg:flex-row flex-wrap justify-between gap-3">
        <PageName name="Armaan Rashid" link="/home"/>
        <div class="flex flex-col lg:flex-row flex-wrap items-start lg:items-end justify-around lg:gap-12">
        {links.into_iter()
              .map(|(name, route)| view! {<NavBarLink name=name route=route/>})
              .collect_view()}
        </div>
        </div>
    }
}

#[component]
pub fn NavBarLink(name: &'static str, route: &'static str) -> impl IntoView {
    view! {
        <div class="text-black text-lg font-light font-lexend hover:underline">
            <a href={route}>{name}</a>
        </div>
    }
}

#[component]
/// It's the page header. In big sans-serif.
pub fn PageName(name: &'static str, #[prop(optional)] link: &'static str) -> impl IntoView {
    view! {
            <div class="pagename">
                <a href={link}> {name} </a>
            </div>
    }
}
