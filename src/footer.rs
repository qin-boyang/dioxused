
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
            div { class: "pt-10 text-center text-gray-400 text-sm border-t",
                "Built with Dioxus 0.7.3 — The future of Rust UI."
            }
    }
}