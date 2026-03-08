<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { favoritos } from '$lib/stores/favoritos';
	import type { Favorito, FavoritoInput } from '$lib/types/api';
	import FolderSelector from './FolderSelector.svelte';
	import TagInput from './TagInput.svelte';
	import { invoke } from '@tauri-apps/api/core';

	let { onSave, initialData = null }: { onSave?: () => void, initialData?: Favorito | null } = $props();

	const init = initialData;
	let url = $state(init?.url || '');
	let title = $state(init?.title || '');
	let description = $state(init?.description || '');
	let notes = $state(init?.notes || '');
	
	// Normalización: Extraer nombres si son objetos, o usar el array si son strings
	let tags = $state<string[]>(
		init?.tags 
			? init.tags.map(t => typeof t === 'string' ? t : t.name) 
			: []
	);
	
	// Normalización: Usar folderId si existe, o el id del objeto folder
	let folderId = $state(init?.folderId || init?.folder?.id || "");
	
	let loading = $state(false);
	let error = $state('');

	async function fetchMetadata() {
		if (!url) return;
		try {
			const metadata = await invoke<any>('metadata_fetch', { url });
			if (metadata.title && !title) title = metadata.title;
			if (metadata.description && !description) description = metadata.description;
		} catch (e) {
			console.error('Error fetching metadata:', e);
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		loading = true;
		error = '';

		const payload: FavoritoInput = { 
			url, 
			title,
			description: description || undefined,
			notes: notes || undefined,
			tags,
			folder: folderId || undefined
		};

		try {
			if (initialData) {
				await favoritos.update(initialData.id, payload);
			} else {
				await favoritos.create(payload);
			}
			if (onSave) onSave();
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
		} finally {
			loading = false;
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4 max-h-[70vh] overflow-y-auto px-1 pr-2 custom-scrollbar bg-white rounded-lg">
	<div class="space-y-2">
		<Label for="fav-url" class="text-taupe font-semibold">URL</Label>
		<Input 
			id="fav-url" 
			bind:value={url} 
			placeholder="https://ejemplo.com" 
			required 
			disabled={loading}
			onblur={fetchMetadata}
			class="focus:ring-taupe focus:border-taupe bg-white"
		/>
	</div>

	<div class="space-y-2">
		<Label for="fav-title" class="text-taupe font-semibold">Título</Label>
		<Input id="fav-title" bind:value={title} placeholder="Mi sitio favorito" required disabled={loading} class="focus:ring-taupe focus:border-taupe bg-white" />
	</div>

	<div class="space-y-2">
		<Label for="fav-description" class="text-taupe font-semibold">Descripción Corta</Label>
		<Input id="fav-description" bind:value={description} placeholder="De qué trata este sitio..." disabled={loading} class="focus:ring-taupe focus:border-taupe bg-white" />
	</div>

	<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
		<div class="space-y-2">
			<Label class="text-taupe font-semibold">Carpeta</Label>
			<FolderSelector bind:value={folderId} />
		</div>

		<div class="space-y-2">
			<Label for="fav-tags" class="text-taupe font-semibold">Etiquetas</Label>
			<TagInput id="fav-tags" bind:tags={tags} />
		</div>
	</div>

	<div class="space-y-2">
		<Label for="fav-notes" class="text-taupe font-semibold">Notas Personales</Label>
		<textarea 
			id="fav-notes" 
			bind:value={notes} 
			placeholder="Anotaciones, contraseñas temporales, por qué guardé esto..." 
			disabled={loading}
			rows={3}
			class="flex min-h-[80px] w-full rounded-md border border-silver bg-white px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-taupe/20 focus-visible:border-taupe disabled:cursor-not-allowed disabled:opacity-50"
		></textarea>
	</div>

	{#if error}
		<div class="p-3 bg-red-50 text-red-600 text-xs rounded border border-red-100">
			{error}
		</div>
	{/if}

	<div class="flex justify-end gap-2 pt-4 border-t border-silver/20 sticky bottom-0 bg-white pb-2">
		<Button type="submit" class="bg-taupe hover:bg-taupe/90 text-parchment font-bold min-w-[120px]" disabled={loading || !url || !title}>
			{#if loading}
				Guardando...
			{:else}
				{initialData ? 'Actualizar Favorito' : 'Crear Favorito'}
			{/if}
		</Button>
	</div>
</form>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 6px;
	}
	.custom-scrollbar::-webkit-scrollbar-track {
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
