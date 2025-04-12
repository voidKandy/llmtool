use components::{model::ModelLoader, notes::NotesViewComponent};
use dioxus::prelude::*;
mod components;
pub mod services;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const GLOBAL_CSS: Asset = asset!("/assets/styles/global.css");

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
    rsx! {
        document::Link{ rel:"preconnect", href:"https://fonts.googleapis.com"}
        document::Link{ rel:"preconnect", href:"https://fonts.gstatic.com", crossorigin: "true"}
        document::Link{ href:"https://fonts.googleapis.com/css2?family=Kode+Mono:wght@400..700&display=swap", rel:"stylesheet"}

        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: GLOBAL_CSS }
        NotesViewComponent {  }
        ModelLoader {  }

    }
}
