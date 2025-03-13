use components::notes::{generate_mock_notes, NotesComponent};
use dioxus::prelude::*;
mod components;
mod services;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

pub type Error = Box<dyn std::error::Error + 'static + Send + Sync>;
fn main() {
    #[cfg(feature = "desktop")]
    {
        dioxus_sdk::set_dir!();
    }
    // https://dioxuslabs.com/learn/0.6/guide/state#global-state-with-context
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut response = use_signal(|| String::from("..."));

    let try_load_model = move |_| {
        spawn(async move {
            let resp = services::ml::load_model().await;

            match resp {
                Ok(_data) => {
                    tracing::info!("model loaded responded!");
                    response.set("model loaded".into());
                }
                Err(err) => {
                    tracing::info!("Request failed with error: {err:?}");
                    response.set(format!("Request failed with error: {err:?}"));
                }
            }
        });
    };

    let cached_notes =
        dioxus_sdk::storage::use_persistent::<Vec<components::notes::Note>>("notes", || {
            tracing::warn!("generating");
            generate_mock_notes()
        });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script {src: asset!("assets/javascript/surreal.js"), type: Some("module".to_string())}
        document::Script {src: asset!("assets/javascript/tst.js"), type: Some("module".to_string())}
        button { onclick: try_load_model, "Response: {response}" }
        NotesComponent{ notes: cached_notes }
    }
}
