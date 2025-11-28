<script setup>
const props = defineProps({
	title: String,
	description: String,
});
</script>

<template>
	<h1 class="mix-blend-difference font-bold text-4xl">
		{{ props.title }}
	</h1>

	<h2 class="mt-2 text-zinc-600 dark:text-zinc-300">
		{{ props.description }}
	</h2>
</template>
