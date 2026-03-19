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
</script>

<div class="space-y-8">
	<div>
		<h1 class="text-4xl font-bold text-taupe">Mis Favoritos</h1>
		<p class="text-greyOlive mt-2">Gestiona tus enlaces importantes de forma segura.</p>
	</div>

	<FavoritoGrid bind:this={gridComponent} />
</div>

<!-- FAB: Nuevo Favorito (flotante abajo-derecha) -->
<button
	onclick={() => open = true}
	class="fixed bottom-6 right-6 z-50 w-14 h-14 rounded-full bg-taupe/80 text-parchment hover:bg-taupe/95 shadow-lg hover:shadow-xl active:scale-95 transition-all flex items-center justify-center backdrop-blur-sm"
	aria-label="Nuevo Favorito"
	title="Nuevo Favorito (Ctrl+N o +)"
>
	<Plus class="w-7 h-7" />
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
