<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { favoritos } from '$lib/stores/favoritos';
	import { apiClient } from '$lib/services/api-client';
	import type { Favorito, FavoritoInput } from '$lib/types/api';
	import FolderSelector from './FolderSelector.svelte';
	import TagInput from './TagInput.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { AlertCircle, CheckCircle2, Sparkles } from 'lucide-svelte';

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

	let folderId = $state(String(init?.folderId || init?.folder?.id || ""));

	let loading = $state(false);
	let enriching = $state(false);
	let enrichSuccess = $state(false);
	let error = $state('');

	// Estado post-creación: el favorito fue creado y estamos esperando/mostrando enriquecimiento
	let createdId = $state<string | null>(null);
	let postCreateMode = $derived(createdId !== null);
	let duplicateDetected = $state(false);

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
		// No ejecutar en modo edición (ya tenemos datos), ni si estamos enriqueciendo
		if (initialData || enriching) return;
		if (!url || !isValidUrl) return;
		try {
			const metadata = await invoke<any>('metadata_fetch', { url });
			if (metadata.title && !title) title = metadata.title;
			if (metadata.description && !description) description = metadata.description;
		} catch (e) {
			console.error('Error fetching metadata:', e);
		}
	}

	async function enrichWithAI(targetId?: string) {
		const id = targetId || initialData?.id || createdId;
		if (!id) return;
		enriching = true;
		enrichSuccess = false;
		error = '';
		try {
			const refreshed = await invoke<any>('metadata_refresh', { id });
			console.log('[Enrich] Datos enriquecidos:', refreshed);

			// Registrar qué campos cambiaron
			const changes: string[] = [];
			if (refreshed.title && refreshed.title !== title) {
				title = refreshed.title;
				changes.push('título');
			}
			if (refreshed.description && refreshed.description !== description) {
				description = refreshed.description;
				changes.push('descripción');
			}
			if (refreshed.notes && refreshed.notes !== notes) {
				notes = refreshed.notes;
				changes.push('notas');
			}
			if (refreshed.tags && Array.isArray(refreshed.tags) && refreshed.tags.length > 0) {
				const newTags = refreshed.tags.map((t: any) => typeof t === 'string' ? t : t.name);
				if (JSON.stringify(newTags) !== JSON.stringify(tags)) {
					tags = newTags;
					changes.push('etiquetas');
				}
			}

			enrichSuccess = true;
			if (changes.length > 0) {
				console.log('[Enrich] Campos actualizados:', changes.join(', '));
			} else {
				console.log('[Enrich] El servidor no devolvió cambios respecto a los valores actuales');
			}
			setTimeout(() => { enrichSuccess = false; }, 5000);
		} catch (e) {
			error = e instanceof Error ? e.message : String(e);
			console.error('[Enrich Error]', e);
		} finally {
			enriching = false;
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
			if (initialData || postCreateMode) {
				// Modo edición o post-creación: actualizar y cerrar
				const updateId = initialData?.id || createdId!;
				await favoritos.update(updateId, payload);
				if (onSave) onSave();
			} else {
				// Modo creación: crear, luego enriquecer automáticamente
				const newItem = await favoritos.create(payload);
				createdId = newItem.id;
				loading = false;
				// Lanzar enriquecimiento automático en background
				enrichWithAI(newItem.id);
				return; // No cerrar el modal
			}
		} catch (e) {
			const errMsg = e instanceof Error ? e.message : String(e);
			// Detectar conflicto 409: el favorito ya existe
			if (errMsg.includes('409') || errMsg.includes('Conflict')) {
				try {
					// Buscar en el servidor por URL para obtener los datos del duplicado
					const searchResult = await apiClient.getFavorites({ search: url.trim(), limit: 5 });
					const existing = searchResult.items.find(
						(item) => item.url.toLowerCase() === url.trim().toLowerCase()
					);
					if (existing) {
						createdId = existing.id;
						if (existing.title) title = existing.title;
						if (existing.description) description = existing.description;
						if (existing.notes) notes = existing.notes;
						if (existing.tags && existing.tags.length > 0) {
							tags = existing.tags.map(t => typeof t === 'string' ? t : t.name);
						}
						if (existing.folderId || existing.folder?.id) {
							folderId = String(existing.folderId || existing.folder?.id || '');
						}
						duplicateDetected = true;
						error = '';
					} else {
						error = 'Este favorito ya existe, pero no se pudo localizar. Intenta refrescar la lista.';
					}
				} catch {
					error = 'Este favorito ya existe, pero hubo un error al buscar sus datos.';
				}
			} else {
				error = errMsg;
			}
		} finally {
			loading = false;
		}
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4 max-h-[85vh] overflow-y-auto px-1 pr-2 custom-scrollbar bg-white rounded-lg">
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
			disabled={loading || postCreateMode}
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
		<textarea
			id="fav-description"
			bind:value={description}
			placeholder="Breve resumen..."
			disabled={loading}
			rows={3}
			class="flex w-full rounded-md border border-silver bg-white px-3 py-2 text-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-taupe/20 focus-visible:border-taupe disabled:opacity-50"
		></textarea>
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

	{#if duplicateDetected}
		<div class="p-3 bg-blue-50 text-blue-700 text-xs rounded-lg border border-blue-200 flex items-start gap-2 shadow-sm">
			<AlertCircle class="h-4 w-4 shrink-0 mt-0.5" />
			<span>Este favorito ya existe. Se han cargado sus datos para que puedas revisarlos y actualizar lo que necesites.</span>
		</div>
	{/if}

	{#if postCreateMode && enriching}
		<div class="p-3 bg-blue-50 text-blue-700 text-xs rounded-lg border border-blue-200 flex items-start gap-2 shadow-sm">
			<div class="w-4 h-4 border-2 border-blue-300 border-t-blue-600 rounded-full animate-spin shrink-0 mt-0.5"></div>
			<span>Favorito creado. Esperando enriquecimiento del servidor...</span>
		</div>
	{/if}

	{#if enrichSuccess}
		<div class="p-3 bg-amber-50 text-amber-700 text-xs rounded-lg border border-amber-200 flex items-start gap-2 shadow-sm">
			<Sparkles class="h-4 w-4 shrink-0 mt-0.5" />
			<span>Enriquecimiento completado. Revisa los campos actualizados y pulsa "{postCreateMode ? 'Guardar y Cerrar' : 'Actualizar Favorito'}" para guardar los cambios.</span>
		</div>
	{/if}

	{#if error}
		<div class="p-3 bg-red-50 text-red-600 text-xs rounded-lg border border-red-100 flex items-start gap-2 shadow-sm">
			<AlertCircle class="h-4 w-4 shrink-0 mt-0.5" />
			<span>{error}</span>
		</div>
	{/if}

	<div class="flex justify-end gap-2 pt-4 border-t border-silver/20 sticky bottom-0 bg-white pb-2">
		{#if initialData || postCreateMode}
			<Button
				type="button"
				variant="outline"
				class="border-amber-300 text-amber-700 hover:bg-amber-50 font-semibold gap-1.5 active:scale-95 transition-all"
				disabled={loading || enriching}
				onclick={() => enrichWithAI()}
			>
				{#if enriching}
					<div class="w-4 h-4 border-2 border-amber-300 border-t-amber-600 rounded-full animate-spin"></div>
					Enriqueciendo...
				{:else}
					<Sparkles class="w-4 h-4" />
					Enriquecer con IA
				{/if}
			</Button>
		{/if}
		{#if postCreateMode}
			<Button
				type="button"
				variant="outline"
				class="border-greyOlive/40 text-greyOlive hover:bg-greyOlive/10 font-semibold min-w-[100px] active:scale-95 transition-all"
				disabled={loading || enriching}
				onclick={() => { if (onSave) onSave(); }}
			>
				Cerrar sin guardar
			</Button>
		{/if}
		<Button
			type="submit"
			class="bg-taupe hover:bg-taupe/90 text-parchment font-bold min-w-[140px] shadow-sm active:scale-95 transition-all"
			disabled={loading || enriching || !isValidUrl}
		>
			{#if loading}
				<div class="flex items-center gap-2">
					<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
					Guardando...
				</div>
			{:else if postCreateMode}
				Guardar y Cerrar
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
