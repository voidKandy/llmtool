use crate::services;
use dioxus::prelude::*;

const MODEL_STYLES: Asset = asset!("/assets/styles/model.css");

#[component]
pub fn ModelLoader() -> Element {
    // let mut response = use_signal(|| String::from("..."));
    let mut modelError = use_signal(|| "");
    let mut modelLoading = use_signal(|| false);
    let mut modelSuccess = use_signal(|| false);

    let try_load_model = move |_| {
        spawn(async move {
            modelLoading.set(true);
            let resp = services::ml::load_model().await;

            match resp {
                Ok(_) => {
                    tracing::info!("model loaded");
                    // response.set("model loaded".into());
                }
                Err(err) => {
                    tracing::info!("model loading failed with error: {err:?}");
                    modelError.set("failed to load model");
                    // response.set(format!("Request failed with error: {err:?}"));
                }
            }
            modelLoading.set(false);
        });
    };

    rsx!(
        document::Link { rel: "stylesheet", href: MODEL_STYLES },
        div {
            class:"model-loading-container",
            onmounted:try_load_model,
            if modelLoading() == true && modelError() == "" {
                p{"loading model"}
            }
            else if modelLoading() == false && modelError() != "" {
                p{"{modelError}"}
            }
        }
    )
}
