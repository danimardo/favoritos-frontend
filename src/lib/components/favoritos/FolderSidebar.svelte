<script lang="ts">
	import { favoritos, folderCounts, tagCounts } from '$lib/stores/favoritos';
	import { Button } from '$lib/components/ui/button';
	import { Folder, FolderOpen, LayoutGrid, RefreshCw, Settings, Info, Tag, ChevronDown, ChevronRight, Search, X } from 'lucide-svelte';
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { normalize } from '$lib/utils';

	let { onSelectFolder } = $props<{ onSelectFolder?: () => void }>();
	let refreshing = $state(false);
	let foldersExpanded = $state(true);
	let tagsExpanded = $state(true);
	let tagFilter = $state('');
	let folderFilter = $state('');

	onMount(async () => {
		try {
			const [fe, te] = await Promise.all([
				invoke<string | null>('preference_get', { key: 'sidebar_folders_expanded' }),
				invoke<string | null>('preference_get', { key: 'sidebar_tags_expanded' }),
			]);
			if (fe !== null) foldersExpanded = fe !== 'false';
			if (te !== null) tagsExpanded = te !== 'false';
		} catch (e) {
			console.warn('[Sidebar] Error al restaurar estado:', e);
		}
	});

	function selectFolder(id: string | null) {
		favoritos.selectFolder(id);
		if (onSelectFolder) onSelectFolder();
	}

	function selectTag(tag: string | null) {
		favoritos.selectTag(tag);
		if (onSelectFolder) onSelectFolder();
	}

	async function handleRefresh() {
		if (refreshing) return;
		refreshing = true;
		try {
			await favoritos.fetchAll();
		} finally {
			setTimeout(() => {
				refreshing = false;
			}, 800);
		}
	}

	// Sorted tags by count descending, filtered by tagFilter
	let sortedTags = $derived(
		Object.entries($tagCounts)
			.filter(([name]) => {
				if (!tagFilter.trim()) return true;
				return normalize(name).includes(normalize(tagFilter.trim()));
			})
			.sort((a, b) => b[1] - a[1])
	);

	// Filtered folders
	let filteredFolders = $derived(
		$favoritos.folders.filter(folder => {
			if (!folderFilter.trim()) return true;
			return normalize(folder.name).includes(normalize(folderFilter.trim()));
		})
	);
</script>

