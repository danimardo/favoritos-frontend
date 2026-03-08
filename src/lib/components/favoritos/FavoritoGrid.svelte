<script lang="ts">
	import FavoritoCard from './FavoritoCard.svelte';
	import { favoritos, filteredByFolder } from '$lib/stores/favoritos';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { LayoutGrid, List, Search, ExternalLink, Hash } from 'lucide-svelte';

	let searchQuery = $state('');
	let searchInput = $state<HTMLInputElement | null>(null);

	export function focusSearch() {
		if (searchInput) searchInput.focus();
	}

	// Filtramos primero por carpeta (del store derivado) y luego por búsqueda local
	let filteredItems = $derived(
		$filteredByFolder.filter(item => {
			const query = searchQuery.toLowerCase();
			if (!query) return true;
			
			// Búsqueda en campos básicos
			const inBasic = 
				item.title.toLowerCase().includes(query) ||
				item.url.toLowerCase().includes(query) ||
				(item.description?.toLowerCase().includes(query) ?? false) ||
				(item.notes?.toLowerCase().includes(query) ?? false);
			
			if (inBasic) return true;

			// Búsqueda en etiquetas
			const inTags = item.tags?.some(tag => {
				const tagName = typeof tag === 'string' ? tag : tag.name;
				return tagName.toLowerCase().includes(query);
			}) ?? false;

			if (inTags) return true;

			return false;
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
	<!-- Toolbar Superior -->
	<div class="flex flex-col sm:flex-row items-center gap-4 bg-white/50 p-2 rounded-xl border border-silver/20">
		<div class="relative flex-1 w-full">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-greyOlive" />
			<Input 
				bind:ref={searchInput}
				placeholder="Buscar en esta carpeta..." 
				bind:value={searchQuery} 
				class="pl-10 bg-white border-silver/50 focus:border-taupe h-11"
			/>
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

	<!-- Lista de Favoritos -->
	{#if $favoritos.viewMode === 'grid'}
		<!-- VISTA GRID -->
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-4 gap-4">
			{#each filteredItems as favorito (favorito.id)}
				<FavoritoCard {favorito} />
			{:else}
				{#if !$favoritos.loading}
					<div class="col-span-full py-20 text-center text-greyOlive border-2 border-dashed border-silver/30 rounded-2xl bg-silver/5">
						<Search class="w-12 h-12 mx-auto mb-4 opacity-20" />
						{#if searchQuery}
							<p class="text-lg">No se encontraron resultados para "<span class="text-taupe font-bold">{searchQuery}</span>"</p>
						{:else}
							<p class="text-lg">Esta carpeta está vacía</p>
						{/if}
					</div>
				{/if}
			{/each}
		</div>
	{:else}
		<!-- VISTA LISTA COMPACTA -->
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
						<tr class="hover:bg-silver/5 transition-colors group">
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
									<a href={favorito.url} target="_blank" class="text-sm font-bold text-taupe hover:underline truncate max-w-[200px] md:max-w-[300px]">
										{favorito.title}
									</a>
									<span class="text-[10px] text-greyOlive truncate max-w-[200px]">{favorito.domain}</span>
								</div>
							</td>
							<td class="px-4 py-3 hidden md:table-cell">
								<p class="text-xs text-greyOlive line-clamp-1 italic">{favorito.description || '-'}</p>
							</td>
							<td class="px-4 py-3 hidden lg:table-cell">
								<div class="flex flex-wrap gap-1">
									{#each (favorito.tags || []).slice(0, 3) as tag}
										<span class="px-1.5 py-0.5 rounded bg-silver/20 text-[9px] text-taupe border border-silver/30">
											{typeof tag === 'string' ? tag : tag.name}
										</span>
									{/each}
									{#if (favorito.tags || []).length > 3}
										<span class="text-[9px] text-greyOlive">+{favorito.tags!.length - 3}</span>
									{/if}
								</div>
							</td>
							<td class="px-4 py-3 text-right">
								<div class="flex justify-end gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
									<Button variant="ghost" size="sm" class="h-8 px-2" onclick={() => window.open(favorito.url)}>
										<ExternalLink class="w-3 h-3" />
									</Button>
									<!-- Aquí podríamos añadir los botones de editar/borrar si quisiéramos -->
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
			{#if filteredItems.length === 0 && !$favoritos.loading}
				<div class="py-20 text-center text-greyOlive bg-silver/5">
					<p>No hay favoritos en esta vista</p>
				</div>
			{/if}
		</div>
	{/if}

	{#if $favoritos.loading}
		<div class="flex justify-center py-8">
			<div class="flex items-center gap-3 text-taupe bg-white px-4 py-2 rounded-full shadow-sm border border-silver/30">
				<div class="w-4 h-4 border-2 border-taupe/20 border-t-taupe rounded-full animate-spin"></div>
				<span class="text-sm font-medium">Cargando más...</span>
			</div>
		</div>
	{/if}

	<div bind:this={sentinel} class="h-10"></div>
</div>
