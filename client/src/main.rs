use components::notes::{generate_mock_notes, NotesComponent, NotesProps, NotesPropsBuilder};
use dioxus::prelude::*;
// use model::get_model_and_load;
mod components;
mod model;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

pub type Error = Box<dyn std::error::Error + 'static + Send + Sync>;
async fn load_model() -> Result<(), Error> {
    let url =
        "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/refs%2Fpr%2F21/";
    let info = get_model_info(url);
    let weights = fetch(&info.model_url).await;
    let tokenizer = fetch(&info.tokenizer_url).await;
    let config = fetch(&info.config_url).await;
    let mut model = model::Model::load(weights, tokenizer, config)?;
    tracing::warn!("model loaded!");
    let embeddings = model.get_embeddings(model::Params {
        sentences: vec![String::from("sentence about something")],
        normalize_embeddings: false,
    });
    tracing::warn!("embeddings: {embeddings:#?}");

    Ok(())
}

struct ModelInfo {
    model_url: String,
    config_url: String,
    tokenizer_url: String,
    search_prefix: String,
    document_prefix: String,
}

async fn fetch(url: &str) -> Vec<u8> {
    // const cacheName = "bert-candle-cache";
    // const cache = await caches.open(cacheName);
    // const cachedResponse = await cache.match(url);
    // if (cachedResponse) {
    //   console.log("data is cached");
    //   const data = await cachedResponse.arrayBuffer();
    //   return new Uint8Array(data);
    // }
    // console.log("need to fetch resource");
    // const res = await fetch(url, { cache: "force-cache" });
    let res = reqwest::Client::new().get(url).send().await.unwrap();
    res.bytes().await.unwrap().to_vec()
}

fn get_model_info(url: &str) -> ModelInfo {
    return ModelInfo {
        model_url: format!("{url}model.safetensors"),
        config_url: format!("{url}config.json"),
        tokenizer_url: format!("{url}tokenizer.json"),
        search_prefix: String::new(),
        document_prefix: String::new(),
    };
}

#[component]
fn App() -> Element {
    let mut response = use_signal(|| String::from("..."));

    let log_in = move |_| {
        spawn(async move {
            let resp = load_model().await;

            match resp {
                Ok(_data) => {
                    tracing::info!("dioxuslabs.com responded!");
                    response.set("dioxuslabs.com responded!".into());
                }
                Err(err) => {
                    tracing::info!("Request failed with error: {err:?}")
                }
            }
        });
    };
    let mock_notes = generate_mock_notes();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        button { onclick: log_in, "Response: {response}" }
        // Hero {}
        NotesComponent{notes: mock_notes }

        // button { onclick: move |_| async move{  get_model_and_load }, id:"click!", "click me"}
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
