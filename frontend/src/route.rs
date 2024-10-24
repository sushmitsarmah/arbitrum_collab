use dioxus::prelude::*;

use crate::pages::{
    blog::Blog,
    home::Home,
    landing::Landing,
};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/landing")]
    Landing {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}