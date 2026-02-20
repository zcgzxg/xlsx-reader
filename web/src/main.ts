import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";

const t0 = Date.now();

createApp(App).mount("#app");

console.log(`应用挂载完成，耗时: ${Date.now() - t0}ms`);
