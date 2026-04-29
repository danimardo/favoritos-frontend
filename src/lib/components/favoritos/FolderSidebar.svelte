<script lang="ts">
	import { favoritos, folderCounts, tagCounts } from '$lib/stores/favoritos';
	import { apiClient } from '$lib/services/api-client';
	import { Button } from '$lib/components/ui/button';
	import { Folder, FolderOpen, LayoutGrid, RefreshCw, Settings, Tag, ChevronDown, ChevronRight, Search, X, Trash2, Bookmark } from 'lucide-svelte';
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
			setTimeout(() => { refreshing = false; }, 800);
		}
	}

	const SIDEBAR_LIMIT = 30;

	let sortedTags = $derived.by(() => {
		const all = Object.entries($tagCounts).sort((a, b) => b[1] - a[1]);
		const query = tagFilter.trim();
		if (!query) return { items: all.slice(0, SIDEBAR_LIMIT), total: all.length, filtered: false };
		const filtered = all.filter(([name]) => normalize(name).includes(normalize(query)));
		return { items: filtered, total: all.length, filtered: true };
	});

	let sortedFolders = $derived.by(() => {
		const all = [...$favoritos.folders].sort((a, b) => (b.bookmarkCount ?? 0) - (a.bookmarkCount ?? 0));
		const query = folderFilter.trim();
		if (!query) return { items: all.slice(0, SIDEBAR_LIMIT), total: all.length, filtered: false };
		const filtered = all.filter(folder => normalize(folder.name).includes(normalize(query)));
		return { items: filtered, total: all.length, filtered: true };
	});
</script>

