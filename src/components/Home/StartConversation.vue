<script setup>
import ButtonPrimary from "../Button/Primary.vue";
import { onMounted, ref } from "vue";
import { ClipboardIcon } from "@heroicons/vue/24/outline";
import { invoke } from "@tauri-apps/api/core";

const copyPeerIdentifier = () => {
	navigator.clipboard.writeText(peerIdentifier.value);
};

const peerIdentifier = ref("");
onMounted(async () => {
	peerIdentifier.value = await invoke("generate_offer");
});
</script>

<template>
	<div class="h-full flex flex-col justify-center items-center">
		<div class="flex flex-col">
			<h2 class="font-semibold text-2xl">Peer-to-peer end-to-end encrypted.</h2>

			<div @click="copyPeerIdentifier" class="mt-4 relative flex w-full">
				<ButtonPrimary
					@click="copyPeerIdentifier"
					class="absolute right-1 top-1 h-9 px-5 -mt-1 z-10"
				>
					<ClipboardIcon class="size-5" />
				</ButtonPrimary>

				<input
					type="text"
					readonly
					class="h-9 w-full rounded border py-2 px-4 pr-20 text-sm border-white/50 border-t-transparent transition-all"
					:value="peerIdentifier"
				/>
				<!--
				<input
					type="text"
					readonly
					class="peer h-full w-full rounded border border-white/20 py-2 px-4 pr-20 text-sm focus:border-2 focus:border-zinc-500 focus:border-t-transparent transition-all"
					:value="peerIdentifier"
				/>-->
				<label
					class="pointer-events-none absolute left-0 -top-1.5 flex h-full w-full select-none text-xs transition-all before:pointer-events-none before:mt-[6.5px] before:mr-1 before:box-border before:block before:h-1.5 before:w-2.5 before:rounded-tl-md before:border-t before:border-l before:border-zinc-200 before:transition-all after:pointer-events-none after:mt-[6.5px] after:ml-1 after:box-border after:block after:h-1.5 after:w-2.5 after:flex-grow after:rounded-tr-md after:border-t after:border-r after:border-zinc-200 after:transition-all peer-focus:before:border-t-2 peer-focus:before:border-l-2 peer-focus:before:!border-zinc-500 peer-focus:after:border-t-2 peer-focus:after:border-r-2 peer-focus:after:!border-zinc-500"
				>
					P2P identifier
				</label>
			</div>
		</div>
	</div>
</template>
