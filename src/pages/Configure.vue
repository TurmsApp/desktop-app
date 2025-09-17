<script setup>
import Title from "../components/Title.vue";
import PrimaryButton from "../components/Button/Primary.vue";
import SecondaryButton from "../components/Button/Secondary.vue";
import { ChevronLeftIcon } from "@heroicons/vue/24/outline";
import { QuestionMarkCircleIcon } from "@heroicons/vue/24/solid";
import Tooltip from "../components/Button/Tooltip.vue";
import { ref } from "vue";

const step = ref(1);
const discoveryTooltip = ref(false);
const discoveryUrl = ref("discover.gravitalia.com");
</script>

<template>
	<div
		class="absolute w-full bg-amber-100 border-t border-b border-amber-500 text-amber-700 px-4 py-3"
		role="alert"
	>
		<p class="font-bold">Warning</p>
		<p class="text-sm">
			Custom instances require ports to be opened if you do not configure a STUN
			server.
		</p>
	</div>

	<div class="h-screen flex flex-col justify-center items-center">
		<div>
			<RouterLink
				to="/"
				class="mb-6 flex underline mt-6 xl:ml-1 text-md xl:text-lg text-zinc-500 dark:text-zinc-400"
			>
				<ChevronLeftIcon class="mt-1 mr-0.5 size-4" />
				Return back to login page
			</RouterLink>

			<Title
				title="Configure your instance"
				:description="`Step ${step}/2: configure the discovery server`"
			/>

			<!-- 1st step. -->
			<div v-if="step === 1">
				<!-- Turms Discovery configuration. -->
				<div class="mt-10">
					<h2 class="flex font-semibold text-xl">
						Turms discovery server <span class="text-red-500">*</span>
						<Tooltip text="Instance of https://github.com/TurmsApp/discovery">
							<QuestionMarkCircleIcon class="size-6 hover:cursor-help" />
						</Tooltip>
					</h2>

					<!-- Turms Discovery URL. -->
					<input
						class="w-[23rem] h-7 text-sm text-zinc-700 dark:text-zinc-200 dark:placeholder:text-zinc-300 outline-none bg-transparent border border-zinc-400 dark:border-zinc-700 border-b-2 border-x-0 border-t-0"
						type="text"
						placeholder="URL de Turms discovery"
						v-model="discoveryUrl"
					/>
				</div>

				<!-- Who are you? -->
				<h2 class="mt-10 font-semibold text-xl">
					Network identifier <span class="text-red-500">*</span>
				</h2>
				<div class="flex">
					<input
						class="w-36 h-7 text-sm text-zinc-700 dark:text-zinc-200 dark:placeholder:text-zinc-300 outline-none bg-transparent border border-zinc-400 dark:border-zinc-700 border-b-2 border-x-0 border-t-0"
						type="text"
						placeholder="Identifiant"
						maxlength="15"
					/>
					<span class="font-mono text-sm"> @{{ discoveryUrl }} </span>
				</div>
			</div>

			<!-- 2nd step. -->
			<div v-if="step === 2">
				<!-- STUN server configuration. -->
				<div class="mt-10">
					<h2 class="font-semibold text-xl">Relay server configuration</h2>
					<!-- STUN server URL. -->
					<h3 class="mt-4 font-semibold text-md">
						STUN server URL <span class="text-red-500">*</span>
					</h3>
					<input
						class="w-[23rem] h-7 text-sm text-zinc-700 dark:text-zinc-200 dark:placeholder:text-zinc-300 outline-none bg-transparent border border-zinc-400 dark:border-zinc-700 border-b-2 border-x-0 border-t-0"
						type="text"
						placeholder="URL"
						maxlength="15"
					/>

					<h3 class="mt-4 font-semibold text-md">STUN server authentication</h3>

					<input
						class="w-[23rem] h-7 text-sm text-zinc-700 dark:text-zinc-200 dark:placeholder:text-zinc-300 outline-none bg-transparent border border-zinc-400 dark:border-zinc-700 border-b-2 border-x-0 border-t-0"
						type="text"
						placeholder="Nom d'utilisateur"
					/>
					<div class="mt-2"></div>
					<input
						class="w-[23rem] h-7 text-sm text-zinc-700 dark:text-zinc-200 dark:placeholder:text-zinc-300 outline-none bg-transparent border border-zinc-400 dark:border-zinc-700 border-b-2 border-x-0 border-t-0"
						type="text"
						placeholder="Mot de passe"
					/>
				</div>
			</div>

			<!-- Step buttons. -->
			<div class="mt-16">
				<PrimaryButton v-if="step < 2" @click="step++" class="w-[23rem]">
					Next
				</PrimaryButton>
				<div v-else class="w-[23rem] flex justify-between space-x-4">
					<SecondaryButton v-if="step > 1" @click="step--" class="w-1/2">
						Previous
					</SecondaryButton>
					<PrimaryButton class="w-1/2"> Use this instance </PrimaryButton>
				</div>
			</div>
		</div>
	</div>
</template>
