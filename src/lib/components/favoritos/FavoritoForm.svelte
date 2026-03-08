<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { favoritos } from '$lib/stores/favoritos';
	import type { Favorito, FavoritoInput } from '$lib/types/api';
	import FolderSelector from './FolderSelector.svelte';
	import TagInput from './TagInput.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { AlertCircle, CheckCircle2 } from 'lucide-svelte';

	let { onSave, initialData = null }: { onSave?: () => void, initialData?: Favorito | null } = $props();

	const init = initialData;
	let url = $state(init?.url || '');
	let title = $state(init?.title || '');
	let description = $state(init?.description || '');
	let notes = $state(init?.notes || '');
	
	let tags = $state<string[]>(
		init?.tags 
			? init.tags.map(t => typeof t === 'string' ? t : t.name) 
			: []
	);
	
	let folderId = $state(init?.folderId || init?.folder?.id || "");
	
	let loading = $state(false);
	let error = $state('');

	// VALIDACIÓN DE URL ROBUSTA
	// Usamos el constructor URL nativo para máxima fiabilidad
	let isValidUrl = $derived.by(() => {
		const trimmedUrl = url.trim();
		if (trimmedUrl.length < 4) return false;
		
		try {
			// Si no tiene protocolo, le añadimos uno temporal para validar la estructura
			const urlToTest = trimmedUrl.includes('://') ? trimmedUrl : `https://${trimmedUrl}`;
			const parsed = new URL(urlToTest);
			// Verificamos que tenga al menos un punto en el host (dominio)
			return parsed.hostname.includes('.');
		} catch {
			return false;
		}
	});

	async function fetchMetadata() {
		if (!url || !isValidUrl) return;
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
		
		if (!isValidUrl) {
			error = 'La URL introducida no tiene un formato válido.';
			return;
		}

		loading = true;
		error = '';

		const payload: FavoritoInput = { 
			url: url.trim(), 
			title: title.trim(),
			description: description.trim(),
			notes: notes.trim(),
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
	<!-- CAMPO URL: ÚNICO OBLIGATORIO -->
	<div class="space-y-2">
		<div class="flex justify-between items-center">
			<Label for="fav-url" class="text-taupe font-bold">URL del Sitio</Label>
			{#if url.trim().length > 0}
				{#if isValidUrl}
					<span class="text-[10px] text-green-600 font-medium flex items-center gap-1 animate-in fade-in">
						<CheckCircle2 class="w-3 h-3" /> URL Válida
					</span>
				{:else}
					<span class="text-[10px] text-red-500 font-medium flex items-center gap-1 animate-in shake-1">
						<AlertCircle class="w-3 h-3" /> Formato incompleto
					</span>
				{/if}
			{/if}
		</div>
		<Input 
			id="fav-url" 
			bind:value={url} 
			placeholder="https://ejemplo.com" 
			required 
			disabled={loading}
			onblur={fetchMetadata}
			class="focus:ring-taupe focus:border-taupe bg-white {url.trim().length > 0 && !isValidUrl ? 'border-red-300' : ''}"
		/>
	</div>

	<!-- TÍTULO: OPCIONAL -->
	<div class="space-y-2">
		<Label for="fav-title" class="text-taupe font-semibold">Título (Opcional)</Label>
		<Input 
			id="fav-title" 
			bind:value={title} 
			placeholder="Se autorrellenará desde la web si se deja vacío" 
			disabled={loading} 
			class="focus:ring-taupe focus:border-taupe bg-white text-sm" 
		/>
	</div>

	<div class="space-y-2">
		<Label for="fav-description" class="text-taupe font-semibold text-xs text-greyOlive">Descripción Corta</Label>
		<Input id="fav-description" bind:value={description} placeholder="Breve resumen..." disabled={loading} class="focus:ring-taupe focus:border-taupe bg-white h-9 text-sm" />
	</div>

	<div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
		<div class="space-y-2">
			<Label class="text-taupe font-semibold text-xs text-greyOlive">Carpeta</Label>
			<FolderSelector bind:value={folderId} />
		</div>

		<div class="space-y-2">
			<Label for="fav-tags" class="text-taupe font-semibold text-xs text-greyOlive">Etiquetas</Label>
			<TagInput id="fav-tags" bind:tags={tags} />
		</div>
	</div>

	<div class="space-y-2">
		<Label for="fav-notes" class="text-taupe font-semibold text-xs text-greyOlive">Notas Personales</Label>
		<textarea 
			id="fav-notes" 
			bind:value={notes} 
			placeholder="Tus notas aquí..." 
			disabled={loading}
			rows={8}
			class="flex min-h-[180px] w-full rounded-md border border-silver bg-white px-3 py-2 text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-taupe/20 focus-visible:border-taupe disabled:opacity-50"
		></textarea>
	</div>

	{#if error}
		<div class="p-3 bg-red-50 text-red-600 text-xs rounded-lg border border-red-100 flex items-start gap-2 shadow-sm">
			<AlertCircle class="h-4 w-4 shrink-0 mt-0.5" />
			<span>{error}</span>
		</div>
	{/if}

	<div class="flex justify-end gap-2 pt-4 border-t border-silver/20 sticky bottom-0 bg-white pb-2">
		<Button 
			type="submit" 
			class="bg-taupe hover:bg-taupe/90 text-parchment font-bold min-w-[140px] shadow-sm active:scale-95 transition-all" 
			disabled={loading || !isValidUrl}
		>
			{#if loading}
				<div class="flex items-center gap-2">
					<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
					Guardando...
				</div>
			{:else}
				{initialData ? 'Actualizar Favorito' : 'Crear Favorito'}
			{/if}
		</Button>
	</div>
</form>

<style>
	.custom-scrollbar::-webkit-scrollbar { width: 6px; }
	.custom-scrollbar::-webkit-scrollbar-thumb { background: #C0C0C0; border-radius: 10px; }
</style>
