use crate::components::code_frame::*;
use crate::components::code_icons::*;
use leptos::*;

#[component]
pub fn Python() -> impl IntoView {
    let projects = vec![
        Project {
            name: "DetectChatGPT",
            link: "https://github.com/armaan-rashid/detect-chatgpt",
            img: "ProjectImages/DetectChatGPT.png",
            alt: "A picture of some text perturbations.",
            height: 198,
            width: 1894,
            description: "There are more advanced detection methods out there today, but this project, created in the heady early days of ChatGPT with <a href=\"https://profiles.stanford.edu/196008\">Julia Park</a>, was an effective no-training-required method to detect if a random piece of text was generated by ChatGPT or not. It’s an extension of <a href=\"https://ericmitchell.ai/detectgpt/\">Eric Mitchell’s DetectGPT</a>.
            It works by subtly distorting or perturbing passages of text (see the example above!) numerous times, and then asking various LLMs how ‘probable’ they think the perturbed text is compared to the original. This arose in response to the fact that at the time there was no way of accessing ChatGPT’s internal probabilities. (If this is all gibberish to you <a href=\"https://www.lakera.ai/blog/large-language-models-guide\">this</a> is a great intro to the basics on how LLMs work.)
            This requires no training, but a lot of compute for the inference on these often big LLMs. (That said, LLMs seem to get smaller every day.) However, it did work pretty well, and you can see all the results in our paper and even run experiments yourself."
        }
    ];
    view! {<CodePage lang=LanguageIcon::Python projects=projects/>}
}