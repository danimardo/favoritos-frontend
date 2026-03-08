<script lang="ts">
	import FavoritoCard from './FavoritoCard.svelte';
	import { favoritos } from '$lib/stores/favoritos';
	import { Input } from '$lib/components/ui/input';

	let searchQuery = $state('');
	let searchInput = $state<HTMLInputElement | null>(null);

	export function focusSearch() {
		if (searchInput) searchInput.focus();
	}

	let filteredItems = $derived(
		$favoritos.items.filter(item => {
			const query = searchQuery.toLowerCase();
			
			// Búsqueda en campos básicos
			const inBasic = 
				item.title.toLowerCase().includes(query) ||
				item.url.toLowerCase().includes(query) ||
				(item.description?.toLowerCase().includes(query) ?? false) ||
				(item.notes?.toLowerCase().includes(query) ?? false);
			
			if (inBasic) return true;

			// Búsqueda en etiquetas (objetos {name})
			const inTags = item.tags?.some(tag => {
				const tagName = typeof tag === 'string' ? tag : tag.name;
				return tagName.toLowerCase().includes(query);
			}) ?? false;

			if (inTags) return true;

			// Búsqueda en carpeta
			const inFolder = item.folder?.name.toLowerCase().includes(query) ?? false;
			
			return inFolder;
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
	<div class="flex items-center gap-4">
		<div class="relative flex-1">
			<span class="absolute left-3 top-1/2 -translate-y-1/2 text-silver">🔍</span>
			<Input 
				bind:ref={searchInput}
				placeholder="Buscar favoritos..." 
				bind:value={searchQuery} 
				class="pl-10 bg-parchment border-silver focus:border-taupe"
			/>
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
		{#each filteredItems as favorito (favorito.id)}
			<FavoritoCard {favorito} />
		{:else}
			{#if !$favoritos.loading}
				<div class="col-span-full py-12 text-center text-greyOlive border-2 border-dashed border-silver rounded-lg">
					{#if searchQuery}
						No se encontraron favoritos que coincidan con "{searchQuery}"
					{:else}
						No tienes favoritos guardados todavía.
					{/if}
				</div>
			{/if}
		{/each}
	</div>

	{#if $favoritos.loading}
		<div class="flex justify-center py-8">
			<div class="animate-spin text-taupe text-2xl">⏳</div>
		</div>
	{/if}

	<div bind:this={sentinel} class="h-1"></div>
</div>
