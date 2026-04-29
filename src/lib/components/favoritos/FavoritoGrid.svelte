<script lang="ts">
	import FavoritoCard from './FavoritoCard.svelte';
	import { favoritos } from '$lib/stores/favoritos';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { LayoutGrid, List, Search, ExternalLink, Hash, Pencil, Trash2, XCircle, BookmarkX } from 'lucide-svelte';
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

	function handleTagClick(tag: any) {
		const tagName = typeof tag === 'string' ? tag : tag.name;
		favoritos.setSearchQuery(tagName);
		favoritos.executeSearch();
	}

	// IntersectionObserver for infinite scroll
	$effect(() => {
		if (sentinelRef) {
			const observer = new IntersectionObserver((entries) => {
				const entry = entries[0];
				if (entry.isIntersecting && $favoritos.hasMore && !$favoritos.loadingMore && !$favoritos.loading) {
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

<div class="space-y-4">

	<!-- ── Topbar: búsqueda + controles ──────────────────────────── -->
	<div class="flex items-center gap-3">
		<!-- Buscador pill -->
		<div class="relative flex-1">
			<Search class="absolute left-3.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-greyOlive/60 pointer-events-none" />
			<Input
				bind:ref={searchInput}
				placeholder="Buscar favoritos..."
				value={$favoritos.searchQuery}
				oninput={(e) => handleSearchInput((e.target as HTMLInputElement).value)}
				class="pl-9 pr-9 h-9 rounded-full bg-white border-silver/40 focus:border-taupe/40 text-[13px] shadow-sm"
			/>
			{#if $favoritos.searchQuery}
				<button
					type="button"
					onclick={clearSearch}
					class="absolute right-3 top-1/2 -translate-y-1/2 text-greyOlive/60 hover:text-taupe transition-colors p-1"
					title="Limpiar búsqueda"
				>
					<XCircle class="w-3.5 h-3.5" />
				</button>
			{/if}
		</div>

		<!-- Toggle de vista -->
		<div class="flex items-center gap-1 bg-silver/20 p-1 rounded-lg shrink-0">
			<Button
				variant="ghost"
				size="icon"
				class="h-7 w-7 rounded-md {$favoritos.viewMode === 'grid' ? 'bg-white text-taupe shadow-sm' : 'text-greyOlive hover:text-taupe'}"
				onclick={() => favoritos.setViewMode('grid')}
				title="Vista cuadrícula"
			>
				<LayoutGrid class="w-3.5 h-3.5" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				class="h-7 w-7 rounded-md {$favoritos.viewMode === 'list' ? 'bg-white text-taupe shadow-sm' : 'text-greyOlive hover:text-taupe'}"
				onclick={() => favoritos.setViewMode('list')}
				title="Vista lista"
			>
				<List class="w-3.5 h-3.5" />
			</Button>
		</div>
	</div>

	<!-- Indicador de resultados -->
	{#if $favoritos.total > 0}
		<p class="text-[10px] text-greyOlive/60 text-right -mt-2 tabular-nums">
			{$favoritos.items.length} de {$favoritos.total} favoritos
		</p>
	{/if}

	<!-- ── Vista grid ─────────────────────────────────────────────── -->
	{#if $favoritos.viewMode === 'grid'}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-3">
			{#each $favoritos.items as favorito (favorito.id)}
				<FavoritoCard {favorito} />
			{:else}
				{#if !$favoritos.loading}
					<div class="col-span-full py-20 text-center text-greyOlive border-2 border-dashed border-silver/30 rounded-2xl bg-white/30">
						<BookmarkX class="w-10 h-10 mx-auto mb-3 opacity-15" />
						{#if $favoritos.searchQuery}
							<p class="text-[14px] font-medium">Sin resultados para "<span class="text-taupe">{$favoritos.searchQuery}</span>"</p>
							<p class="text-[11px] mt-1 opacity-60">Prueba con otros términos</p>
						{:else}
							<p class="text-[14px] font-medium opacity-60">No hay favoritos en esta vista</p>
						{/if}
					</div>
				{/if}
			{/each}
		</div>

	<!-- ── Vista lista ────────────────────────────────────────────── -->
	{:else}
		<div class="bg-white rounded-xl border border-silver/35 overflow-hidden shadow-sm">
			<!-- Cabecera -->
			<div class="flex items-center bg-taupe/[0.03] border-b border-silver/20">
				<div class="w-12 shrink-0"></div>
				<div class="flex-1 min-w-0 px-3 py-2.5 text-[9.5px] uppercase tracking-wider text-greyOlive/60 font-bold">Título / URL</div>
				<div class="hidden md:block w-52 shrink-0 px-3 py-2.5 text-[9.5px] uppercase tracking-wider text-greyOlive/60 font-bold">Descripción</div>
				<div class="hidden lg:block w-44 shrink-0 px-3 py-2.5 text-[9.5px] uppercase tracking-wider text-greyOlive/60 font-bold">Etiquetas</div>
				<div class="w-28 shrink-0 px-3 py-2.5 text-right text-[9.5px] uppercase tracking-wider text-greyOlive/60 font-bold">Acciones</div>
			</div>
			<!-- Filas -->
			<div>
				{#each $favoritos.items as favorito (favorito.id)}
					<div
						class="relative isolate flex items-center group cursor-default select-none border-b border-silver/15 last:border-b-0 overflow-hidden transition-all duration-150"
						ondblclick={() => handleEdit(favorito)}
					>
						<!-- Fondo blur -->
						{#if favorito.faviconUrl}
							<img
								src={favorito.faviconUrl}
								alt=""
								class="absolute inset-0 w-full h-full object-cover -z-10 opacity-[0.10] blur-[20px] saturate-150 brightness-90 pointer-events-none group-hover:opacity-[0.15] transition-opacity duration-200"
								onerror={(e) => { (e.currentTarget as HTMLImageElement).style.display = 'none'; }}
							/>
						{/if}
						<!-- Favicon -->
						<div class="w-12 shrink-0 px-3 py-2.5">
							<div class="w-6 h-6 rounded-md bg-silver/10 flex items-center justify-center overflow-hidden">
								{#if favorito.faviconUrl}
									<img src={favorito.faviconUrl} alt="" class="w-4 h-4 object-contain" />
								{:else}
									<Hash class="w-3 h-3 text-greyOlive/50" />
								{/if}
							</div>
						</div>
						<!-- Título / URL -->
						<div class="flex-1 min-w-0 px-3 py-2.5">
							<div class="flex flex-col">
								<button
									onclick={() => apiClient.openUrl(favorito.url)}
									title={favorito.title || ''}
									class="text-[12.5px] font-semibold text-taupe hover:underline text-left truncate max-w-[200px] md:max-w-[300px] decoration-taupe/30"
								>
									{favorito.title || 'Sin título'}
								</button>
								<span class="text-[9.5px] text-greyOlive/60 truncate max-w-[200px]">{favorito.domain}</span>
							</div>
						</div>
						<!-- Descripción -->
						<div class="hidden md:block w-52 shrink-0 px-3 py-2.5">
							<p class="text-[11px] text-greyOlive/70 line-clamp-1 italic" title={favorito.description || ''}>{favorito.description || '—'}</p>
						</div>
						<!-- Etiquetas -->
						<div class="hidden lg:block w-44 shrink-0 px-3 py-2.5">
							{#if (favorito.tags || []).length > 0}
								<OverflowTags tags={favorito.tags || []} onTagClick={handleTagClick} />
							{/if}
						</div>
						<!-- Acciones -->
						<div class="w-28 shrink-0 px-3 py-2.5">
							<div class="flex justify-end gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
								<Button variant="ghost" size="icon" class="h-7 w-7 rounded-md text-red-400/70 hover:text-red-600 hover:bg-red-50" onclick={() => handleDeleteClick(favorito)} title="Eliminar">
									<Trash2 class="w-3.5 h-3.5" />
								</Button>
								<Button variant="ghost" size="icon" class="h-7 w-7 rounded-md text-greyOlive hover:text-taupe hover:bg-silver/20" onclick={() => handleEdit(favorito)} title="Editar">
									<Pencil class="w-3.5 h-3.5" />
								</Button>
								<Button variant="ghost" size="icon" class="h-7 w-7 rounded-md text-greyOlive hover:text-taupe hover:bg-silver/20" onclick={() => apiClient.openUrl(favorito.url)} title="Abrir">
									<ExternalLink class="w-3.5 h-3.5" />
								</Button>
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}

	<!-- ── Loading states ─────────────────────────────────────────── -->
	{#if $favoritos.loading}
		<div class="flex justify-center py-10">
			<div class="flex items-center gap-3 text-taupe bg-white px-5 py-2.5 rounded-full shadow-sm border border-silver/30">
				<div class="w-4 h-4 border-2 border-taupe/20 border-t-taupe rounded-full animate-spin"></div>
				<span class="text-[12.5px] font-medium">Cargando datos...</span>
			</div>
		</div>
	{/if}

	{#if $favoritos.loadingMore}
		<div class="flex justify-center py-4">
			<div class="flex items-center gap-2 text-greyOlive/60">
				<div class="w-3.5 h-3.5 border-2 border-taupe/20 border-t-taupe rounded-full animate-spin"></div>
				<span class="text-[11px]">Cargando más...</span>
			</div>
		</div>
	{/if}

	<!-- ── Paginación: sentinel + botón ──────────────────────────── -->
	{#if $favoritos.hasMore && !$favoritos.loading}
		<div bind:this={sentinelRef} class="h-1"></div>
		{#if !$favoritos.loadingMore}
			<div class="flex justify-center py-4">
				<Button
					variant="outline"
					onclick={loadMore}
					class="border-taupe/25 text-taupe hover:bg-taupe/5 hover:border-taupe/40 font-semibold px-8 rounded-full text-[12.5px] h-9"
				>
					Cargar más ({$favoritos.items.length} de {$favoritos.total})
				</Button>
			</div>
		{/if}
	{/if}
</div>

<!-- ── Diálogos ──────────────────────────────────────────────────── -->
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
