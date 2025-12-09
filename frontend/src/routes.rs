use dioxus::prelude::*;
use crate::components::Navbar;
use crate::views::Home;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

}
