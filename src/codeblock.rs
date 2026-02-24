use dioxus::prelude::*;
use crate::footer::Footer;
use crate::header::Header;
use crate::Route;

#[component]
pub fn CodeBlock() -> Element {
    let struct_code_content = r#"
        #[derive(Default)]
        struct Country {
            name: String,
            population: i32,
            states: Vec<State>,
        }
        #[derive(Default)]
        struct State {
            name: String,
            population: i32,
            cities: Vec<City>,
        }
        #[derive(Default)]
        struct City {
            name: String,
            population: i32,
        }
    "#;

    let impl_code_content = r#"
        impl Country {
            // We use FnOnce because the "build" block only runs once per Country
            pub fn country(lambda: impl FnOnce(&mut Country)) -> Self {
                let mut country = Country { ..Default::default() };
                lambda(&mut country);
                country
            }

            pub fn state(&mut self, lambda: impl FnOnce(&mut State)) {
                let mut state = State { ..Default::default() };
                lambda(&mut state);
                self.states.push(state);
            }
        }
        impl State {
            pub fn city(&mut self, lambda: impl FnOnce(&mut City)) {
                let mut city = City { ..Default::default() };
                lambda(&mut city);
                self.cities.push(city);
            }
        }
    "#;
    let test_code_content = r#"
        #[test]
        fn test_dsl() {
            let mut usa = Country::country(|country| {
                country.name = "USA".to_string();
                country.population = 328_200_000;
                country.state( |state| {
                    state.name = "California".to_string();
                    state.population = 39_500_000;
                    state.city( |city| {
                        city.name = "Los Angeles".to_string();
                        city.population = 19_000_000;
                    });
                    state.city(|city| {
                        city.name = "San Francisco".to_string();
                        city.population = 8_500_000;
                    });
                });
            });
        }
    "#;

    rsx! {
        div { class: "p-8 max-w-4xl mx-auto font-sans text-gray-900",
            Header {}

            div { class: "border-b pb-6",
                p { class: "text-lg text-gray-300 italic",
                    "writing a Kotlin style DSL to create rust objects... taking country/state/city as example."
                }
            }
            pre {
                class: "bg-gray-800 text-white p-4 rounded",
                code {
                    { struct_code_content }
                }
            }

            br {}
            br {}

            div { class: "border-b pb-6",
                p { class: "text-lg text-gray-300 italic",
                    "impl structs with Kotlin DSL style."
                }
            }
            pre {
                class: "bg-gray-800 text-white p-4 rounded",
                code {
                    { impl_code_content }
                }
            }

            br {}
            br {}

            div { class: "border-b pb-6",
                p { class: "text-lg text-gray-300 italic",
                    "how to use the DSL"
                }
            }
            pre {
                class: "bg-gray-800 text-white p-4 rounded",
                code {
                    {test_code_content}
                }
            }
            br {}
            Footer {}
        }
    }
}



#[derive(Default)]
struct Country {
    name: String,
    population: i32,
    states: Vec<State>,
}
#[derive(Default)]
struct State {
    name: String,
    population: i32,
    cities: Vec<City>,
}
#[derive(Default)]
struct City {
    name: String,
    population: i32,
}
impl Country {
    // We use FnOnce because the "build" block only runs once per Country
    pub fn country(lambda: impl FnOnce(&mut Country)) -> Self {
        let mut country = Country { ..Default::default() };
        lambda(&mut country);
        country
    }

    pub fn state(&mut self, lambda: impl FnOnce(&mut State)) {
        let mut state = State { ..Default::default() };
        lambda(&mut state);
        self.states.push(state);
    }
}
impl State {
    pub fn city(&mut self, lambda: impl FnOnce(&mut City)) {
        let mut city = City { ..Default::default() };
        lambda(&mut city);
        self.cities.push(city);
    }
}
#[test]
fn test_dsl() {
    let mut usa = Country::country(|country| {
        country.name = "USA".to_string();
        country.population = 328_200_000;
        country.state( |state| {
            state.name = "California".to_string();
            state.population = 39_500_000;
            state.city( |city| {
                city.name = "Los Angeles".to_string();
                city.population = 19_000_000;
            });
            state.city(|city| {
                city.name = "San Francisco".to_string();
                city.population = 8_500_000;
            });
        });
    });
}

