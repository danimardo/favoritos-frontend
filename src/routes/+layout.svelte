<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { configuracion, isConfigured } from '$lib/stores/configuracion';
	import { favoritos } from '$lib/stores/favoritos';
	import { invoke } from '@tauri-apps/api/core';
	import FolderSidebar from '$lib/components/favoritos/FolderSidebar.svelte';

	onMount(() => {
		configuracion.init().then(() => checkConfig());
		favoritos.restoreViewMode();
		favoritos.restoreNavigation();

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
		// Esc (Minimizar al tray, solo si no hay modales abiertos)
		if (e.key === 'Escape') {
			const isSearchPage = window.location.pathname === '/busqueda-global';
			const hasOpenDialog = document.querySelector('[role="dialog"], [data-dialog-overlay]');
			if (!isSearchPage && !hasOpenDialog) {
				invoke('window_hide_main');
			}
		}
	}

	$effect(() => {
		if ($configuracion.ready) {
			checkConfig();
		}
	});

	function checkConfig() {
		if (!$configuracion.ready) return;
		
		// Only redirect if not already in configuration or search
		const isConfigPage = $page.url.pathname === '/configuracion';
		const isSearchPage = $page.url.pathname === '/busqueda-global';
		
		if (!isConfigPage && !isSearchPage && !$isConfigured) {
			goto('/configuracion');
		}
	}

	const isSearchPage = $derived($page.url.pathname === '/busqueda-global');
</script>

<div class="min-h-screen bg-parchment text-taupe selection:bg-powderBlush/30 selection:text-taupe relative font-sans flex">
	{#if !$configuracion.ready}
		<div class="fixed inset-0 flex items-center justify-center bg-parchment z-[10000]">
			<div class="flex flex-col items-center gap-4">
				<div class="w-12 h-12 border-4 border-taupe/20 border-t-taupe rounded-full animate-spin"></div>
				<p class="text-taupe font-medium animate-pulse">Cargando aplicación...</p>
			</div>
		</div>
	{/if}

	<!-- Sidebar (Solo si está configurado y no es búsqueda global) -->
	{#if $isConfigured && !isSearchPage}
		<aside class="sticky top-0 h-screen hidden md:block">
			<FolderSidebar />
		</aside>
	{/if}

	<!-- Contenido Principal -->
	<div class="flex-1 flex flex-col h-screen overflow-hidden">
		<main class="flex-1 overflow-y-auto custom-scrollbar p-4 md:p-8">
			<div class="max-w-[1600px] mx-auto h-full">
				<slot />
			</div>
		</main>
	</div>
</div>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 8px;
	}
	.custom-scrollbar-track {
		background: transparent;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: #C0C0C0;
		border-radius: 10px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb:hover {
		background: #A9A9A1;
	}
</style>
