import { createMemoryHistory, createRouter } from "vue-router";

import Conversation from "./pages/Conversation.vue";
import Connect from "./pages/Connect.vue";
import Configure from "./pages/Configure.vue";

import { authentificator } from "./middleware";

const routes = [
	{ path: "/", component: Connect, beforeEnter: [authentificator] },
	{
		path: "/conversation",
		component: Conversation,
		afterEnter: [authentificator],
	}, // equivalent to home.
	{
		path: "/conversation/:id",
		component: Conversation,
		afterEnter: [authentificator],
	},
	{ path: "/configure", component: Configure },
];

const router = createRouter({
	history: createMemoryHistory(),
	routes,
});

export default router;