<div class="flex flex-col h-screen bg-taupe text-parchment w-64 border-r border-white/10 shadow-xl overflow-hidden">
	<!-- Header Sidebar -->
	<div class="p-6 border-b border-white/5 bg-black/5">
		<h2 class="text-xs font-bold uppercase tracking-widest text-parchment/40">Navegación</h2>
	</div>

	<!-- Contenido con scroll -->
	<div class="flex-1 overflow-y-auto py-4 px-3 space-y-1 custom-scrollbar">
		<!-- Opción Todos -->
		<button
			onclick={() => selectFolder(null)}
			class="w-full flex items-center justify-between px-3 py-2.5 rounded-lg transition-all group {$favoritos.selectedFolderId === null && $favoritos.selectedTag === null ? 'bg-black/40 text-white font-bold shadow-inner' : 'hover:bg-white/5 text-parchment/70'}"
		>
			<div class="flex items-center gap-3">
				<LayoutGrid class="w-4 h-4 {$favoritos.selectedFolderId === null && $favoritos.selectedTag === null ? 'text-powderBlush' : 'text-greyOlive group-hover:text-parchment'}" />
				<span class="text-sm">Todos</span>
			</div>
			<span class="text-[10px] bg-white/10 px-1.5 py-0.5 rounded-full font-normal">
				{$favoritos.items.length}
			</span>
		</button>

		<!-- Sección Carpetas (colapsable) -->
		<button
			onclick={() => { foldersExpanded = !foldersExpanded; invoke('preference_set', { key: 'sidebar_folders_expanded', value: String(foldersExpanded) }).catch(console.warn); }}
			class="w-full flex items-center gap-2 pt-6 pb-2 px-3 hover:opacity-80 transition-opacity"
		>
			{#if foldersExpanded}
				<ChevronDown class="w-3 h-3 text-parchment/30" />
			{:else}
				<ChevronRight class="w-3 h-3 text-parchment/30" />
			{/if}
			<p class="text-[10px] font-bold uppercase text-parchment/30 tracking-wider">Carpetas</p>
			{#if !foldersExpanded}
				<span class="text-[10px] text-parchment/20 ml-auto">{$favoritos.folders.length}</span>
			{/if}
		</button>

		{#if foldersExpanded}
			<div class="relative px-2 pb-1">
				<Search class="absolute left-4 top-1/2 -translate-y-1/2 w-3 h-3 text-parchment/30 pointer-events-none" />
				<input
					type="text"
					bind:value={folderFilter}
					placeholder="Filtrar carpetas..."
					class="w-full bg-white/5 border border-white/10 rounded-md pl-7 pr-7 py-1.5 text-xs text-parchment placeholder:text-parchment/25 focus:outline-none focus:border-white/20 focus:bg-white/10 transition-colors"
				/>
				{#if folderFilter}
					<button
						onclick={() => folderFilter = ''}
						class="absolute right-4 top-1/2 -translate-y-1/2 text-parchment/30 hover:text-parchment/60 transition-colors"
					>
						<X class="w-3 h-3" />
					</button>
				{/if}
			</div>
			{#each filteredFolders as folder}
				<button
					onclick={() => selectFolder(folder.id)}
					class="w-full flex items-center justify-between px-3 py-2.5 rounded-lg transition-all group {$favoritos.selectedFolderId === folder.id ? 'bg-black/40 text-white font-bold shadow-inner' : 'hover:bg-white/5 text-parchment/70'}"
				>
					<div class="flex items-center gap-3">
						{#if $favoritos.selectedFolderId === folder.id}
							<FolderOpen class="w-4 h-4 text-powderBlush" />
						{:else}
							<Folder class="w-4 h-4 text-greyOlive group-hover:text-parchment" />
						{/if}
						<span class="text-sm truncate max-w-[130px]">{folder.name}</span>
					</div>
					{#if $folderCounts[folder.id]}
						<span class="text-[10px] bg-white/10 px-1.5 py-0.5 rounded-full font-normal">
							{$folderCounts[folder.id]}
						</span>
					{/if}
				</button>
			{:else}
				<div class="px-3 py-4 text-center">
					<p class="text-xs text-parchment/20 italic">No hay carpetas</p>
				</div>
			{/each}
		{/if}

		<!-- Sección Etiquetas (colapsable) -->
		<button
			onclick={() => { tagsExpanded = !tagsExpanded; invoke('preference_set', { key: 'sidebar_tags_expanded', value: String(tagsExpanded) }).catch(console.warn); }}
			class="w-full flex items-center gap-2 pt-6 pb-2 px-3 hover:opacity-80 transition-opacity"
		>
			{#if tagsExpanded}
				<ChevronDown class="w-3 h-3 text-parchment/30" />
			{:else}
				<ChevronRight class="w-3 h-3 text-parchment/30" />
			{/if}
			<p class="text-[10px] font-bold uppercase text-parchment/30 tracking-wider">Etiquetas</p>
			{#if !tagsExpanded}
				<span class="text-[10px] text-parchment/20 ml-auto">{sortedTags.length}</span>
			{/if}
		</button>

		{#if tagsExpanded}
			<div class="relative px-2 pb-1">
				<Search class="absolute left-4 top-1/2 -translate-y-1/2 w-3 h-3 text-parchment/30 pointer-events-none" />
				<input
					type="text"
					bind:value={tagFilter}
					placeholder="Filtrar etiquetas..."
					class="w-full bg-white/5 border border-white/10 rounded-md pl-7 pr-7 py-1.5 text-xs text-parchment placeholder:text-parchment/25 focus:outline-none focus:border-white/20 focus:bg-white/10 transition-colors"
				/>
				{#if tagFilter}
					<button
						onclick={() => tagFilter = ''}
						class="absolute right-4 top-1/2 -translate-y-1/2 text-parchment/30 hover:text-parchment/60 transition-colors"
					>
						<X class="w-3 h-3" />
					</button>
				{/if}
			</div>
			{#each sortedTags as [tagName, count]}
				<button
					onclick={() => selectTag(tagName)}
					class="w-full flex items-center justify-between px-3 py-2 rounded-lg transition-all group {$favoritos.selectedTag === tagName ? 'bg-black/40 text-white font-bold shadow-inner' : 'hover:bg-white/5 text-parchment/70'}"
				>
					<div class="flex items-center gap-3">
						<Tag class="w-3.5 h-3.5 {$favoritos.selectedTag === tagName ? 'text-powderBlush' : 'text-greyOlive group-hover:text-parchment'}" />
						<span class="text-sm truncate max-w-[130px]">{tagName}</span>
					</div>
					<span class="text-[10px] bg-white/10 px-1.5 py-0.5 rounded-full font-normal">
						{count}
					</span>
				</button>
			{:else}
				<div class="px-3 py-4 text-center">
					<p class="text-xs text-parchment/20 italic">No hay etiquetas</p>
				</div>
			{/each}
		{/if}
	</div>

	<!-- Acciones Globales (Fijadas abajo) -->
	<div class="px-3 py-4 border-t border-white/5 bg-black/10 space-y-1">
		<button
			onclick={handleRefresh}
			disabled={refreshing}
			class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-parchment/60 hover:bg-white/5 hover:text-parchment transition-all group disabled:opacity-50"
		>
			<div class="p-1.5 bg-white/5 rounded-md group-hover:bg-white/10">
				<RefreshCw class="w-3.5 h-3.5 {refreshing ? 'animate-spin' : ''}" />
			</div>
			<span class="font-medium">{refreshing ? 'Sincronizando...' : 'Refrescar'}</span>
		</button>

		<button
			onclick={() => goto('/configuracion')}
			class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-parchment/60 hover:bg-white/5 hover:text-parchment transition-all group"
		>
			<div class="p-1.5 bg-white/5 rounded-md group-hover:bg-white/10">
				<Settings class="w-3.5 h-3.5" />
			</div>
			<span class="font-medium">Configuración</span>
		</button>
	</div>

	<!-- Footer Sidebar Informativo -->
	<div class="p-4 bg-black/30">
		<div class="flex items-center justify-between text-[10px] text-parchment/30 px-1 font-medium">
			<div class="flex items-center gap-1.5">
				<Info class="w-3 h-3" />
				<span>v0.3.2</span>
			</div>
			<span class="bg-white/5 px-2 py-0.5 rounded-full">{$favoritos.items.length} ítems</span>
		</div>
	</div>
</div>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 4px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 10px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.1);
	}
</style>
