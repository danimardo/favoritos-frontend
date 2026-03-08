<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { configuracion, isConfigured } from '$lib/stores/configuracion';
	import { favoritos } from '$lib/stores/favoritos';
	import { invoke } from '@tauri-apps/api/core';
	import { RefreshCw, Settings } from 'lucide-svelte';

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
			const isSearchPage = window.location.pathname === '/busqueda-global';
			if (!isSearchPage) {
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
	const isConfigPage = $derived($page.url.pathname === '/configuracion');

	let refreshing = $state(false);

	async function handleRefresh() {
		if (refreshing) return;
		refreshing = true;
		try {
			await favoritos.fetchAll();
		} finally {
			// Añadimos un pequeño retardo artificial para que la animación se vea bien
			setTimeout(() => {
				refreshing = false;
			}, 800);
		}
	}
</script>

<div class="min-h-screen bg-parchment text-taupe selection:bg-powderBlush/30 selection:text-taupe relative font-sans">
	{#if !$configuracion.ready}
		<div class="fixed inset-0 flex items-center justify-center bg-parchment z-[10000]">
			<div class="flex flex-col items-center gap-4">
				<div class="w-12 h-12 border-4 border-taupe/20 border-t-taupe rounded-full animate-spin"></div>
				<p class="text-taupe font-medium animate-pulse">Cargando aplicación...</p>
			</div>
		</div>
	{/if}

	<main class="container mx-auto py-8 px-4">
		<slot />
	</main>

	{#if !isSearchPage && !isConfigPage && $isConfigured}
		<div class="fixed bottom-6 right-6 flex items-center gap-3 z-[9999]">
			<!-- Botón de Refresco (Blanco) -->
			<button
				onclick={handleRefresh}
				class="w-12 h-12 bg-white text-taupe border-2 border-taupe/10 rounded-full shadow-lg hover:shadow-xl hover:scale-110 active:scale-95 transition-all flex items-center justify-center group disabled:opacity-70"
				title="Refrescar datos del servidor"
				aria-label="Refrescar"
				disabled={refreshing}
			>
				<RefreshCw class="w-5 h-5 {refreshing ? 'animate-spin' : 'group-hover:rotate-180 transition-transform duration-700'}" />
			</button>

			<!-- Botón de Configuración (Taupe) -->
			<button
				onclick={() => goto('/configuracion')}
				class="w-12 h-12 bg-taupe text-parchment rounded-full shadow-lg hover:shadow-xl hover:scale-110 active:scale-95 transition-all flex items-center justify-center group"
				title="Ajustes y Perfil"
				aria-label="Ir a configuración"
			>
				<Settings class="w-6 h-6 group-hover:rotate-90 transition-transform duration-500" />
			</button>
		</div>
	{/if}
</div>
