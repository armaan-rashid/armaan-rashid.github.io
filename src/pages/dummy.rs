use crate::components::code_frame::*;
use crate::components::code_icons::*;
use leptos::{ev::EventHandler, *};
use web_sys::MouseEvent;

static haskell_project: Project =
    Project {
        name: "BNFCSwift",
        link: "https://github.com/armaan-rashid/BNFCSwift",
        img: "ProjectImages/BNFCSwift.png",
        description: "The awesome BNFC program generates parsers for languages that are specified by grammar rules like the ones shown above. You can give just these simple rules in a nice syntax and BNFC can generate parsers for you in Haskell, Agda, C++, even Tree-sitter grammars.
        @filbertphang and I added a backend for this in Swift’s ultra-fast Lotsawa parser. It hasn’t been merged yet into the main BNFC codebase but you can import it as a separate module  which depends on BNFC if you want to use it."
    };

#[component]
pub fn CodeIcon(
    lang: LanguageIcon,
    #[prop(default = "#D9D9D9")] fill: &'static str,
    #[prop(optional)] stroke: &'static str,
) -> impl IntoView {
    let LanguageIconData { width, height, .. } = lang.icon_data();
    let s = "60%";
    view! {
            <svg //width=s
                 //height=s
                 viewBox={format!("0 0 {} {}", width, height)}
                 fill={fill}
                 stroke={stroke}
                 xmlns="http://www.w3.org/2000/svg">
                 {lang.component()}
            </svg>
    }
}

#[component]
pub fn Dummy() -> impl IntoView {
    view! {
        <div class="flex flex-col lg:flex-row md:justify-between bg-haskelldark h-dvh w-dvh p-6">
            <div class="flex flex-col min-h-0 min-w-lg md:justify-between justify-around">
                <CodeIcon lang=LanguageIcon::Haskell />
                <a href="/code"><CodeArrow/></a>
            </div>
            <ProjectSlider lang=LanguageIcon::Haskell projects={vec![haskell_project.clone()]}/>
        </div>
    }
}
