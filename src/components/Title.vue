<script setup>
const props = defineProps({
	title: String,
	description: String,
});
</script>

<template>
	<h1 class="font-bold text-4xl">
		{{ props.title }}
	</h1>

	<h2 class="mt-2 text-zinc-600">
		{{ props.description }}
	</h2>
</template>
