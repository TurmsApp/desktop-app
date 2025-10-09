<template>
	<button
		type="button"
		tabindex="0"
		class="flex justify-center text-sm py-2 cursor-pointer rounded border border-black dark:border-zinc-800 enabled:hover:bg-zinc-900 font-medium text-black enabled:hover:text-white transition-all duration-300 disabled:cursor-not-allowed"
	>
		<slot />
	</button>
</template>
