use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::route::Route;
use crate::server::server::{get_server_data, post_server_data};

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            class: "flex flex-col gap-4 items-center",
            h1 { class: "text-3xl font-bold underline", "High-Five counter: {count}" }
            button { class: "btn btn-primary", onclick: move |_| count += 1, "Up high!" }
            button { class: "btn btn-primary", onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}