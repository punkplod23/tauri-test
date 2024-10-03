const { invoke } = window.__TAURI__.core;
//import { open } from '@tauri-apps/plugin-dialog';
// when using `"withGlobalTauri": true`, you may use
const { open } = window.__TAURI__.dialog;
// Open a dialog
let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function run_csv() {
  const file = await open({
    multiple: false,
    directory: false,
  });
  if(file != null){
    greetMsgEl.innerHTML = await invoke("run_csv", { filepath: file});
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet").addEventListener("click", (e) => {
    e.preventDefault();
    greet();
  });
  document.querySelector("#upload-file").addEventListener("click", (e) => {
    e.preventDefault();
    run_csv();
  });
});
