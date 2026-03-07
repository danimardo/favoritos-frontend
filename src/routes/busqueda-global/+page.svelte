<script lang="ts">
	import BuscadorGlobal from '$lib/components/favoritos/BuscadorGlobal.svelte';
	import { onMount } from 'svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';

	const appWindow = getCurrentWindow();

	onMount(() => {
		// Ensure the window is focused when opened
		appWindow.setFocus();

		// Handle ESC key to hide window
		const handleKeydown = (e: KeyboardEvent) => {
			if (e.key === 'Escape') {
				appWindow.hide();
			}
		};

		window.addEventListener('keydown', handleKeydown);
		
		// Hide when losing focus (Spotlight style)
		const unlisten = appWindow.onFocusChanged(({ payload: focused }) => {
			if (!focused) {
				appWindow.hide();
			}
		});

		return () => {
			window.removeEventListener('keydown', handleKeydown);
			unlisten.then(u => u());
		};
	});
</script>

<svelte:head>
	<title>Búsqueda Global</title>
</svelte:head>

<main class="h-screen w-screen overflow-hidden bg-transparent p-4">
	<BuscadorGlobal />
</main>

<style>
	:global(body) {
		background-color: transparent !important;
		margin: 0;
		padding: 0;
		overflow: hidden;
	}
</style>
