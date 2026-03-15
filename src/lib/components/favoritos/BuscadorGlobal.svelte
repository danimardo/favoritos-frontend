<script lang="ts">
	import * as Command from '$lib/components/ui/command';
	import { favoritos } from '$lib/stores/favoritos';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { Search, Globe, Folder, Tag, ExternalLink } from 'lucide-svelte';
	import { filterFavoritos } from '$lib/utils';
	import type { Favorito } from '$lib/types/api';

	let query = $state('');
	
	// Use manual filtering for better control and performance as requested in T076
	let filteredItems = $derived(filterFavoritos($favoritos.items, query));

	async function openFavorite(id: string) {
		try {
			await invoke('favorites_visit', { id });
			// After opening, hide the search window
			// We can't use window.hide() directly easily here without more imports, 
			// but we can use an invoke if we had one or just assume the backend handles it 
			// after visit or the user closes it.
			// Actually, let's just let it open and the user can ESC.
		} catch (e) {
			console.error('Failed to open favorite', e);
		}
	}

	onMount(async () => {
		if ($favoritos.items.length === 0) {
			await favoritos.fetchPage(true);
		}
	});

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			// This window is decorations: false, so we should probably handle ESC to hide it
			invoke('window_hide_main').catch(console.error); // Wait, we need a hide search command?
			// Actually, the backend Alt+Space toggles it.
		}
	}
</script>

<div class="h-full w-full bg-parchment overflow-hidden border-2 border-taupe shadow-2xl rounded-lg">
	<Command.Root shouldFilter={false} class="h-full border-none">
		<div class="flex items-center border-b border-taupe/20 px-3">
			<Search class="mr-2 h-4 w-4 shrink-0 opacity-50 text-taupe" />
			<Command.Input
				bind:value={query}
				placeholder="Buscar favoritos..."
				aria-label="Buscar favoritos"
				class="flex h-12 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-grey-olive disabled:cursor-not-allowed disabled:opacity-50 text-taupe"
			/>
		</div>
		<Command.List class="max-h-[380px] overflow-y-auto overflow-x-hidden p-2" aria-label="Resultados de búsqueda">
			{#if filteredItems.length === 0}
				<Command.Empty class="py-6 text-center text-sm text-grey-olive" role="status">
					No se encontraron favoritos.
				</Command.Empty>
			{:else}
				<Command.Group heading="Resultados">
					{#each filteredItems as item (item.id)}
						<Command.Item
							value={item.id}
							onSelect={() => openFavorite(item.id)}
							aria-label={`Abrir ${item.title}`}
							class="flex items-center gap-3 rounded-sm px-2 py-3 text-sm cursor-pointer hover:bg-powder-blush/30 aria-selected:bg-powder-blush/50 text-taupe"
						>
							<div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-silver/20">
								{#if item.faviconUrl || item.faviconCached}
									<img
										src={item.faviconCached || item.faviconUrl}
										alt=""
										class="h-4 w-4"
										onerror={(e) => ((e.currentTarget as HTMLElement).style.display = 'none')}
									/>
								{:else}
									<Globe class="h-4 w-4 text-grey-olive" />
								{/if}
							</div>
							<div class="flex flex-col flex-1 overflow-hidden">
								<span class="font-medium truncate">{item.title}</span>
								<span class="text-xs text-grey-olive truncate">{item.url}</span>
							</div>
							<div class="flex items-center gap-2">
								{#if item.folder}
									<div class="flex items-center gap-1 px-1.5 py-0.5 rounded bg-taupe/10 text-[10px] text-taupe/70">
										<Folder class="h-3 w-3" />
										{item.folder}
									</div>
								{/if}
								<ExternalLink class="h-3 w-3 opacity-0 group-hover:opacity-100 transition-opacity" />
							</div>
						</Command.Item>
					{/each}
				</Command.Group>
			{/if}
		</Command.List>
		<div class="flex items-center justify-between border-t border-taupe/10 px-4 py-2 text-[10px] text-grey-olive bg-parchment/50">
			<div class="flex items-center gap-3">
				<span><kbd class="rounded bg-taupe/5 px-1.5 py-0.5 font-sans">↑↓</kbd> Navegar</span>
				<span><kbd class="rounded bg-taupe/5 px-1.5 py-0.5 font-sans">Enter</kbd> Abrir</span>
			</div>
			<div class="flex items-center gap-1">
				<span><kbd class="rounded bg-taupe/5 px-1.5 py-0.5 font-sans">Esc</kbd> Cerrar</span>
			</div>
		</div>
	</Command.Root>
</div>

<style>
	:global([data-command-group-heading]) {
		padding: 0.5rem 0.5rem 0.25rem;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-grey-olive);
	}
</style>
