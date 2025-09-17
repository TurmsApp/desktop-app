import { createMemoryHistory, createRouter } from "vue-router";

import Conversation from "./pages/Conversation.vue";
import Connect from "./pages/Connect.vue";
import Configure from "./pages/Configure.vue";

const routes = [
	{ path: "/", component: Connect },
	{ path: "/conversation", component: Conversation }, // equivalent to home.
	{ path: "/conversation/:id", component: Conversation },
	{ path: "/configure", component: Configure },
];

const router = createRouter({
	history: createMemoryHistory(),
	routes,
});

export default router;
