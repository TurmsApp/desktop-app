<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import ModalFooter from "./ModalFooter.vue";
import ModalHeader from "./ModalHeader.vue";

const { visible } = defineProps({
	title: String,
	description: String,
	visible: Boolean,
	disabledButton: Boolean,
});

const emit = defineEmits(["close", "finish"]);

const handleEscapeKey = (e: KeyboardEvent) => {
	if (e.key === "Escape" && visible) {
		emit("close");
		e.preventDefault();
	}
};

onMounted(() => {
	window.addEventListener("keydown", handleEscapeKey);
});

onUnmounted(() => {
	window.removeEventListener("keydown", handleEscapeKey);
});
</script>

<template>
	<Teleport to="body">
		<!-- Put modal over others elements. -->
		<div v-show="visible" class="relative z-40" role="dialog" aria-modal="true">
			<!-- Add a blured font to highlight modal. -->
			<div class="fixed inset-0 bg-zinc-900/75" aria-hidden="true"></div>

			<div
				class="fixed inset-0 flex items-center justify-center overflow-y-auto"
			>
				<div class="flex items-end justify-center p-2 text-center w-screen">
					<div
						class="relative transform overflow-hidden rounded bg-white dark:bg-zinc-950 border border-zinc-200 dark:border-zinc-800 text-left w-screen max-w-lg"
					>
						<div class="px-3 pt-5 pb-4 sm:p-6 sm:pb-4">
							<div class="sm:flex sm:items-start">
								<div
									class="w-full mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left"
								>
									<!-- Modal header. -->
									<ModalHeader
										:title="title"
										:description="description"
										@close="$emit('close')"
									/>
									<div class="mt-6">
										<slot />
									</div>
								</div>
							</div>
						</div>
						<ModalFooter
							:disabledButton="disabledButton"
							@finish="$emit('finish')"
						/>
					</div>
				</div>
			</div>
		</div>
	</Teleport>
</template>
