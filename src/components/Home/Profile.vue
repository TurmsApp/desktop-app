<script setup>
import { UserIcon } from "@heroicons/vue/24/outline";
</script>

<template>
	<div
		class="flex flex-col items-center bg-violet-100 border border-zinc-200 mt-4 w-full py-6 px-4 rounded-lg"
	>
		<div class="h-20 w-20 rounded-full border overflow-hidden">
			<div class="h-full w-full bg-zinc-300 flex items-center justify-center">
				<UserIcon class="size-12" />
			</div>
		</div>
		<div class="text-sm font-semibold mt-2">Guest</div>
		<div class="text-xs text-zinc-500">No metadata, no history.</div>
		<div class="flex flex-row items-center mt-3 cursor-not-allowed">
			<div
				class="flex flex-col justify-center h-4 w-8 bg-violet-500 rounded-full"
			>
				<div class="h-3 w-3 bg-white rounded-full self-end mr-1"></div>
			</div>
			<div class="leading-none ml-1 text-xs select-none">
				Conversation request
			</div>
		</div>
	</div>
</template>
