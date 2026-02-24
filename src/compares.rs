use dioxus::prelude::*;
use crate::footer::Footer;
use crate::header::Header;
use crate::Route;

#[component]
pub fn Compares() -> Element {
    let my_article = build_article_content();
    rsx! {
        div { class: "p-8 max-w-4xl mx-auto font-sans text-gray-900",

            Header {}

            // Main Article Container
            article { class: "space-y-8",

                // Header Section
                div { class: "border-b pb-6",
                    h1 { class: "text-4xl font-bold mb-4 text-white",
                        {my_article.h1}
                    }
                    p { class: "text-lg text-gray-600 italic",
                        {my_article.p}
                    }
                }

                // Introduction
                p { class: "text-gray-700 leading-relaxed",
                    {my_article.intro_p_1}
                    strong {
                        {my_article.intro_p_2}
                    }
                    {my_article.intro_p_3}
                }

                for s in my_article.sections {
                    section { class: "p-6 {s.bg_color} border {s.border_color} rounded-xl shadow-sm",
                        h2 { class: "text-2xl font-bold {s.h2_color} mb-3",
                            {s.h2}
                        }
                        p { class: "text-gray-700 mb-4",
                            {s.p}
                        }
                        ul { class: "list-disc ml-6 space-y-2 text-gray-600",
                            for i in s.ul {
                                li { b { "{i.b}"} " {i.p}" }
                            }
                        }
                        section { class: "mt-4 font-semibold {s.footer_text_color}", {s.footer} }
                    }
                }

                Footer {}
            }
        }
    }
}

struct ArticleContent {
    h1: String,
    p: String,
    intro_p_1: String,
    intro_p_2: String,
    intro_p_3: String,
    sections: Vec<SectionContent>
}
impl ArticleContent {
    pub fn new(h1: &str, block: impl FnOnce(&mut Self)) -> Self {
        let mut article = Self {
            h1: h1.to_string(),
            p: String::new(),
            intro_p_1: String::new(),
            intro_p_2: String::new(),
            intro_p_3: String::new(),
            sections: Vec::new(),
        };
        block(&mut article);
        article
    }

    pub fn section(&mut self, bg: &str, border: &str, h2: &str, h2_color: &str, block: impl FnOnce(&mut SectionContent)) {
        let mut section = SectionContent {
            bg_color: bg.to_string(),
            border_color: border.to_string(),
            h2: h2.to_string(),
            h2_color: h2_color.to_string(),
            p: String::new(),
            ul: Vec::new(),
            footer: String::new(),
            footer_text_color: String::new(),
        };
        block(&mut section);
        self.sections.push(section);
    }
}
struct SectionContent {
    bg_color: String,
    border_color: String,
    h2: String,
    h2_color: String,
    p: String,
    ul: Vec<ListItemContent>,
    footer: String,
    footer_text_color: String,
}
impl SectionContent {
    pub fn item(&mut self, b: &str, p: &str) {
        self.ul.push(ListItemContent {
            b: b.to_string(),
            p: p.to_string(),
        });
    }
}
struct ListItemContent {
    b: String,
    p: String,
}

fn build_article_content() -> ArticleContent {
    let my_article = ArticleContent::new("Why Choose Dioxus?", |a| {
        a.p = "A Comparison of Modern Rust Ecosystems".to_string();
        a.intro_p_1 = "If you are building a web application in Rust, you are likely looking at three main contenders: ".to_string();
        a.intro_p_2 = "Dioxus, Leptos, and Yew".to_string();
        a.intro_p_3 = ". While they all benefit from Rust’s performance and safety, their philosophies differ significantly.".to_string();

        a.section("bg-blue-50",
                  "border-blue-100",
                  "1. Dioxus: The 'Full-Stack' Powerhouse",
                  "text-blue-900",
                  |s| {
                        s.p = "Dioxus is heavily inspired by React. It uses a Virtual DOM (VDOM) and a declarative macro (rsx!) that feels instantly familiar to web developers.".to_string();
                        s.item("Biggest Strength:",
                               "Multi-platform. The same code works for Web, Desktop (WebView), and Mobile."
                        );
                        s.item("Developer Experience:",
                               "Excellent CLI and high-speed hot-reloading."
                        );
                        s.footer = "Best For: Apps that need to live on Web, Desktop, and Mobile simultaneously.".to_string();
                        s.footer_text_color = "text-blue-800".to_string();
            });

        a.section("bg-orange-50",
                  "border-orange-100",
                  "2. Leptos: The Speed Demon",
                  "text-orange-900",
                  |s| {
                      s.p = "Leptos is the current performance king. Unlike Dioxus, it is 'Signals-based' (similar to SolidJS) and doesn't use a Virtual DOM.".to_string();
                      s.item("Biggest Strength:",
                          "Fine-grained reactivity and incredibly fast Server-Side Rendering (SSR)."
                      );
                      s.item("Learning Curve:",
                      "Slightly higher due to manual Signal management."
                      );
                      s.footer = "Best For: High-performance web-only apps where SEO is the top priority.".to_string();
                      s.footer_text_color = "text-orange-800".to_string();
                  });

        a.section("bg-green-50",
                  "border-green-100",
                  "3. Yew: The Original Veteran",
                  "text-green-900",
                  |s| {
                      s.p = "Yew was the first major Rust web framework. It is stable, well-documented, and has the largest community ecosystem.".to_string();
                      s.item("Biggest Strength:",
                             "Stability and a massive library of existing community components."
                      );
                      s.item("The Trade-off:",
                             "Generally slower than Leptos and lacks Dioxus's native mobile features."
                      );
                      s.footer = "Best For: Enterprise projects requiring a mature, battle-tested ecosystem.".to_string();
                      s.footer_text_color = "text-green-800".to_string();
                  });
        a.section("bg-purple-50",
                  "border-purple-100",
                  "4. Dioxus vs. Tauri 2.0: Synergy, Not Competition",
                  "text-purple-900",

                  |s| {
                      s.p = "While Dioxus is a UI framework, Tauri is an app 'shell' or bundler. They are often used together rather than being alternatives to one another.".to_string();

                      s.item("Dioxus (The UI):",
                             "Handles the actual 'view' logic, components, and state management using Rust."
                      );

                      s.item("Tauri (The Shell):",
                             "Provides the native window, system tray, and access to OS APIs like Filesystem and Notifications."
                      );

                      s.item("The Difference:",
                             "Dioxus has its own desktop renderer (Wry), but Tauri 2.0 offers deeper mobile plugin support (biometrics, geofencing)."
                      );

                      s.footer = "Best For: Use Dioxus standalone for speed; use Dioxus + Tauri for deep OS integration.".to_string();
                      s.footer_text_color = "text-purple-800".to_string();
                  });
    });
    my_article
}

