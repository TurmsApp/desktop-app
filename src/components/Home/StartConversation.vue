<script setup lang="ts">
import ButtonAction from "../Button/Action.vue";
import { onMounted, reactive, ref } from "vue";
import { ClipboardIcon, PaperAirplaneIcon } from "@heroicons/vue/24/outline";
import { invoke } from "@tauri-apps/api/core";
import AnswerModal from "./AnswerModal.vue";
import Tooltip from "../Button/Tooltip.vue";
import { QuestionMarkCircleIcon } from "@heroicons/vue/24/solid";

const modal = ref(false);
const data = reactive({
	identifier: "",
	peerId: "",
	answer: "",
});

const copyIdentifier = () => {
	navigator.clipboard.writeText(data.identifier);
};

onMounted(async () => {
	data.identifier = await invoke("generate_offer");
});

const requestConversation = async () => {
	if (data.peerId === "") return;

	data.answer = await invoke("connect_peer", { session: data.peerId });
	if (data.answer !== "") modal.value = true;
};
</script>

<template>
	<AnswerModal @close="modal = false" :visible="modal" :answer="data.answer" />

	<div class="h-full flex flex-col justify-center items-center">
		<div class="flex flex-col">
			<h2 class="font-semibold text-2xl">Peer-to-peer end-to-end encrypted.</h2>

			<label class="mt-4 flex text-sm text-zinc-600 dark:text-zinc-300 z-20">
				P2P identifier
			</label>
			<div @click="copyIdentifier" class="relative flex w-full">
				<ButtonAction
					@click="copyIdentifier"
					class="absolute right-1 top-1 h-9 px-5 -mt-1 z-10"
				>
					<ClipboardIcon class="size-5" />
				</ButtonAction>

				<input
					type="text"
					readonly
					class="text-sm p-2 w-full rounded border-zinc-200 dark:border-zinc-800 dark:text-white outline-none border"
					:value="data.identifier"
				/>
				<!--
				<input
					type="text"
					readonly
					class="peer h-full w-full rounded border border-white/20 py-2 px-4 pr-20 text-sm focus:border-2 focus:border-zinc-500 focus:border-t-transparent transition-all"
					:value="data.identifier"
				/>-->
			</div>

			<hr class="w-full my-4 border-zinc-200 dark:border-zinc-800" />

			<label class="flex text-sm text-zinc-600 dark:text-zinc-300 z-20">
				Connect using peer identifier
				<Tooltip
					text="WebRTC is two-step. Enter either offer (P2P identifier) or answer."
				>
					<QuestionMarkCircleIcon
						class="ml-1 mt-0.5 text-black dark:text-white size-4"
					/>
				</Tooltip>
			</label>
			<div class="relative flex w-full">
				<ButtonAction
					@click="requestConversation"
					class="absolute right-1 top-1 h-9 px-5 -mt-1 z-10"
				>
					<PaperAirplaneIcon class="size-5" />
				</ButtonAction>

				<input
					v-model="data.peerId"
					autocomplete="off"
					type="text"
					class="text-sm p-2 w-full rounded border-zinc-200 dark:border-zinc-800 dark:text-white outline-none border"
					placeholder="Contact P2P identifier or answer"
				/>
			</div>
		</div>
	</div>
</template>
