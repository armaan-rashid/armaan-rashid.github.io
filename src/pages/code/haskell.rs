use crate::components::code_frame::*;
use crate::components::code_icons::*;
use leptos::*;

#[component]
pub fn Haskell() -> impl IntoView {
    let projects = vec![Project {
        name: "BNFCSwift",
        link: "https://github.com/armaan-rashid/BNFCSwift",
        img: "ProjectImages/BNFCSwift.png",
        description: "The awesome BNFC program generates parsers for languages that are specified by grammar rules like the ones shown above. You can give just these simple rules in a nice syntax and BNFC can generate parsers for you in Haskell, Agda, C++, even Tree-sitter grammars.
        @filbertphang and I added a backend for this in Swift’s ultra-fast Lotsawa parser. It hasn’t been merged yet into the main BNFC codebase but you can import it as a separate module  which depends on BNFC if you want to use it."
    }];
    view! {<CodePage lang=LanguageIcon::Haskell projects=projects/>}
}

// view! {
//     <div class="flex flex-col lg:flex-row md:justify-between bg-haskelldark h-dvh w-dvh p-6">
//         <div class="flex flex-col min-h-0 min-w-lg md:justify-between justify-around">
//             <CodeIcon lang=LanguageIcon::Haskell />
//             <a href="/code"><CodeArrow/></a>
//         </div>
//         <ProjectSlider lang=LanguageIcon::Haskell projects={vec![haskell_project.clone()]}/>
//     </div>
// }
