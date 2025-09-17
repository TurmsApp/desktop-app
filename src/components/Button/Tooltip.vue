<script setup>
import { ref } from "vue";
const props = defineProps(["text"]);
const isTooltipShown = ref(false);
</script>

<template>
	<div @mouseover="isTooltipShown = true" @mouseleave="isTooltipShown = false">
		<slot />
	</div>
	<div
		v-if="isTooltipShown"
		role="tooltip"
		class="absolute px-2 py-1 mt-4 text-xs text-white bg-zinc-900 rounded dark:bg-zinc-700"
	>
		{{ text }}
	</div>
</template>
