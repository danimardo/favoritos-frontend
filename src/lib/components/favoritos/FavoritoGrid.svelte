<script lang="ts">
	import FavoritoCard from './FavoritoCard.svelte';
	import { favoritos } from '$lib/stores/favoritos';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { LayoutGrid, List, Search, ExternalLink, Hash, Pencil, Trash2, XCircle } from 'lucide-svelte';
	import { apiClient } from '$lib/services/api-client';
	import * as Dialog from '$lib/components/ui/dialog';
	import FavoritoForm from './FavoritoForm.svelte';
	import DeleteConfirmDialog from './DeleteConfirmDialog.svelte';
	import type { Favorito } from '$lib/types/api';
	import OverflowTags from './OverflowTags.svelte';
	import { onMount } from 'svelte';

	let searchInput = $state<HTMLInputElement | null>(null);

	let selectedFavorito = $state<Favorito | null>(null);
	let showEditDialog = $state(false);
	let showDeleteDialog = $state(false);
	let sentinelRef = $state<HTMLDivElement | null>(null);
	let debounceTimer: ReturnType<typeof setTimeout>;

	export function focusSearch() {
		if (searchInput) searchInput.focus();
	}

	function handleEdit(fav: Favorito) {
		selectedFavorito = fav;
		showEditDialog = true;
	}

	function handleDeleteClick(fav: Favorito) {
		selectedFavorito = fav;
		showDeleteDialog = true;
	}

	async function confirmDelete() {
		if (selectedFavorito) {
			await favoritos.delete(selectedFavorito.id);
			showDeleteDialog = false;
		}
	}

	function handleSearchInput(value: string) {
		favoritos.setSearchQuery(value);
		clearTimeout(debounceTimer);
		debounceTimer = setTimeout(() => {
			favoritos.executeSearch();
		}, 400);
	}

	function clearSearch() {
		favoritos.setSearchQuery('');
		favoritos.executeSearch();
		focusSearch();
	}

	// Handle tag click in list view - search by tag
	function handleTagClick(tag: any) {
		const tagName = typeof tag === 'string' ? tag : tag.name;
		favoritos.setSearchQuery(tagName);
		favoritos.executeSearch();
	}

	// Log state for debugging pagination
	$effect(() => {
		console.log(`[Grid] hasMore=${$favoritos.hasMore}, items=${$favoritos.items.length}, total=${$favoritos.total}, page=${$favoritos.page}, totalPages=${$favoritos.totalPages}`);
	});

	// IntersectionObserver for infinite scroll (viewport-based)
	$effect(() => {
		if (sentinelRef) {
			console.log('[Grid] Observer enganchado al sentinel');
			const observer = new IntersectionObserver((entries) => {
				const entry = entries[0];
				console.log(`[Grid] Sentinel intersecting=${entry.isIntersecting}, hasMore=${$favoritos.hasMore}, loadingMore=${$favoritos.loadingMore}`);
				if (entry.isIntersecting && $favoritos.hasMore && !$favoritos.loadingMore && !$favoritos.loading) {
					console.log('[Grid] Cargando siguiente página...');
					favoritos.fetchPage(false);
				}
			}, { rootMargin: '300px' });
			observer.observe(sentinelRef);
			return () => observer.disconnect();
		}
	});

	function loadMore() {
		if ($favoritos.hasMore && !$favoritos.loadingMore) {
			favoritos.fetchPage(false);
		}
	}
</script>