<div class="flex flex-col h-screen w-[220px] border-r border-white/5 shadow-xl overflow-hidden" style="background: linear-gradient(180deg, #3b2f25 0%, #483C32 55%);">

	<!-- ── Branding ─────────────────────────────────────────── -->
	<div class="px-4 py-4 border-b border-white/5 flex items-center gap-3 bg-black/10">
		<div class="w-7 h-7 rounded-lg bg-white/8 flex items-center justify-center shrink-0">
			<Bookmark class="w-3.5 h-3.5 text-white/80" />
		</div>
		<div>
			<p class="text-[13px] font-bold text-white tracking-tight leading-none">Favoritos</p>
			<p class="text-[9px] text-white/60 uppercase tracking-widest mt-0.5">Desktop</p>
		</div>
		{#if $favoritos.loading}
			<div class="ml-auto w-3 h-3 border-2 border-parchment/15 border-t-parchment/50 rounded-full animate-spin shrink-0"></div>
		{/if}
	</div>

	<!-- ── Nav scroll area ────────────────────────────────────── -->
	<div class="flex-1 overflow-y-auto py-3 px-2.5 space-y-0.5 custom-scrollbar">

		<!-- Todos -->
		<button
			onclick={() => selectFolder(null)}
			class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg transition-all group {$favoritos.selectedFolderId === null && $favoritos.selectedTag === null ? 'bg-black/35 text-white font-semibold shadow-inner' : 'hover:bg-white/5 text-white/70'}"
		>
			<div class="flex items-center gap-2.5">
				<LayoutGrid class="w-3.5 h-3.5 {$favoritos.selectedFolderId === null && $favoritos.selectedTag === null ? 'text-powderBlush' : 'text-white/60 group-hover:text-white/80'} shrink-0" />
				<span class="text-[12.5px] text-white/70">Todos</span>
			</div>
			<span class="text-[9px] bg-white/8 px-1.5 py-0.5 rounded-full font-normal tabular-nums">
				{$favoritos.globalTotal || $favoritos.total || $favoritos.items.length}
			</span>
		</button>

		<!-- ── Carpetas ── -->
		<button
			onclick={() => {
				foldersExpanded = !foldersExpanded;
				invoke('preference_set', { key: 'sidebar_folders_expanded', value: String(foldersExpanded) }).catch(console.warn);
			}}
			class="w-full flex items-center gap-2 pt-5 pb-1.5 px-2.5 hover:opacity-80 transition-opacity"
		>
			{#if foldersExpanded}
				<ChevronDown class="w-2.5 h-2.5 text-white/50" />
			{:else}
				<ChevronRight class="w-2.5 h-2.5 text-white/50" />
			{/if}
			<p class="text-[9px] font-bold uppercase text-white/60 tracking-wider">Carpetas</p>
			{#if !foldersExpanded}
				<span class="text-[9px] text-white/50 ml-auto">{$favoritos.folders.length}</span>
			{/if}
		</button>

		{#if foldersExpanded}
			<!-- Search carpetas -->
			<div class="relative px-1 pb-1">
				<Search class="absolute left-3.5 top-1/2 -translate-y-1/2 w-2.5 h-2.5 text-white/40 pointer-events-none" />
				<input
					type="text"
					bind:value={folderFilter}
					placeholder="Buscar en {sortedFolders.total} carpetas..."
					class="w-full bg-white/4 border border-white/8 rounded-md pl-6 pr-6 py-1.5 text-[10.5px] text-white placeholder:text-white/40 focus:outline-none focus:border-white/18 focus:bg-white/8 transition-colors"
				/>
				{#if folderFilter}
					<button onclick={() => folderFilter = ''} class="absolute right-3.5 top-1/2 -translate-y-1/2 text-white/40 hover:text-white/70 transition-colors">
						<X class="w-2.5 h-2.5" />
					</button>
				{/if}
			</div>

			{#each sortedFolders.items as folder}
				<button
					onclick={() => selectFolder(folder.id)}
					class="w-full flex items-center justify-between px-2.5 py-2 rounded-lg transition-all group {String($favoritos.selectedFolderId) === String(folder.id) ? 'bg-black/35 text-white font-semibold shadow-inner' : 'hover:bg-white/5 text-white/70'}"
				>
					<div class="flex items-center gap-2.5 min-w-0">
						{#if String($favoritos.selectedFolderId) === String(folder.id)}
							<FolderOpen class="w-3.5 h-3.5 text-powderBlush shrink-0" />
						{:else}
							<Folder class="w-3.5 h-3.5 text-white/60 group-hover:text-white/80 shrink-0" />
						{/if}
						<span class="text-[11.5px] truncate">{folder.name}</span>
					</div>
					{#if $folderCounts[folder.id]}
						<span class="text-[9px] bg-white/8 px-1.5 py-0.5 rounded-full font-normal tabular-nums shrink-0">
							{$folderCounts[folder.id]}
						</span>
					{/if}
				</button>
			{:else}
				<div class="px-3 py-3 text-center">
					<p class="text-[10px] text-white/40 italic">{folderFilter ? 'Sin resultados' : 'No hay carpetas'}</p>
				</div>
			{/each}

			{#if !sortedFolders.filtered && sortedFolders.total > SIDEBAR_LIMIT}
				<p class="text-[9px] text-parchment/18 text-center py-1 italic">
					{SIDEBAR_LIMIT} de {sortedFolders.total} — usa el buscador
				</p>
			{/if}
		{/if}

		<!-- ── Etiquetas ── -->
		<button
			onclick={() => {
				tagsExpanded = !tagsExpanded;
				invoke('preference_set', { key: 'sidebar_tags_expanded', value: String(tagsExpanded) }).catch(console.warn);
			}}
			class="w-full flex items-center gap-2 pt-5 pb-1.5 px-2.5 hover:opacity-80 transition-opacity"
		>
			{#if tagsExpanded}
				<ChevronDown class="w-2.5 h-2.5 text-white/50" />
			{:else}
				<ChevronRight class="w-2.5 h-2.5 text-white/50" />
			{/if}
			<p class="text-[9px] font-bold uppercase text-white/60 tracking-wider">Etiquetas</p>
			{#if !tagsExpanded}
				<span class="text-[9px] text-white/50 ml-auto">{$favoritos.allTags.length}</span>
			{/if}
		</button>

		{#if tagsExpanded}
			<!-- Search etiquetas -->
			<div class="relative px-1 pb-1">
				<Search class="absolute left-3.5 top-1/2 -translate-y-1/2 w-2.5 h-2.5 text-white/40 pointer-events-none" />
				<input
					type="text"
					bind:value={tagFilter}
					placeholder="Buscar en {sortedTags.total} etiquetas..."
					class="w-full bg-white/4 border border-white/8 rounded-md pl-6 pr-6 py-1.5 text-[10.5px] text-white placeholder:text-white/40 focus:outline-none focus:border-white/18 focus:bg-white/8 transition-colors"
				/>
				{#if tagFilter}
					<button onclick={() => tagFilter = ''} class="absolute right-3.5 top-1/2 -translate-y-1/2 text-white/40 hover:text-white/70 transition-colors">
						<X class="w-2.5 h-2.5" />
					</button>
				{/if}
			</div>

			{#each sortedTags.items as [tagName, count]}
				<button
					onclick={() => selectTag(tagName)}
					oncontextmenu={(e) => handleTagContextMenu(e, tagName)}
					class="w-full flex items-center justify-between px-2.5 py-1.5 rounded-lg transition-all group {$favoritos.selectedTag === tagName ? 'bg-black/35 text-white font-semibold shadow-inner' : 'hover:bg-white/5 text-white/70'}"
				>
					<div class="flex items-center gap-2.5 min-w-0">
						<Tag class="w-3 h-3 {$favoritos.selectedTag === tagName ? 'text-powderBlush' : 'text-white/60 group-hover:text-white/80'} shrink-0" />
						<span class="text-[11.5px] truncate">{tagName}</span>
					</div>
					<span class="text-[9px] bg-white/8 px-1.5 py-0.5 rounded-full font-normal tabular-nums shrink-0">
						{count}
					</span>
				</button>
			{:else}
				<div class="px-3 py-3 text-center">
					<p class="text-[10px] text-white/40 italic">{tagFilter ? 'Sin resultados' : 'No hay etiquetas'}</p>
				</div>
			{/each}

			{#if !sortedTags.filtered && sortedTags.total > SIDEBAR_LIMIT}
				<p class="text-[9px] text-parchment/18 text-center py-1 italic">
					{SIDEBAR_LIMIT} de {sortedTags.total} — usa el buscador
				</p>
			{/if}
		{/if}
	</div>

	<!-- ── Acciones fijas abajo ────────────────────────────────── -->
	<div class="px-2.5 py-3 border-t border-white/5 space-y-0.5">
		<button
			onclick={handleRefresh}
			disabled={refreshing}
			class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-[11.5px] text-white/70 hover:bg-white/5 hover:text-white/90 transition-all group disabled:opacity-40"
		>
			<div class="w-5 h-5 bg-white/5 rounded-md flex items-center justify-center group-hover:bg-white/10">
				<RefreshCw class="w-3 h-3 text-white/70 {refreshing ? 'animate-spin' : ''}" />
			</div>
			<span class="font-medium">{refreshing ? 'Sincronizando...' : 'Refrescar'}</span>
		</button>

		<button
			onclick={() => goto('/configuracion')}
			class="w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-[11.5px] text-white/70 hover:bg-white/5 hover:text-white/90 transition-all group"
		>
			<div class="w-5 h-5 bg-white/5 rounded-md flex items-center justify-center group-hover:bg-white/10">
				<Settings class="w-3 h-3 text-white/70" />
			</div>
			<span class="font-medium">Configuración</span>
		</button>
	</div>

	<!-- ── Footer ─────────────────────────────────────────────── -->
	<div class="px-4 py-2.5 bg-black/25 flex items-center justify-between">
		<span class="text-[9px] text-white/50 font-medium">v0.3.7</span>
		<span class="text-[9px] text-white/60 bg-white/5 px-2 py-0.5 rounded-full tabular-nums">
			{$favoritos.globalTotal || $favoritos.total || $favoritos.items.length} ítems
		</span>
	</div>
</div>

<!-- Context menu etiquetas -->
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
		width: 3px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.06);
		border-radius: 10px;
	}
	.custom-scrollbar::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.12);
	}
</style>
