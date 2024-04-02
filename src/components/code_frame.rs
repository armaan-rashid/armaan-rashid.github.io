use crate::components::code_icons::*;
use leptos::*;

#[component]
pub fn CodeArrow() -> impl IntoView {
    view! {
        <svg width="25%" height="25%" viewBox="0 0 64 90" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M60.5 3L5.5 44L60.5 88.5" stroke="#D9D9D9" stroke-width="6" stroke-linecap="round"/>
        </svg>
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Project {
    pub name: &'static str,
    pub link: &'static str,
    pub img: &'static str,
    pub description: &'static str,
}

#[component]
pub fn CodePage(lang: LanguageIcon, projects: Vec<Project>) -> impl IntoView {
    view! {
    <div class=format!("flex flex-col lg:flex-row md:justify-between h-dvh w-dvh p-6 {}", lang.bg_color())>
        <div class="flex flex-col min-h-0 min-w-lg md:justify-between justify-around">
            <CodeIcon lang=lang />
            <a class="hover:fill-black" href="/code"><CodeArrow/></a>
        </div>
        <ProjectSlider lang=lang projects=projects/>
    </div>
    }
}

#[component]
pub fn ProjectSlider(projects: Vec<Project>, lang: LanguageIcon) -> impl IntoView {
    let num_panels = projects.len();
    let (get_active_panel, active_panel) = create_signal::<usize>(0);
    view! {
        <div class="flex flex-col max-w-2xl">
            <div class="flex flex-row overflow-x-hidden transition-transform">
                {projects.into_iter()
                        .map(|p| view! {<ProjectPanel p=p/>})
                        .collect_view()}
            </div>
            <div class="bg-codeview grow">
            <PearlSlider lang=lang num_panels=num_panels active_panel=active_panel/>
            </div>
        </div>
    }
}

#[component]
pub fn ProjectPanel(p: Project) -> impl IntoView {
    let Project {
        name,
        link,
        img,
        description,
    } = p;
    view! {
        <div class="bg-codeview flex flex-col gap-6 p-6 max-h-xl">
            <a href={link}><h1 class="hover:underline font-lexend font-normal text-3xl text-light">{name}</h1></a>
            <img src={img}/>
            <p class="copy text-light overflow-scroll">{description}</p>
        </div>
    }
}

#[component]
pub fn PearlSlider(
    lang: LanguageIcon,
    num_panels: usize,
    active_panel: WriteSignal<usize>,
) -> impl IntoView {
    let hue = lang.fill_color();
    view! {
        <div class="flex flex-row justify-center gap-6 bg-pearl-string">
            {(0..num_panels).into_iter()
                .map(|i| view! {<Pearl on:click=move |_| active_panel(i) color_class=hue/>})
                .collect_view()}
        </div>
    }
}

#[component]
pub fn Pearl(color_class: &'static str) -> impl IntoView {
    view! {
        <svg width="40" height="40"
             class=format!("hover:fill-black {}", color_class)
        >
            <circle r="5" cx="20" cy="25"></circle>
        </svg>
    }
}
