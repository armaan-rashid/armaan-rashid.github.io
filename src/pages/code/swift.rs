use crate::components::code_frame::*;
use crate::components::code_icons::*;
use leptos::*;

#[component]
pub fn Swift() -> impl IntoView {
    let projects = vec![
        Project {
            name: "ticket",
            link: "https://ticket.io",
            img: "ProjectImages/ticket.png",
            description: "Wow, a note-taking app! There’s definitely not enough of those! But ticket is for people (like me) who use these apps as further procrastination tools, not organisational ones, endlessly creating new notes that will go nowhere.
            ticket is simple: all notes, or tickets, are connected to some larger project. If you want to write a note you have to pull one of a finite number of tickets, and you have to put that note “on a branch”, giving it context.
            When you run out of tickets, you have to either wait, or organise your notes to get tickets back. You can fold notes together or rebase one branch of notes onto another. This is needlessly punishing for most people, but if you need restrictions, this might be for you. ticket is currently in closed beta with a selected group of power users (me and my friends with ADHD) but will be available soon."
        }
    ];
    view! {<CodePage lang=LanguageIcon::Swift projects=projects/>}
}
