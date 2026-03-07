<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { configuracion, isConfigured } from '$lib/stores/configuracion';
	import { invoke } from '@tauri-apps/api/core';

	onMount(() => {
		configuracion.init().then(() => checkConfig());

		// Global shortcuts
		window.addEventListener('keydown', handleGlobalKeydown);
		return () => window.removeEventListener('keydown', handleGlobalKeydown);
	});

	function handleGlobalKeydown(e: KeyboardEvent) {
		// Ctrl + , (Configuración)
		if (e.ctrlKey && e.key === ',') {
			e.preventDefault();
			goto('/configuracion');
		}
		// Ctrl + Q (Salir)
		if (e.ctrlKey && e.key === 'q') {
			e.preventDefault();
			invoke('app_quit');
		}
		// Esc (Minimizar al tray)
		if (e.key === 'Escape') {
			// Check if we are not in the global search window (which handles ESC on its own)
			const isSearchPage = window.location.pathname === '/busqueda-global';
			if (!isSearchPage) {
				// Only minimize if nothing else (like a dialog) has focused or handled the key
				// Wait, in SvelteKit with Shadcn, dialogs usually handle ESC themselves.
				// We can just call hide and if a dialog was open, it might close too, but the window will hide.
				// Actually, many Spotlight apps use ESC to close.
				invoke('window_hide_main');
			}
		}
	}

	$: if ($isConfigured !== undefined) {
		checkConfig();
	}

	function checkConfig() {
		// Only redirect if not already in configuration or welcome
		const isConfigPage = $page.url.pathname === '/configuracion';
		if (!$isConfigured && !isConfigPage) {
			goto('/configuracion');
		}
	}
</script>

<div class="min-h-screen bg-parchment text-taupe selection:bg-powderBlush/30 selection:text-taupe">
	<main class="container mx-auto py-8">
		<slot />
	</main>
</div>
