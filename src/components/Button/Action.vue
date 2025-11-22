<script setup lang="ts">
import ButtonPrimary from "./Primary.vue";
import { CheckIcon } from '@heroicons/vue/24/outline';
import { ref } from 'vue';

const clicked = ref(false);

const validateClick = () => {
    clicked.value = true;
    setTimeout(() => {
        clicked.value = false;
    }, 1000);
};
</script>

<template>
    <ButtonPrimary
				@click="validateClick"
				class="absolute right-1 top-1 h-9 px-5 -mt-1 z-10"
			>
                <CheckIcon v-if="clicked" class="size-5" />
				<slot v-else />
			</ButtonPrimary>
</template>