<script setup>
import { LockClosedIcon } from "@heroicons/vue/24/outline";
import Tooltip from "../Button/Tooltip.vue";
</script>

<template>
	<div class="flex flex-row items-center justify-center h-12 w-full">
		<div class="bg-violet-100 p-1.5 rounded">
			<Tooltip
				text="Conversations are end to end encrypted using rotating keys."
			>
				<LockClosedIcon class="size-5" />
			</Tooltip>
		</div>

		<div class="ml-2 font-bold text-2xl">Turms</div>
	</div>
</template>
