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
	import { Globe, Copy, Pencil, Trash2, ExternalLink } from 'lucide-svelte';
	import { onMount } from 'svelte';

	let { favorito }: { favorito: Favorito } = $props();

	let showDeleteDialog = $state(false);
	let showEditDialog = $state(false);
	let faviconUrl = $state<string | null>(favorito.faviconUrl || null);
	let copied = $state(false);

	onMount(async () => {
		if (!faviconUrl && favorito.domain) {
			const cached = await favicons.getOrFetch(favorito.domain);
			if (cached) faviconUrl = cached;
		}
	});

	async function handleVisit(e: MouseEvent) {
		e.preventDefault();
		try {
			const updated = await apiClient.visitFavorite(favorito.id);
			favoritos.update(favorito.id, { visitCount: updated.visitCount });
		} catch (e) {
			console.error('Error visiting favorite:', e);
			await apiClient.openUrl(favorito.url);
		}
	}

	async function handleDelete() {
		await favoritos.delete(favorito.id);
	}

	function handleCopy() {
		navigator.clipboard.writeText(favorito.url);
		copied = true;
		setTimeout(() => { copied = false; }, 1500);
	}
</script>

<Card.Root
	class="group isolate hover:shadow-lg hover:-translate-y-0.5 transition-all duration-150 bg-white border border-silver/40 hover:border-taupe/20 overflow-hidden cursor-default select-none rounded-xl relative"
	ondblclick={() => showEditDialog = true}
>
	<!-- Background blur image -->
	{#if faviconUrl}
		<img
			src={faviconUrl}
			alt=""
			class="absolute inset-0 w-full h-full object-cover -z-10 opacity-[0.20] blur-[20px] saturate-150 brightness-90 scale-110 pointer-events-none group-hover:opacity-[0.30] transition-opacity duration-200"
			onerror={() => { faviconUrl = null; }}
		/>
		<div class="absolute inset-0 -z-10 bg-gradient-to-t from-white/70 via-white/30 to-transparent pointer-events-none"></div>
	{/if}

	<Card.Header class="p-4 pb-3 relative z-10">
		<div class="flex justify-between items-start gap-3">
			<!-- Favicon -->
			{#if faviconUrl}
				<img
					src={faviconUrl}
					alt=""
					class="w-6 h-6 rounded shrink-0 mt-0.5 object-contain"
					onerror={() => { faviconUrl = null; }}
				/>
			{:else}
				<div class="w-6 h-6 rounded shrink-0 mt-0.5 bg-silver/15 border border-silver/25 flex items-center justify-center">
					<Globe class="w-3.5 h-3.5 text-greyOlive/60" />
				</div>
			{/if}

			<!-- Title + domain -->
			<div class="flex-1 min-w-0">
				<Card.Title class="text-[13px] font-semibold text-taupe leading-snug" title={favorito.title}>
					<button onclick={handleVisit} class="hover:underline text-left w-full line-clamp-2 decoration-taupe/30">
						{favorito.title}
					</button>
				</Card.Title>
				<Card.Description class="text-[10px] text-greyOlive truncate mt-0.5">
					{favorito.domain}
				</Card.Description>
			</div>

			<!-- Action buttons — visible on hover -->
			<div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity duration-150 shrink-0 -mr-1">
				<button
					onclick={handleCopy}
					title={copied ? 'Copiado' : 'Copiar URL'}
					class="w-6 h-6 rounded-md flex items-center justify-center text-greyOlive hover:text-taupe hover:bg-silver/20 transition-colors"
				>
					<Copy class="w-3 h-3" />
				</button>
				<button
					onclick={() => showEditDialog = true}
					title="Editar"
					class="w-6 h-6 rounded-md flex items-center justify-center text-greyOlive hover:text-taupe hover:bg-silver/20 transition-colors"
				>
					<Pencil class="w-3 h-3" />
				</button>
				<button
					onclick={() => showDeleteDialog = true}
					title="Eliminar"
					class="w-6 h-6 rounded-md flex items-center justify-center text-red-400/70 hover:text-red-600 hover:bg-red-50 transition-colors"
				>
					<Trash2 class="w-3 h-3" />
				</button>
			</div>
		</div>
	</Card.Header>

	<Card.Content class="px-4 pt-0 pb-4 space-y-3 relative z-10">
		<!-- Description -->
		{#if favorito.description}
			<p class="text-[11px] text-greyOlive leading-relaxed line-clamp-2">
				{favorito.description}
			</p>
		{/if}

		<!-- Tags -->
		{#if favorito.tags && favorito.tags.length > 0}
			<div class="flex flex-wrap gap-1.5">
				{#each favorito.tags.slice(0, 4) as tag}
					<button
						onclick={(e) => { e.stopPropagation(); favoritos.setSearchQuery(typeof tag === 'string' ? tag : tag.name); }}
						class="px-2 py-0.5 rounded-full bg-powderBlush/40 text-[10px] text-taupe border border-powderBlush/60 hover:bg-powderBlush hover:border-powderBlush transition-colors font-medium"
					>
						{typeof tag === 'string' ? tag : tag.name}
					</button>
				{/each}
				{#if favorito.tags.length > 4}
					<span class="text-[10px] text-greyOlive self-center">+{favorito.tags.length - 4}</span>
				{/if}
			</div>
		{/if}

		<!-- Footer -->
		<div class="flex items-center justify-between pt-2 border-t border-silver/15">
			<span class="text-[9px] text-greyOlive/50 font-medium tabular-nums">
				{favorito.visitCount ?? 0} visitas
			</span>
			<button
				onclick={handleVisit}
				class="flex items-center gap-1 text-[10px] text-greyOlive hover:text-taupe transition-colors px-1.5 py-0.5 rounded-md hover:bg-silver/15"
			>
				<ExternalLink class="w-2.5 h-2.5" />
				Abrir
			</button>
		</div>
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
