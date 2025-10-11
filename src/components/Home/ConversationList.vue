<script setup>
import {
	PlusIcon,
	ChatBubbleOvalLeftEllipsisIcon,
} from "@heroicons/vue/24/outline";
import ButtonSecondary from "../Button/Secondary.vue";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

const users = ref([]);
onMounted(async () => {
	users.value = await invoke("get_conversations");
});
</script>
<template>
	<div class="flex flex-col h-full mt-8">
		<div class="flex flex-row items-center justify-between text-xs">
			<span class="font-bold">Active conversations</span>
		</div>
		<div
			v-if="users.length === 0"
			class="flex-1 flex flex-col justify-center items-center"
		>
			<p class="text-xs">Start your first chat securely!</p>
			<ChatBubbleOvalLeftEllipsisIcon class="mt-4 size-12" />
		</div>
		<div
			v-else
			class="flex-1 flex flex-col space-y-1 mt-4 -mx-2 overflow-y-auto"
		>
			<RouterLink
				v-for="user in users"
				class="flex flex-row items-center hover:bg-zinc-100 rounded p-2 hover:cursor-pointer"
				draggable="false"
			>
				<div
					class="flex items-center justify-center size-8 bg-violet-100/30 rounded-full select-none font-semibold"
				>
					{{ user.username[0] }}
				</div>
				<div class="ml-2 text-sm font-semibold">{{ user.username }}</div>
			</RouterLink>
		</div>
		<RouterLink to="/conversation/">
			<ButtonSecondary class="flex mt-4 w-full">
				<PlusIcon class="size-4 mt-0.5 mr-1" />
				Add a contact
			</ButtonSecondary>
		</RouterLink>
	</div>
</template>
