<script setup lang="ts">
import { ClipboardIcon } from "@heroicons/vue/24/outline";
import ButtonAction from "../Button/Action.vue";
import Modal from "../Modal/Modal.vue";

const emit = defineEmits(["close"]);
const { answer } = defineProps({
	visible: Boolean,
	answer: {
		type: String,
		required: true,
	},
});

const copyAnswer = () => {
	navigator.clipboard.writeText(answer);
};

const close = () => {
	emit("close");
};
</script>

<template>
	<Modal
		title="Answer to peer"
		description="Peer received your offer! You can now send him this answer."
		:visible="visible"
		@close="close()"
		@finish="close()"
	>
		<div @click="copyAnswer" class="mt-4 relative flex w-full">
			<ButtonAction
				@click="copyAnswer"
				class="absolute right-1 top-1 h-9 px-5 -mt-1 z-10"
			>
				<ClipboardIcon class="size-5" />
			</ButtonAction>

			<input
				type="text"
				readonly
				class="h-9 w-full rounded border py-2 px-4 pr-20 text-sm border-white/50 dark:border-zinc-800 border-t-transparent dark:border-t-zinc-800 transition-all"
				:value="answer"
			/>
			<label
				class="pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-xs transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-zinc-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-zinc-200 after:transition-all peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:!border-zinc-500 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:!border-zinc-500"
			>
				WebRTC answer
			</label>
		</div>
	</Modal>
</template>
