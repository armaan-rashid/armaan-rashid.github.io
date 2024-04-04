use crate::components::code_frame::*;
use crate::components::code_icons::*;
use leptos::*;

#[component]
pub fn Rust() -> impl IntoView {
    let projects = vec![Project {
        name: "implies",
        img: "ProjectImages/implies.png",
        link: "https://github.com/armaan-rashid/implies",
        alt: "A logic formula and its parse tree",
        width: 1774,
        height: 606,
        description: "A Rust library for dealing with logical formulas in a sane way, transforming them between easily written human representations and binary trees which are easy to manipulate unambiguously, like in the displayed example.
        The crate has two key features: it’s extremely generic, so it’s easy to start using the library for almost any logical (or otherwise) language with unary and binary operators. It’s also fast for formula manipulation, since it uses a zipper on the inside. A bunch of methods for manipulating the formulae are builtin to the crate. Implementing just one simple trait also gives free access to the string-to-formula parser. Thoroughly documented on <a href=\"https://docs.rs/implies/latest/implies/\">docs.rs</a> with examples."
    }];
    view! {<CodePage lang=LanguageIcon::Rust projects=projects/>}
}
