import './styles.css'
import { invoke } from "@tauri-apps/api/core";
import Alpine from 'alpinejs'

// suggested in the Alpine docs:
// make Alpine on window available for better DX
 
Alpine.data("fileProcessing", () => ({
    formatFileSize(bytesSize: number) {
        if(bytesSize == 0) return '0 Bytes';
        var k = 1000,
            dm = 2,
            sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'],
            i = Math.floor(Math.log(bytesSize) / Math.log(k));
        return parseFloat((bytesSize / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
    }
  })
)

Alpine.data("fetchApi", () => ({
  message: "",
  async fetchData() {
    await fetch("http://localhost:8008/").then((res) => res.json()).then((body) => this.message = body.message);
  }
}))

window.Alpine = Alpine

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

Alpine.start()
