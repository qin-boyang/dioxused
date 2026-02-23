use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Tips() -> Element {
    let my_article = build_article_content();
    rsx! {
        div { class: "p-8 max-w-4xl mx-auto font-sans text-gray-900",

            // Navigation Link
            Link {
                to: Route::Home {},
                class: "text-blue-600 hover:text-blue-800 font-medium mb-8 block transition-colors",
                "← Back Home"
            }

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

                // Footer Note
                div { class: "pt-10 text-center text-gray-400 text-sm border-t",
                    "Built with Dioxus 0.7.3 — The future of Rust UI."
                }
            }
        }
    }
}
#[derive(Default)]
struct ArticleContent {
    h1: String,
    p: String,
    intro_p_1: String,
    intro_p_2: String,
    intro_p_3: String,
    sections: Vec<SectionContent>
}
impl ArticleContent {
    pub fn article(lambda: impl FnOnce(&mut Self)) -> Self {
        let mut article = Self {
            ..Default::default()
        };
        lambda(&mut article);
        article
    }

    pub fn section(&mut self, lambda: impl FnOnce(&mut SectionContent)) {
        let mut section = SectionContent {
            ..Default::default()
        };
        lambda(&mut section);
        self.sections.push(section);
    }
}
#[derive(Default)]
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
#[derive(Default)]
struct ListItemContent {
    b: String,
    p: String,
}

fn build_article_content() -> ArticleContent {
    let my_article = ArticleContent::article( |a| {
        a.h1 = "Why Choose Dioxus?".to_string();
        a.p = "Tips for Java Developer".to_string();
        a.intro_p_1 = "For Java Developer to learn Rust, there's some concept which used to be ignored or people tends to ignore because of ".to_string();
        a.intro_p_2 = "JVM Garbage Collector".to_string();
        a.intro_p_3 = ".".to_string();

        a.section(
            |s| {
                s.bg_color = "bg-blue-50".to_string();
                s.border_color = "border-blue-100".to_string();
                s.h2 = "1. Dangling Reference".to_string();
                s.h2_color = "text-blue-900".to_string();
                s.p = "fn dangle() -> &String { &some_string }".to_string();
                s.item("Problem:",
                     "It's basically returning a dangling reference. That reference points no String"
                );
                s.item("Lifetime:",
                     "Once the lifetime of 'some_string' is over, the reference becomes invalid"
                );
                s.footer = "IMPORTANCE: Rust does not allow dangling reference at compile time".to_string();
                s.footer_text_color = "text-blue-800".to_string();
        });
        a.section(
            |s| {
                s.bg_color = "bg-green-50".to_string();
                s.border_color = "border-green-100".to_string();
                s.h2 = "2. Ownership and Borrowing".to_string();
                s.h2_color = "text-green-900".to_string();
                s.p = "s , &s , &mut s".to_string();
                s.item("Ownership:",
                       "'s' could be a scalar or compound type, and also it can be a reference type"
                );
                s.item("Borrowing:",
                       "'&s' is borrowing of s, and '&mut s' is mutable borrowing of s, value of s can be changed"
                );
                s.footer = "IMPORTANCE: scarlar/compound type implemented 'Copy trait'. They can be borrowed without '&', because internally they are copied in stack memory".to_string();
                s.footer_text_color = "text-green-800".to_string();
            });
        a.section(
            |s| {
                s.bg_color = "bg-orange-50".to_string();
                s.border_color = "border-orange-100".to_string();
                s.h2 = "3. let if".to_string();
                s.h2_color = "text-orange-900".to_string();
                s.p = "let number = if condition { 5 } else { 6 };".to_string();
                s.item("Java:",
                       "int number = condition ? 5 : 6 "
                );
                s.item("Rust Readability:",
                       "👍"
                );
                s.footer = "IMPORTANCE: ✅".to_string();
                s.footer_text_color = "text-orange-800".to_string();
            });
        a.section(
            |s| {
                s.bg_color = "bg-purple-50".to_string();
                s.border_color = "border-purple-100".to_string();
                s.h2 = "4. Return multiple values (tuple) in a function".to_string();
                s.h2_color = "text-purple-900".to_string();
                s.p = "fn get_stats(numbers: &[i32]) -> (i32, i32, f64) {".to_string();
                s.item("Java/Kotlin:",
                       "Java or Kotlin function does not support returning multiple values"
                );
                s.item("Rust:",
                       "Rust function supports returning multiple values, but actually it is a compound type 'Tuple'"
                );
                s.footer = "IMPORTANCE: ✅".to_string();
                s.footer_text_color = "text-purple-800".to_string();
            });
        a.section(
            |s| {
                s.bg_color = "bg-yellow-50".to_string();
                s.border_color = "border-yellow-100".to_string();
                s.h2 = "5. Scalar Type Borrowing".to_string();
                s.h2_color = "text-yellow-900".to_string();
                s.p = "let x = 5;let y = x;println!('x = {} and y = {}', x, y);".to_string();
                s.item("Memory Storage:",
                       "Scalar type or you might say Java's primitive stores in Stack memory, not Heap"
                );
                s.item("Copy trait:",
                       "Types in Stack can be borrowed directly without '&'. They are actually copied in Stack. They implemented Copy trait"
                );
                s.item("What about Strings:",
                       "If x,y are Strings, the above will not compile. Because Strings are in Heap. Heap memory is different from a reference in Stack"
                );
                s.footer = "IMPORTANCE: Scalar types in Stack memory but Objects in Heap memory".to_string();
                s.footer_text_color = "text-yellow-800".to_string();
            });
    });
    my_article
}

#[cfg(test)]
fn test() {
    let x = 5;let y = x;println!("x = {} and y = {}", x, y);
    // let s1 = "hello".to_string();let s2 = s1;println!("s1 = {} and s2 = {}", s1, s2);
 }