<div class="space-y-6">
	<div class="flex flex-col sm:flex-row items-center gap-4 bg-white/50 p-2 rounded-xl border border-silver/20">
		<div class="relative flex-1 w-full group">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-greyOlive" />
			<Input
				bind:ref={searchInput}
				placeholder="Buscar favoritos..."
				value={$favoritos.searchQuery}
				oninput={(e) => handleSearchInput((e.target as HTMLInputElement).value)}
				class="pl-10 pr-10 bg-white border-silver/50 focus:border-taupe h-11"
			/>
			{#if $favoritos.searchQuery}
				<button
					type="button"
					onclick={clearSearch}
					class="absolute right-3 top-1/2 -translate-y-1/2 text-greyOlive hover:text-taupe transition-colors p-1 z-10"
					title="Limpiar búsqueda"
				>
					<XCircle class="w-4 h-4" />
				</button>
			{/if}
		</div>

		<div class="flex items-center gap-1 bg-silver/20 p-1 rounded-lg">
			<Button
				variant="ghost"
				size="icon"
				class="h-9 w-9 {$favoritos.viewMode === 'grid' ? 'bg-white text-taupe shadow-sm' : 'text-greyOlive'}"
				onclick={() => favoritos.setViewMode('grid')}
			>
				<LayoutGrid class="w-4 h-4" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				class="h-9 w-9 {$favoritos.viewMode === 'list' ? 'bg-white text-taupe shadow-sm' : 'text-greyOlive'}"
				onclick={() => favoritos.setViewMode('list')}
			>
				<List class="w-4 h-4" />
			</Button>
		</div>
	</div>

	<!-- Info de paginación -->
	{#if $favoritos.total > 0}
		<p class="text-xs text-greyOlive text-right">
			Mostrando {$favoritos.items.length} de {$favoritos.total} favoritos
		</p>
	{/if}

	{#if $favoritos.viewMode === 'grid'}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-4">
			{#each $favoritos.items as favorito (favorito.id)}
				<FavoritoCard {favorito} />
			{:else}
				{#if !$favoritos.loading}
					<div class="col-span-full py-20 text-center text-greyOlive border-2 border-dashed border-silver/30 rounded-2xl bg-silver/5">
						<Search class="w-12 h-12 mx-auto mb-4 opacity-20" />
						{#if $favoritos.searchQuery}
							<p class="text-lg">No se encontró "<span class="text-taupe font-bold">{$favoritos.searchQuery}</span>"</p>
							<p class="text-xs mt-2 italic">Prueba con otros términos de búsqueda</p>
						{:else}
							<p class="text-lg">No hay favoritos en esta vista</p>
						{/if}
					</div>
				{/if}
			{/each}
		</div>
	{:else}
		<div class="bg-white rounded-xl border border-silver/50 overflow-hidden shadow-sm">
			<table class="w-full text-left border-collapse">
				<thead class="bg-silver/10 text-[10px] uppercase tracking-wider text-greyOlive font-bold border-b border-silver/30">
					<tr>
						<th class="px-4 py-3 w-8"></th>
						<th class="px-4 py-3">Título / URL</th>
						<th class="px-4 py-3 hidden md:table-cell">Descripción</th>
						<th class="px-4 py-3 hidden lg:table-cell">Etiquetas</th>
						<th class="px-4 py-3 text-right">Acciones</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-silver/20">
					{#each $favoritos.items as favorito (favorito.id)}
						<tr
							class="hover:bg-silver/5 transition-colors group cursor-default select-none"
							ondblclick={() => handleEdit(favorito)}
						>
							<td class="px-4 py-3">
								<div class="w-6 h-6 bg-silver/10 rounded flex items-center justify-center text-[10px]">
									{#if favorito.faviconUrl}
										<img src={favorito.faviconUrl} alt="" class="w-4 h-4" />
									{:else}
										<Hash class="w-3 h-3 text-greyOlive" />
									{/if}
								</div>
							</td>
							<td class="px-4 py-3">
								<div class="flex flex-col">
									<button
										onclick={() => apiClient.openUrl(favorito.url)}
										title={favorito.title || ''}
										class="text-sm font-bold text-taupe hover:underline text-left truncate max-w-[200px] md:max-w-[300px]"
									>
										{favorito.title || 'Sin título'}
									</button>
									<span class="text-[10px] text-greyOlive truncate max-w-[200px]">{favorito.domain}</span>
								</div>
							</td>
							<td class="px-4 py-3 hidden md:table-cell">
								<p class="text-xs text-greyOlive line-clamp-1 italic" title={favorito.description || ''}>{favorito.description || '-'}</p>
							</td>
							<td class="px-4 py-3 hidden lg:table-cell">
								{#if (favorito.tags || []).length > 0}
									<OverflowTags tags={favorito.tags || []} onTagClick={handleTagClick} />
								{/if}
							</td>
							<td class="px-4 py-3 text-right">
								<div class="flex justify-end gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
									<Button variant="ghost" size="icon" class="h-8 w-8 text-red-400 hover:text-red-600" onclick={() => handleDeleteClick(favorito)} title="Eliminar">
										<Trash2 class="w-4 h-4" />
									</Button>
									<Button variant="ghost" size="icon" class="h-8 w-8 text-greyOlive hover:text-taupe" onclick={() => handleEdit(favorito)} title="Editar">
										<Pencil class="w-4 h-4" />
									</Button>
									<Button variant="ghost" size="icon" class="h-8 w-8 text-greyOlive hover:text-taupe" onclick={() => apiClient.openUrl(favorito.url)} title="Abrir">
										<ExternalLink class="w-4 h-4" />
									</Button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}

	{#if $favoritos.loading}
		<div class="flex justify-center py-8">
			<div class="flex items-center gap-3 text-taupe bg-white px-4 py-2 rounded-full shadow-sm border border-silver/30">
				<div class="w-4 h-4 border-2 border-taupe/20 border-t-taupe rounded-full animate-spin"></div>
				<span class="text-sm font-medium">Cargando datos...</span>
			</div>
		</div>
	{/if}

	{#if $favoritos.loadingMore}
		<div class="flex justify-center py-4">
			<div class="flex items-center gap-2 text-greyOlive">
				<div class="w-4 h-4 border-2 border-taupe/20 border-t-taupe rounded-full animate-spin"></div>
				<span class="text-sm">Cargando más...</span>
			</div>
		</div>
	{/if}

	<!-- Paginación: sentinel para auto-scroll + botón manual -->
	{#if $favoritos.hasMore && !$favoritos.loading}
		<div bind:this={sentinelRef} class="h-1"></div>
		{#if !$favoritos.loadingMore}
			<div class="flex justify-center py-4">
				<Button
					variant="outline"
					onclick={loadMore}
					class="border-taupe/30 text-taupe hover:bg-taupe/10 font-semibold px-8"
				>
					Cargar más ({$favoritos.items.length} de {$favoritos.total})
				</Button>
			</div>
		{/if}
	{/if}
</div>

{#if selectedFavorito}
	<Dialog.Root bind:open={showEditDialog}>
		<Dialog.Content class="bg-white border-2 border-taupe/20 shadow-2xl z-[100]">
			<Dialog.Header>
				<Dialog.Title class="text-taupe">Editar Favorito</Dialog.Title>
			</Dialog.Header>
			<FavoritoForm initialData={selectedFavorito} onSave={() => showEditDialog = false} />
		</Dialog.Content>
	</Dialog.Root>

	<DeleteConfirmDialog
		bind:open={showDeleteDialog}
		onConfirm={confirmDelete}
		title="¿Eliminar favorito?"
		description={`¿Estás seguro de que deseas eliminar "${selectedFavorito.title || 'este favorito'}"?`}
	/>
{/if}
