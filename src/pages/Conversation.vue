<script setup>
import { useRoute } from "vue-router";
import Title from "../components/Home/Title.vue";
import Profile from "../components/Home/Profile.vue";
import ConversationList from "../components/Home/ConversationList.vue";
import ConversationText from "../components/Home/ConversationText.vue";
import StartConversation from "../components/Home/StartConversation.vue";

const route = useRoute();
const conversationId = route.params.id;
</script>

<template>
	<div class="flex h-screen antialiased text-zinc-800">
		<div class="flex flex-row h-full w-full overflow-x-hidden">
			<div class="flex flex-col py-8 px-4 w-64 bg-white/20 shrink-0">
				<Title />
				<Profile />
				<ConversationList />
			</div>
			<div class="flex flex-col flex-auto h-full p-6">
				<div
					class="flex flex-col flex-auto shrink-0 rounded-2xl bg-white/20 p-4"
				>
					<ConversationText v-if="conversationId" :id="conversationId" />
					<StartConversation v-else />
				</div>
			</div>
		</div>
	</div>
</template>
