use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Header() -> Element {
    rsx! {
            // Navigation Link
            Link {
                to: Route::Home {},
                class: "text-blue-600 hover:text-blue-800 font-medium mb-8 block transition-colors",
                "← Back Home"
            }
    }
}