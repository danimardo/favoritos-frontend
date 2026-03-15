<script lang="ts">
	import FavoritoCard from './FavoritoCard.svelte';
	import { favoritos, filteredByFolder } from '$lib/stores/favoritos';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { LayoutGrid, List, Search, ExternalLink, Hash, Pencil, Trash2, XCircle } from 'lucide-svelte';
	import { apiClient } from '$lib/services/api-client';
	import { normalize } from '$lib/utils';
	import * as Dialog from '$lib/components/ui/dialog';
	import FavoritoForm from './FavoritoForm.svelte';
	import DeleteConfirmDialog from './DeleteConfirmDialog.svelte';
	import type { Favorito } from '$lib/types/api';

	let searchInput = $state<HTMLInputElement | null>(null);
	
	let selectedFavorito = $state<Favorito | null>(null);
	let showEditDialog = $state(false);
	let showDeleteDialog = $state(false);

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

	// Lógica de filtrado protegida contra valores nulos/undefined
	// Usa normalize() para búsqueda insensible a acentos y mayúsculas
	let filteredItems = $derived(
		$filteredByFolder.filter(item => {
			const query = normalize($favoritos.searchQuery.trim());
			if (!query) return true;

			const title = normalize(item.title || '');
			const url = normalize(item.url || '');
			const desc = normalize(item.description || '');
			const notes = normalize(item.notes || '');
			const folderName = normalize(item.folder?.name || '');

			// Búsqueda en textos principales
			if (title.includes(query) || url.includes(query) || desc.includes(query) || notes.includes(query) || folderName.includes(query)) {
				return true;
			}

			// Búsqueda en etiquetas (pueden ser objetos o strings)
			const inTags = item.tags?.some(tag => {
				const tagName = normalize(typeof tag === 'string' ? tag : tag?.name || '');
				return tagName.includes(query);
			}) ?? false;

			return inTags;
		})
	);

	let observer: IntersectionObserver;
	let sentinel = $state<HTMLElement | null>(null);

	$effect(() => {
		if (sentinel) {
			observer = new IntersectionObserver((entries) => {
				if (entries[0].isIntersecting && $favoritos.hasMore && !$favoritos.loading) {
					favoritos.fetchNextPage();
				}
			}, { threshold: 0.1 });
			observer.observe(sentinel);
			return () => observer.disconnect();
		}
	});
</script>

<div class="space-y-6">
	<div class="flex flex-col sm:flex-row items-center gap-4 bg-white/50 p-2 rounded-xl border border-silver/20">
		<div class="relative flex-1 w-full group">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-greyOlive" />
			<Input 
				bind:ref={searchInput}
				placeholder="Buscar en esta carpeta..." 
				value={$favoritos.searchQuery}
				oninput={(e) => favoritos.setSearchQuery((e.target as HTMLInputElement).value)}
				class="pl-10 pr-10 bg-white border-silver/50 focus:border-taupe h-11"
			/>
			{#if $favoritos.searchQuery}
				<button 
					type="button"
					onclick={() => { favoritos.setSearchQuery(''); focusSearch(); }}
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

	{#if $favoritos.viewMode === 'grid'}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-4">
			{#each filteredItems as favorito (favorito.id)}
				<FavoritoCard {favorito} />
			{:else}
				{#if !$favoritos.loading}
					<div class="col-span-full py-20 text-center text-greyOlive border-2 border-dashed border-silver/30 rounded-2xl bg-silver/5">
						<Search class="w-12 h-12 mx-auto mb-4 opacity-20" />
						{#if $favoritos.searchQuery}
							<p class="text-lg">No se encontró "<span class="text-taupe font-bold">{$favoritos.searchQuery}</span>" en esta vista</p>
							<p class="text-xs mt-2 italic">Prueba a seleccionar "Todos" en la barra lateral o haz scroll hacia abajo</p>
						{:else}
							<p class="text-lg">Esta carpeta está vacía</p>
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
					{#each filteredItems as favorito (favorito.id)}
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
										class="text-sm font-bold text-taupe hover:underline text-left truncate max-w-[200px] md:max-w-[300px]"
									>
										{favorito.title || 'Sin título'}
									</button>
									<span class="text-[10px] text-greyOlive truncate max-w-[200px]">{favorito.domain}</span>
								</div>
							</td>
							<td class="px-4 py-3 hidden md:table-cell">
								<p class="text-xs text-greyOlive line-clamp-1 italic">{favorito.description || '-'}</p>
							</td>
							<td class="px-4 py-3 hidden lg:table-cell">
								<div class="flex flex-wrap gap-1">
									{#each (favorito.tags || []).slice(0, 3) as tag}
										<button 
											onclick={(e) => { e.stopPropagation(); favoritos.setSearchQuery(typeof tag === 'string' ? tag : tag.name); }}
											class="px-1.5 py-0.5 rounded bg-silver/20 text-[9px] text-taupe border border-silver/30 hover:bg-taupe hover:text-parchment transition-colors"
										>
											{typeof tag === 'string' ? tag : tag.name}
										</button>
									{/each}
									{#if (favorito.tags || []).length > 3}
										<span class="text-[9px] text-greyOlive">+{favorito.tags!.length - 3}</span>
									{/if}
								</div>
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

	<div bind:this={sentinel} class="h-10"></div>
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
