import { createApp } from "vue";
import App from "./App.vue";
import { devtools } from "@vue/devtools";
import router from "./router";

if (process.env.NODE_ENV === "development") {
	devtools.connect("http://localhost", 3000);
}

createApp(App).use(router).mount("#app");
