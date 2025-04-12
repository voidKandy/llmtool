use crate::services;
use dioxus::prelude::*;

const MODEL_STYLES: Asset = asset!("/assets/styles/model.css");

#[component]
pub fn ModelLoader() -> Element {
    // let mut response = use_signal(|| String::from("..."));
    let mut model_error = use_signal(|| "");
    let mut model_loading = use_signal(|| false);
    let mut model_success = use_signal(|| false);

    let try_load_model = move |_| {
        spawn(async move {
            model_loading.set(true);
            let resp = services::ml::load_model().await;
            // cache hash
            // HashMap< &'static str,BurtModel>

            match resp {
                Ok(_) => {
                    // tracing::info!("model loaded");
                    // response.set("model loaded".into());
                }
                Err(err) => {
                    // tracing::info!("model loading failed with error: {err:?}");
                    model_error.set("failed to load model");
                    // response.set(format!("Request failed with error: {err:?}"));
                }
            }
            model_success.set(true);
            model_loading.set(false);
        });
    };

    rsx!(
        document::Link { rel: "stylesheet", href: MODEL_STYLES },
        div {
            class:"model-loading-container",
            onmounted:try_load_model,
            if model_loading() == false && model_error() == "" && model_success() == true {
                p{"loading model success"}
            }
            else if model_loading() == true && model_error() == "" {
                p{"loading model"}
            }
            else if model_loading() == false && model_error() != "" {
                p{"{model_error}"}
            }
        }
    )
}
