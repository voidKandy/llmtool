//load Candle Bert Module wasm module
import init, { Model } from "./build/model.js";

/**
* @param {URL} url
* @returns {Promise<Uint8Array>} 
**/
async function fetchArrayBuffer(url) {
  console.log(`Fetching array buffer from ${url}`);
  const cacheName = "bert-candle-cache";
  const cache = await caches.open(cacheName);
  const cachedResponse = await cache.match(url);
  if (cachedResponse) {
    console.log("data is cached");
    const data = await cachedResponse.arrayBuffer();
    return new Uint8Array(data);
  }
  console.log("need to fetch resource");
  const res = await fetch(url, { cache: "force-cache" });
  cache.put(url, res.clone());
  return new Uint8Array(await res.arrayBuffer());
}

class Bert {
  /**
  * A cache for storing model instances by their ID.
  * as a `static`, this is shared by all instances of this Class
  * @type {Record<string, Model>}
  */
  static cache = {};

  /**
  * @param {string} weightsURL 
  * @param {string} tokenizerURL  
  * @param {string} configURL   
  * @param {string} modelID
  * @returns {Promise<Model>} A promise that resolves to the loaded model instance.
  **/
  static async getInstance(weightsURL, tokenizerURL, configURL, modelID) {
    return new Promise(async (resolve, reject) => {
      if (!this.cache[modelID]) {
        console.log(`No cached model with id ${modelID}`);
        await init();
        self.postMessage({ status: "loading", message: "Loading Model" });

        try {
          const [weightsArrayU8, tokenizerArrayU8, mel_filtersArrayU8] = await Promise.all([
            fetchArrayBuffer(weightsURL),
            fetchArrayBuffer(tokenizerURL),
            fetchArrayBuffer(configURL),
          ]);

          const model = new Model(
            weightsArrayU8,
            tokenizerArrayU8,
            mel_filtersArrayU8
          );
        
          // Model created successfully, cache it and resolve the Promise
          self.postMessage({ status: "loading", message: "Loaded Model" });
          Bert.cache[modelID] = model;
          resolve(model);  // Resolving with the model
        } catch (error) {
          // If there's an error, reject the Promise with the error message
          self.postMessage({ error: 'Failed to load model: ' + error.message });
          reject(error);  // Rejecting with the error
        }
      } else {
        console.log(`cached model with id ${modelID} exists!`);
        self.postMessage({ status: "ready", message: "Model Already Loaded" });
        resolve(Bert.cache[modelID]);  // Resolving with the cached model
      }
    });
  }


}

self.addEventListener("message", async (event) => {
  console.log(`message: ${JSON.stringify(event.data)}`);
  const {
    weightsURL,
    tokenizerURL,
    configURL,
    modelID,
    sentences,
    normalize = true,
  } = event.data;

  // Start processing the model
  self.postMessage({ status: "ready", message: "Starting Bert Model" });

  try {
    // Load the model
    const model = await Bert.getInstance(weightsURL, tokenizerURL, configURL, modelID);
    console.log('Model loaded successfully:', model);

    // Notify that embeddings calculation is starting
    self.postMessage({
      status: "embedding",
      message: "Calculating Embeddings",
    });

    // Get embeddings for the sentences
    const output = model.get_embeddings({
      sentences: sentences,
      normalize_embeddings: normalize,
    });

    // Notify that the process is complete
    self.postMessage({
      status: "complete",
      message: "complete",
      output: output.data,
    });
  } catch (error) {
    // If there's an error in loading the model or calculating embeddings
    console.error('Error:', error);
    self.postMessage({ error: error.message || "Unknown error" });
  }
});
