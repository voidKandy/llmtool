use components::notes::{generate_mock_notes, NotesComponent, NotesProps, NotesPropsBuilder};
use dioxus::prelude::*;
// use model::get_model_and_load;
mod components;
// mod model;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mock_notes = generate_mock_notes();
    rsx! {
        // document::Script{src: asset!("/assets/bertWorker.js"), type: Some("module".to_string()) }
        // document::Script{src: asset!("/assets/utils.js"), type: Some("module".to_string()) }
        // document::Script{src: asset!("/assets/test.js"), type: Some("module".to_string()) }
        // Hero {}
        NotesComponent{notes: mock_notes }
         // button {
         //    onclick: move |_| async move {
         //        get_model_and_load().await;
         //        tracing::warn!("You clicked the button one second ago!");
         //    },
         //    "Click me"
        // }

        // button { onclick: move |_| async move{  get_model_and_load }, id:"click!", "click me"}
        // document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
         // button {
            // The `onclick` event accepts a closure with the signature `fn(Event)`
            // onclick: |event_data| tracing::warn!("clicked! I got the event data: {event_data:?}"),
            // "Click me",
        // }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
