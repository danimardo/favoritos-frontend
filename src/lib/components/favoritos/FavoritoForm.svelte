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
	let tags = $state<string[]>(init?.tags || []);
	let folderId = $state(init?.folder || "");
	let loading = $state(false);
	let error = $state('');

	async function fetchMetadata() {
		if (!url || title) return; // Don't overwrite existing title
		try {
			const metadata = await invoke<any>('metadata_fetch', { url });
			if (metadata.title) title = metadata.title;
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

<form onsubmit={handleSubmit} class="space-y-4">
	<div class="space-y-2">
		<Label for="fav-url">URL</Label>
		<Input 
			id="fav-url" 
			bind:value={url} 
			placeholder="https://..." 
			required 
			disabled={loading}
			onblur={fetchMetadata}
		/>
	</div>

	<div class="space-y-2">
		<Label for="fav-title">Título</Label>
		<Input id="fav-title" bind:value={title} placeholder="Mi sitio favorito" required disabled={loading} />
	</div>

	<div class="space-y-2">
		<Label>Carpeta</Label>
		<FolderSelector bind:value={folderId} />
	</div>

	<div class="space-y-2">
		<Label for="fav-tags">Etiquetas</Label>
		<TagInput id="fav-tags" bind:tags={tags} />
	</div>

	{#if error}
		<p class="text-sm text-red-600">{error}</p>
	{/if}

	<div class="flex justify-end gap-2 pt-2">
		<Button type="submit" class="bg-taupe text-parchment w-full sm:w-auto" disabled={loading || !url || !title}>
			{loading ? 'Guardando...' : (initialData ? 'Actualizar' : 'Crear')}
		</Button>
	</div>
</form>
