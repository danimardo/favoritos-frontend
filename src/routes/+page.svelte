<script lang="ts">
	import { onMount } from 'svelte';
	import { favoritos } from '$lib/stores/favoritos';
	import FavoritoGrid from '$lib/components/favoritos/FavoritoGrid.svelte';
	import FavoritoForm from '$lib/components/favoritos/FavoritoForm.svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { isConfigured } from '$lib/stores/configuracion';
	import { Plus } from 'lucide-svelte';

	let open = $state(false);
	let gridComponent = $state<any>(null);

	onMount(() => {
		window.addEventListener('keydown', handleKeydown);
		return () => window.removeEventListener('keydown', handleKeydown);
	});

	// Carga inicial: cuando estemos configurados, cargamos los favoritos (solo una vez)
	let initialLoadDone = false;
	$effect(() => {
		if ($isConfigured && !initialLoadDone) {
			initialLoadDone = true;
			console.log('[HomePage] App configurada, cargando favoritos...');
			favoritos.loadFoldersAndTags();
			favoritos.fetchPage(true);
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		// Ctrl + N (Nuevo favorito)
		if (e.ctrlKey && e.key === 'n') {
			e.preventDefault();
			open = true;
		}
		// Tecla + (Nuevo favorito, solo si no estamos en un input/textarea)
		if (e.key === '+' && !e.ctrlKey && !e.altKey && !e.metaKey) {
			const tag = (e.target as HTMLElement)?.tagName;
			if (tag !== 'INPUT' && tag !== 'TEXTAREA' && !(e.target as HTMLElement)?.isContentEditable) {
				e.preventDefault();
				open = true;
			}
		}
		// Ctrl + F (Buscar)
		if (e.ctrlKey && e.key === 'f') {
			e.preventDefault();
			if (gridComponent) gridComponent.focusSearch();
		}
		// Suprimir (Limpiar filtros)
		if (e.key === 'Delete' && !e.ctrlKey && !e.altKey && !e.metaKey) {
			const tag = (e.target as HTMLElement)?.tagName;
			if (tag !== 'INPUT' && tag !== 'TEXTAREA' && !(e.target as HTMLElement)?.isContentEditable) {
				e.preventDefault();
				favoritos.selectFolder(null);
			}
		}
	}

	function handleSave() {
		open = false;
	}

	// Título dinámico según filtro activo
	let pageTitle = $derived.by(() => {
		if ($favoritos.selectedTag) return `#${$favoritos.selectedTag}`;
		if ($favoritos.selectedFolderId) {
			const folder = $favoritos.folders.find(f => String(f.id) === String($favoritos.selectedFolderId));
			return folder ? folder.name : 'Favoritos';
		}
		return 'Mis Favoritos';
	});
</script>

<!-- Cabecera compacta -->
<div class="mb-5 flex items-end justify-between gap-4">
	<div>
		<h1 class="text-[22px] font-bold text-taupe tracking-tight leading-none">{pageTitle}</h1>
		{#if !$favoritos.selectedTag && !$favoritos.selectedFolderId}
			<p class="text-[11.5px] text-greyOlive/70 mt-1">Gestiona tus enlaces importantes de forma segura.</p>
		{/if}
	</div>
</div>

<FavoritoGrid bind:this={gridComponent} />

<!-- FAB: Nuevo Favorito -->
<button
	onclick={() => open = true}
	class="fixed bottom-6 right-6 z-50 w-12 h-12 rounded-full bg-taupe text-parchment hover:bg-taupe/90 shadow-lg hover:shadow-xl active:scale-95 transition-all flex items-center justify-center"
	aria-label="Nuevo Favorito"
	title="Nuevo Favorito (Ctrl+N o +)"
>
	<Plus class="w-5 h-5" />
</button>

<Dialog.Root bind:open>
	<Dialog.Content class="bg-white border-silver/50 shadow-2xl opacity-100">
		<Dialog.Header>
			<Dialog.Title class="text-taupe">Nuevo Favorito</Dialog.Title>
			<Dialog.Description class="text-greyOlive">
				Añade un nuevo enlace a tu colección.
			</Dialog.Description>
		</Dialog.Header>
		<FavoritoForm onSave={handleSave} />
	</Dialog.Content>
</Dialog.Root>
