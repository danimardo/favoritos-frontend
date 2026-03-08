<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { Favorito } from '$lib/types/api';
	import { favoritos } from '$lib/stores/favoritos';
	import { apiClient } from '$lib/services/api-client';
	import DeleteConfirmDialog from './DeleteConfirmDialog.svelte';
	import FavoritoForm from './FavoritoForm.svelte';
	import { favicons } from '$lib/services/favicons';
	import { onMount } from 'svelte';

	let { favorito }: { favorito: Favorito } = $props();

	let showDeleteDialog = $state(false);
	let showEditDialog = $state(false);
	let faviconUrl = $state<string | null>(null);

	onMount(async () => {
		if (favorito.domain) {
			faviconUrl = await favicons.getOrFetch(favorito.domain);
		}
	});

	async function handleVisit(e: MouseEvent) {
		e.preventDefault();
		try {
			const updated = await apiClient.visitFavorite(favorito.id);
			// Update store with updated favorite (visitCount increased)
			favoritos.update(favorito.id, { visitCount: updated.visitCount });
		} catch (e) {
			console.error('Error visiting favorite:', e);
			// Fallback: just open URL
			await apiClient.openUrl(favorito.url);
		}
	}

	async function handleDelete() {
		await favoritos.delete(favorito.id);
	}

	function handleCopy() {
		navigator.clipboard.writeText(favorito.url);
	}
</script>

<Card.Root class="group hover:shadow-md transition-shadow bg-white border-silver/50 overflow-hidden">
	<Card.Header class="p-4 pb-2">
		<div class="flex justify-between items-start gap-2">
			{#if faviconUrl || favorito.faviconUrl}
				<img 
					src={faviconUrl || favorito.faviconUrl} 
					alt="" 
					class="w-8 h-8 rounded shrink-0 bg-white p-1 border border-silver/30"
					onerror={(e) => ((e.currentTarget as HTMLElement).style.display = 'none')}
				/>
			{/if}
			<div class="flex-1 min-w-0">
				<Card.Title class="text-sm font-bold text-taupe truncate" title={favorito.title}>
					<button onclick={handleVisit} class="hover:underline text-left w-full">
						{favorito.title}
					</button>
				</Card.Title>
				<Card.Description class="text-xs text-greyOlive truncate">
					{favorito.domain}
				</Card.Description>
			</div>
			<div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
				<Button variant="ghost" size="icon" class="h-7 w-7 text-greyOlive hover:text-taupe" onclick={handleCopy} title="Copiar URL">
					📋
				</Button>
				<Button variant="ghost" size="icon" class="h-7 w-7 text-greyOlive hover:text-taupe" onclick={() => showEditDialog = true} title="Editar">
					✏️
				</Button>
				<Button variant="ghost" size="icon" class="h-7 w-7 text-red-400 hover:text-red-600" onclick={() => showDeleteDialog = true} title="Eliminar">
					🗑️
				</Button>
			</div>
		</div>
	</Card.Header>
	<Card.Content class="p-4 pt-0">
		<button onclick={handleVisit} class="text-xs text-silver hover:text-taupe break-all line-clamp-1 text-left w-full">
			{favorito.url}
		</button>
		{#if favorito.tags && favorito.tags.length > 0}
			<div class="flex flex-wrap gap-1 mt-2">
				{#each favorito.tags as tag}
					<span class="px-1.5 py-0.5 rounded bg-silver/20 text-[10px] text-taupe border border-silver/30">
						{tag}
					</span>
				{/each}
			</div>
		{/if}
	</Card.Content>
</Card.Root>

<DeleteConfirmDialog 
	bind:open={showDeleteDialog} 
	onConfirm={handleDelete} 
	title="¿Eliminar favorito?"
	description={`¿Estás seguro de que deseas eliminar "${favorito.title}"?`}
/>

<Dialog.Root bind:open={showEditDialog}>
	<Dialog.Content class="bg-white border-2 border-taupe/20 shadow-2xl z-[100]">
		<Dialog.Header>
			<Dialog.Title class="text-taupe">Editar Favorito</Dialog.Title>
		</Dialog.Header>
		<FavoritoForm initialData={favorito} onSave={() => showEditDialog = false} />
	</Dialog.Content>
</Dialog.Root>
