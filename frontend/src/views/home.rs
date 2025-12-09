use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        p { "Welcome to the Home Page!" }
    }
}

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

