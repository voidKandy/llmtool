use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config};
use tokenizers::{PaddingParams, Tokenizer};

pub struct Model {
    bert: BertModel,
    tokenizer: Tokenizer,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Params {
    pub sentences: Vec<String>,
    pub normalize_embeddings: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Embeddings {
    data: Vec<Vec<f32>>,
}

impl Model {
    pub fn load(
        weights: Vec<u8>,
        tokenizer: Vec<u8>,
        config: Vec<u8>,
    ) -> Result<Model, crate::Error> {
        tracing::warn!(
            "wasm constructor loading model\nweights len: {}\ntokenizer len: {}\nconfig len: {}",
            weights.len(),
            tokenizer.len(),
            config.len()
        );
        let device = &Device::Cpu;
        let vb = VarBuilder::from_buffered_safetensors(weights, DType::F32, device)?;
        let config: Config = serde_json::from_slice(&config).unwrap();
        let tokenizer = Tokenizer::from_bytes(&tokenizer)?;
        let bert = BertModel::load(vb, &config)?;

        Ok(Self { bert, tokenizer })
    }

    pub fn get_embeddings(&mut self, params: Params) -> Result<Vec<Vec<f32>>, crate::Error> {
        tracing::warn!("getting embeddings with params: {params:#?}");
        let sentences = params.sentences;
        let normalize_embeddings = params.normalize_embeddings;

        let device = &Device::Cpu;
        if let Some(pp) = self.tokenizer.get_padding_mut() {
            pp.strategy = tokenizers::PaddingStrategy::BatchLongest
        } else {
            let pp = PaddingParams {
                strategy: tokenizers::PaddingStrategy::BatchLongest,
                ..Default::default()
            };
            self.tokenizer.with_padding(Some(pp));
        }
        let tokens = self.tokenizer.encode_batch(sentences.to_vec(), true)?;

        let token_ids: Vec<Tensor> = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_ids().to_vec();
                Tensor::new(tokens.as_slice(), device)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let attention_mask: Vec<Tensor> = tokens
            .iter()
            .map(|tokens| {
                let tokens = tokens.get_attention_mask().to_vec();
                Tensor::new(tokens.as_slice(), device)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let token_ids = Tensor::stack(&token_ids, 0)?;
        let attention_mask = Tensor::stack(&attention_mask, 0)?;
        let token_type_ids = token_ids.zeros_like()?;
        tracing::warn!("running inference on batch {:?}", token_ids.shape());
        let embeddings = self
            .bert
            .forward(&token_ids, &token_type_ids, Some(&attention_mask))?;
        tracing::warn!("generated embeddings {:?}", embeddings.shape());
        // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
        let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
        let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
        let embeddings = if normalize_embeddings {
            embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?
        } else {
            embeddings
        };
        Ok(embeddings.to_vec2()?)
    }
}

pub async fn load_model() -> Result<(), crate::Error> {
    let url =
        "https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2/resolve/refs%2Fpr%2F21/";
    let info = get_model_info(url);
    let weights = fetch(&info.model_url).await;
    let tokenizer = fetch(&info.tokenizer_url).await;
    let config = fetch(&info.config_url).await;
    let mut model = Model::load(weights, tokenizer, config)?;
    tracing::warn!("model loaded!");
    let embeddings = model.get_embeddings(Params {
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

pub async fn fetch(url: &str) -> Vec<u8> {
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
