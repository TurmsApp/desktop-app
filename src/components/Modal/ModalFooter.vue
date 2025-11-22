<script setup lang="ts">
import Secondary from "../Button/Secondary.vue";

defineEmits(["finish"]);
defineProps(["disabledButton"]);
</script>

<template>
	<div class="px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
		<Secondary
			:disabled="disabledButton"
			@click="$emit('finish')"
			class="px-3 dark:text-white"
			tabindex="0"
		>
			Done
		</Secondary>
	</div>
</template>
