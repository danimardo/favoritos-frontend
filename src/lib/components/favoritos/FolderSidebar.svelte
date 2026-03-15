<script lang="ts">
	import { favoritos, folderCounts, tagCounts } from '$lib/stores/favoritos';
	import { apiClient } from '$lib/services/api-client';
	import { Button } from '$lib/components/ui/button';
	import { Folder, FolderOpen, LayoutGrid, RefreshCw, Settings, Info, Tag, ChevronDown, ChevronRight, Search, X, Trash2 } from 'lucide-svelte';
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

	// Context menu state
	let contextMenu = $state<{ x: number; y: number; tagName: string; tagId: string } | null>(null);
	let deletingTag = $state(false);

	function handleTagContextMenu(e: MouseEvent, tagName: string) {
		e.preventDefault();
		const tag = $favoritos.allTags.find(t => t.name === tagName);
		if (!tag) return;
		contextMenu = { x: e.clientX, y: e.clientY, tagName, tagId: tag.id };
	}

	function closeContextMenu() {
		contextMenu = null;
	}

	async function handleDeleteTag() {
		if (!contextMenu || deletingTag) return;
		const { tagId, tagName } = contextMenu;
		if (!confirm(`¿Eliminar la etiqueta "${tagName}"? Se desvinculará de todos los favoritos.`)) {
			closeContextMenu();
			return;
		}
		deletingTag = true;
		try {
			await apiClient.deleteTag(tagId);
			await favoritos.loadFoldersAndTags();
			if ($favoritos.selectedTag === tagName) {
				favoritos.selectTag(null);
			}
		} catch (e) {
			console.error('[Sidebar] Error al eliminar etiqueta:', e);
			alert(`Error al eliminar la etiqueta: ${e}`);
		} finally {
			deletingTag = false;
			closeContextMenu();
		}
	}

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
			await favoritos.loadFoldersAndTags();
			await favoritos.fetchPage(true);
		} finally {
			setTimeout(() => {
				refreshing = false;
			}, 800);
		}
	}

	const SIDEBAR_LIMIT = 30;

	// Tags: sorted by bookmarkCount desc, filtered by search, limited when not searching
	let sortedTags = $derived.by(() => {
		const all = Object.entries($tagCounts).sort((a, b) => b[1] - a[1]);
		const query = tagFilter.trim();
		if (!query) return { items: all.slice(0, SIDEBAR_LIMIT), total: all.length, filtered: false };
		const filtered = all.filter(([name]) => normalize(name).includes(normalize(query)));
		return { items: filtered, total: all.length, filtered: true };
	});

	// Folders: sorted by bookmarkCount desc, filtered by search, limited when not searching
	let sortedFolders = $derived.by(() => {
		const all = [...$favoritos.folders].sort((a, b) => (b.bookmarkCount ?? 0) - (a.bookmarkCount ?? 0));
		const query = folderFilter.trim();
		if (!query) return { items: all.slice(0, SIDEBAR_LIMIT), total: all.length, filtered: false };
		const filtered = all.filter(folder => normalize(folder.name).includes(normalize(query)));
		return { items: filtered, total: all.length, filtered: true };
	});
</script>

<div class="flex flex-col h-screen bg-taupe text-parchment w-64 border-r border-white/10 shadow-xl overflow-hidden">
	<!-- Header Sidebar -->
	<div class="p-6 border-b border-white/5 bg-black/5">
		<h2 class="text-xs font-bold uppercase tracking-widest text-parchment/40">Navegación</h2>
		{#if $favoritos.loading}
			<div class="flex items-center gap-2 mt-2">
				<div class="w-3 h-3 border-2 border-parchment/20 border-t-parchment/60 rounded-full animate-spin"></div>
				<span class="text-[10px] text-parchment/40">
					{#if $favoritos.items.length > 0}
						Sincronizando... {$favoritos.items.length}
					{:else}
						Cargando...
					{/if}
				</span>
			</div>
		{/if}
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
				{$favoritos.total || $favoritos.items.length}
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
					placeholder="Buscar en {sortedFolders.total} carpetas..."
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
			{#each sortedFolders.items as folder}
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
					<p class="text-xs text-parchment/20 italic">{folderFilter ? 'Sin resultados' : 'No hay carpetas'}</p>
				</div>
			{/each}
			{#if !sortedFolders.filtered && sortedFolders.total > SIDEBAR_LIMIT}
				<p class="text-[10px] text-parchment/20 text-center py-1 italic">
					Mostrando {SIDEBAR_LIMIT} de {sortedFolders.total} — usa el buscador para ver más
				</p>
			{/if}
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
				<span class="text-[10px] text-parchment/20 ml-auto">{$favoritos.allTags.length}</span>
			{/if}
		</button>

		{#if tagsExpanded}
			<div class="relative px-2 pb-1">
				<Search class="absolute left-4 top-1/2 -translate-y-1/2 w-3 h-3 text-parchment/30 pointer-events-none" />
				<input
					type="text"
					bind:value={tagFilter}
					placeholder="Buscar en {sortedTags.total} etiquetas..."
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
			{#each sortedTags.items as [tagName, count]}
				<button
					onclick={() => selectTag(tagName)}
					oncontextmenu={(e) => handleTagContextMenu(e, tagName)}
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
					<p class="text-xs text-parchment/20 italic">{tagFilter ? 'Sin resultados' : 'No hay etiquetas'}</p>
				</div>
			{/each}
			{#if !sortedTags.filtered && sortedTags.total > SIDEBAR_LIMIT}
				<p class="text-[10px] text-parchment/20 text-center py-1 italic">
					Mostrando {SIDEBAR_LIMIT} de {sortedTags.total} — usa el buscador para ver más
				</p>
			{/if}
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
				<span>v0.3.4</span>
			</div>
			<span class="bg-white/5 px-2 py-0.5 rounded-full">{$favoritos.total || $favoritos.items.length} ítems</span>
		</div>
	</div>
</div>

<!-- Context menu para etiquetas -->
{#if contextMenu}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div
		class="fixed inset-0 z-50"
		onclick={closeContextMenu}
		oncontextmenu={(e) => { e.preventDefault(); closeContextMenu(); }}
	>
		<div
			class="absolute bg-taupe border border-white/20 rounded-lg shadow-2xl py-1 min-w-[160px]"
			style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
		>
			<button
				onclick={handleDeleteTag}
				disabled={deletingTag}
				class="w-full flex items-center gap-2 px-3 py-2 text-sm text-red-300 hover:bg-red-500/20 transition-colors disabled:opacity-50"
			>
				<Trash2 class="w-3.5 h-3.5" />
				<span>{deletingTag ? 'Eliminando...' : 'Eliminar etiqueta'}</span>
			</button>
		</div>
	</div>
{/if}

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
