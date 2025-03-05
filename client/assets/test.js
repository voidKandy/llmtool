
import { hcl } from "https://cdn.skypack.dev/d3-color@3";
import { interpolateReds } from "https://cdn.skypack.dev/d3-scale-chromatic@3";
import { scaleLinear } from "https://cdn.skypack.dev/d3-scale@4";
import {
  getModelInfo,
  getEmbeddings,
  getWikiText,
  cosineSimilarity,
} from "./utils.js";

const bertWorker = new Worker("./bertWorker.js", {
  type: "module",
});

console.log('loaded');

// const inputContainerEL = document.querySelector("#input-container");
// const textAreaEl = document.querySelector("#input-area");
// const outputAreaEl = document.querySelector("#output-area");
// const formEl = document.querySelector("#form");
// const searchInputEl = document.querySelector("#search-input");
// const formWikiEl = document.querySelector("#form-wiki");
// const searchWikiEl = document.querySelector("#search-wiki");
// const outputStatusEl = document.querySelector("#output-status");
// const modelSelectEl = document.querySelector("#model");

// const sentencesRegex =
//   /(?<!\w\.\w.)(?<![A-Z][a-z]\.)(?<![A-Z]\.)(?<=\.|\?)\s/gm;

let sentenceEmbeddings = [];
// let currInputText = "";
// let isCalculating = false;

// function toggleTextArea(state) {
//   if (state) {
//     textAreaEl.hidden = false;
//     textAreaEl.focus();
//   } else {
//     textAreaEl.hidden = true;
//   }
// }
// inputContainerEL.addEventListener("focus", (e) => {
//   toggleTextArea(true);
// });
// textAreaEl.addEventListener("blur", (e) => {
//   toggleTextArea(false);
// });
// textAreaEl.addEventListener("focusout", (e) => {
//   toggleTextArea(false);
//   if (currInputText === textAreaEl.value || isCalculating) return;
//   populateOutputArea(textAreaEl.value);
//   calculateEmbeddings(textAreaEl.value);
// });

// modelSelectEl.addEventListener("change", (e) => {
//   if (currInputText === "" || isCalculating) return;
//   populateOutputArea(textAreaEl.value);
//   calculateEmbeddings(textAreaEl.value);
// });

// function populateOutputArea(text) {
//   currInputText = text;
//   const sentences = text.split(sentencesRegex);

//   outputAreaEl.innerHTML = "";
//   for (const [id, sentence] of sentences.entries()) {
//     const sentenceEl = document.createElement("span");
//     sentenceEl.id = `sentence-${id}`;
//     sentenceEl.innerText = sentence + " ";
//     outputAreaEl.appendChild(sentenceEl);
//   }
// }
// formEl.addEventListener("submit", async (e) => {
//   e.preventDefault();
//   if (isCalculating || currInputText === "") return;
//   toggleInputs(true);
  // const modelID = modelSelectEl.value;
  // const { modelURL, tokenizerURL, configURL, search_prefix } =
  //   getModelInfo(modelID);
  const { modelURL, tokenizerURL, configURL, search_prefix } =
    getModelInfo("intfloat_e5_base_v2");

//   const text = searchInputEl.value;
  const query = "this is my query";
//   outputStatusEl.classList.remove("invisible");
//   outputStatusEl.innerText = "Calculating embeddings for query...";
//   isCalculating = true;
  const out = await getEmbeddings(
    bertWorker,
    modelURL,
    tokenizerURL,
    configURL,
    "intfloat_e5_base_v2",
    [query]
  );
//   outputStatusEl.classList.add("invisible");
  const queryEmbeddings = out.output[0];
//   // calculate cosine similarity with all sentences given the query
  const distances = sentenceEmbeddings
    .map((embedding, id) => ({
      id,
      similarity: cosineSimilarity(queryEmbeddings, embedding),
    }))
    .sort((a, b) => b.similarity - a.similarity)
    // getting top 10 most similar sentences
    .slice(0, 10);
console.log(distances);
//   const colorScale = scaleLinear()
//     .domain([
//       distances[distances.length - 1].similarity,
//       distances[0].similarity,
//     ])
//     .range([0, 1])
//     .interpolate(() => interpolateReds);
//   outputAreaEl.querySelectorAll("span").forEach((el) => {
//     el.style.color = "unset";
//     el.style.backgroundColor = "unset";
//   });
//   distances.forEach((d) => {
//     const el = outputAreaEl.querySelector(`#sentence-${d.id}`);
//     const color = colorScale(d.similarity);
//     const fontColor = hcl(color).l < 70 ? "white" : "black";
//     el.style.color = fontColor;
//     el.style.backgroundColor = color;
//   });

//   outputAreaEl
//     .querySelector(`#sentence-${distances[0].id}`)
//     .scrollIntoView({
//       behavior: "smooth",
//       block: "center",
//       inline: "nearest",
//     });

//   isCalculating = false;
//   toggleInputs(false);
// });
// async function calculateEmbeddings(text) {
//   isCalculating = true;
//   toggleInputs(true);
//   const modelID = modelSelectEl.value;
//   const { modelURL, tokenizerURL, configURL, document_prefix } =
//     getModelInfo(modelID);

//   const sentences = text.split(sentencesRegex);
//   const allEmbeddings = [];
//   outputStatusEl.classList.remove("invisible");
//   for (const [id, sentence] of sentences.entries()) {
//     const query = document_prefix + sentence;
//     outputStatusEl.innerText = `Calculating embeddings: sentence ${
//       id + 1
//     } of ${sentences.length}`;
//     const embeddings = await getEmbeddings(
//       bertWorker,
//       modelURL,
//       tokenizerURL,
//       configURL,
//       modelID,
//       [query],
//       updateStatus
//     );
//     allEmbeddings.push(embeddings);
//   }
//   outputStatusEl.classList.add("invisible");
//   sentenceEmbeddings = allEmbeddings.map((e) => e.output[0]);
//   isCalculating = false;
//   toggleInputs(false);
// }

// function updateStatus(data) {
//   if ("status" in data) {
//     if (data.status === "loading") {
//       outputStatusEl.innerText = data.message;
//       outputStatusEl.classList.remove("invisible");
//     }
//   }
// }
// function toggleInputs(state) {
//   const interactive = document.querySelectorAll(".interactive");
//   interactive.forEach((el) => {
//     if (state) {
//       el.disabled = true;
//     } else {
//       el.disabled = false;
//     }
//   });
// }

// searchWikiEl.addEventListener("input", () => {
//   searchWikiEl.setCustomValidity("");
// });

// formWikiEl.addEventListener("submit", async (e) => {
//   e.preventDefault();
//   if ("example" in e.submitter.dataset) {
//     searchWikiEl.value = e.submitter.innerText;
//   }
//   const text = searchWikiEl.value;

//   if (isCalculating || text === "") return;
//   try {
//     const wikiText = await getWikiText(text);
//     searchWikiEl.setCustomValidity("");
//     textAreaEl.innerHTML = wikiText;
//     populateOutputArea(wikiText);
//     calculateEmbeddings(wikiText);
//     searchWikiEl.value = "";
//   } catch {
//     searchWikiEl.setCustomValidity("Invalid Wikipedia article name");
//     searchWikiEl.reportValidity();
//   }
//     });
