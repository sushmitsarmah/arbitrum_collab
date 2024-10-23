use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::route::Route;

// launches the app in the main
pub fn launch_app() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

// main app function
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}