<script setup>
import Title from "../components/Title.vue";
import PrimaryButton from "../components/Button/Primary.vue";
import SecondaryButton from "../components/Button/Secondary.vue";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-shell";
import { ChevronRightIcon } from "@heroicons/vue/24/outline";
import { invoke } from "@tauri-apps/api/core";

const router = useRouter();

const openAuthPage = async () => {
	await open("https://turms.gravitalia.com/auth");
};

const asGuest = () => {
	invoke("init", {})
		.then((_) => console.info("success login, connected as guest"), router.push("/conversation/"))
		.catch((error) => console.error(error));
};
</script>

<template>
	<div class="h-screen flex flex-col justify-center items-center">
		<img src="../assets/Square107x107Logo.png" class="size-16" />
		<Title
			title="Welcome on Turms"
			description="Access your secure peer-to-peer messaging service."
		/>

		<PrimaryButton
			class="mt-16 w-[23rem] h-12 flex justify-center items-center text-zinc-100"
			@click="openAuthPage"
		>
			Continue with Gravitalia
		</PrimaryButton>

		<div class="flex mt-4 items-center text-center w-[23rem]">
			<hr class="border-zinc-300 border-1 w-full rounded-md" />
			<label class="block font-medium text-sm text-zinc-600 w-full"> OR </label>
			<hr class="border-zinc-300 border-1 w-full rounded-md" />
		</div>

		<SecondaryButton
			class="mt-4 w-[23rem] h-12 flex justify-center items-center"
			@click="router.push('/configure')"
		>
			Configure your own instance
		</SecondaryButton>

		<span
			@click="asGuest"
			tabindex="0"
			class="flex cursor-pointer underline mt-6 xl:ml-1 text-md xl:text-lg text-zinc-500 dark:text-zinc-400"
		>
			For privacy, I prefer to continue as guest
			<ChevronRightIcon class="mt-1.5 size-4" />
		</span>
	</div>
</template>
