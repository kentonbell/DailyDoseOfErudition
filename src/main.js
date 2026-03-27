const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;
let funFactBtnEl;
let funFactMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function fun_fact() {
  const result = await invoke("fetch_feed", { url: "https://www.brainyquote.com/link/quotebr.rss" });
  const randomIndex = Math.floor(Math.random() * result.length);
  const firstItemTitle = result[randomIndex]?.title || "No title found";
  let firstItemdDescription = result[randomIndex]?.description || "No description found";


  if (firstItemdDescription !== "No description found") {
    console.log("Original description:", firstItemdDescription);
    const parser = new DOMParser();
    const descriptionDoc = parser.parseFromString(firstItemdDescription, "text/html");
    var descriptionText = descriptionDoc.body.textContent || "";
    firstItemdDescription = descriptionText.trim();
  }

  funFactMsgEl.textContent = `Here you are: ${firstItemTitle} - ${firstItemdDescription}`;
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input"); //this is the input element where the user will enter their name
  greetMsgEl = document.querySelector("#greet-msg"); //this is the element where the greeting message will be displayed
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  funFactMsgEl = document.querySelector("#fun-fact-msg"); //this is the element where the fun fact message will be displayed
  funFactBtnEl = document.querySelector("#fun-fact-btn"); //this is the button element that will trigger the fun fact function when clicked
  funFactBtnEl.addEventListener("click", fun_fact);
});
