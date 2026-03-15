import { writable, get, derived } from 'svelte/store';
import type { Favorito, FavoritoInput, FavoritoUpdate, Carpeta, Etiqueta } from '$lib/types/api';
import { databaseService } from '$lib/services/database';
import { apiClient } from '$lib/services/api-client';
import { tauriNotifications } from '$lib/services/tauri';
import { invoke } from '@tauri-apps/api/core';

interface FavoritosState {
	items: Favorito[];
	folders: Carpeta[];
	allTags: Etiqueta[];
	loading: boolean;
	loadingMore: boolean;
	error: string | null;
	page: number;
	total: number;
	totalPages: number;
	hasMore: boolean;
	selectedFolderId: string | null;
	selectedTag: string | null;
	viewMode: 'grid' | 'list';
	searchQuery: string;
}

const initialState: FavoritosState = {
	items: [],
	folders: [],
	allTags: [],
	loading: false,
	loadingMore: false,
	error: null,
	page: 1,
	total: 0,
	totalPages: 0,
	hasMore: false,
	selectedFolderId: null,
	selectedTag: null,
	viewMode: 'grid',
	searchQuery: ''
};

const PAGE_SIZE = 50;

function createFavoritosStore() {
	const { subscribe, set, update } = writable<FavoritosState>(initialState);
	let fetchInProgress = false;

	function getServerParams(state: FavoritosState, pageToLoad: number) {
		const params: Record<string, any> = {
			page: pageToLoad,
			limit: PAGE_SIZE,
		};
		if (state.searchQuery.trim()) params.search = state.searchQuery.trim();
		if (state.selectedFolderId) params.folder = state.selectedFolderId;
		if (state.selectedTag) {
			// Resolve tag name → tag ID for the API (expects tagId param)
			const tag = state.allTags.find(t => t.name === state.selectedTag);
			if (tag) {
				params.tags = String(tag.id);
			}
		}
		return params;
	}

	return {
		subscribe,

		async fetchPage(reset: boolean = true) {
			const state = get({ subscribe });

			if (reset) {
				if (fetchInProgress) {
					console.log('[Favoritos] fetchPage ya en curso, ignorando');
					return;
				}
				fetchInProgress = true;
				update((s) => ({ ...s, loading: true, error: null, page: 1, items: [] }));
			} else {
				if (state.loadingMore || !state.hasMore) return;
				update((s) => ({ ...s, loadingMore: true }));
			}

			try {
				const pageToLoad = reset ? 1 : state.page + 1;
				const params = getServerParams(reset ? { ...state, page: 1 } : state, pageToLoad);

				// Load folders and tags only on initial load
				if (reset && state.folders.length === 0) {
					const [folders, allTags] = await Promise.all([
						apiClient.getFolders(),
						apiClient.getTags()
					]);
					update((s) => ({ ...s, folders, allTags }));
				}

				console.log(`[Favoritos] Cargando página ${pageToLoad}...`, params);
				const result = await apiClient.getFavorites(params);
				const hasMorePages = pageToLoad < result.totalPages;
				console.log(`[Favoritos] Página ${pageToLoad}: ${result.items.length} items (total: ${result.total}, pages: ${result.totalPages}, hasMore: ${hasMorePages})`);

				update((s) => ({
					...s,
					items: reset ? result.items : [...s.items, ...result.items],
					page: pageToLoad,
					total: result.total,
					totalPages: result.totalPages,
					hasMore: pageToLoad < result.totalPages,
					loading: false,
					loadingMore: false,
				}));

				if (reset) {
					await databaseService.saveFavoritos(get({ subscribe }).items);
				}
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				if (reset) {
					const localItems = await databaseService.getAllFavoritos();
					if (localItems.length > 0) {
						update((s) => ({ ...s, items: localItems, loading: false, error: null, hasMore: false }));
					} else {
						update((s) => ({ ...s, loading: false, error: errorMsg }));
						await tauriNotifications.showError(`Error de conexión: ${errorMsg}`);
					}
				} else {
					update((s) => ({ ...s, loadingMore: false, error: errorMsg }));
				}
			} finally {
				if (reset) fetchInProgress = false;
			}
		},

		async loadFoldersAndTags() {
			try {
				const [folders, allTags] = await Promise.all([
					apiClient.getFolders(),
					apiClient.getTags()
				]);
				update((s) => ({ ...s, folders, allTags }));
			} catch (e) {
				console.warn('[Favoritos] Error al cargar folders/tags:', e);
			}
		},

		async create(payload: FavoritoInput) {
			update((s) => ({ ...s, error: null }));
			try {
				const newItem = await apiClient.createFavorite(payload);
				update((s) => ({
					...s,
					items: [newItem, ...s.items],
					total: s.total + 1,
				}));
				return newItem;
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({ ...s, error: errorMsg }));
				await tauriNotifications.showError(`Error al crear favorito: ${errorMsg}`);
				throw error;
			}
		},

		async update(id: string, payload: FavoritoUpdate) {
			update((s) => ({ ...s, error: null }));
			try {
				const updatedItem = await apiClient.updateFavorite(id, payload);
				update((s) => ({
					...s,
					items: s.items.map((item) => (item.id === id ? updatedItem : item)),
				}));
				return updatedItem;
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({ ...s, error: errorMsg }));
				await tauriNotifications.showError(`Error al actualizar favorito: ${errorMsg}`);
				throw error;
			}
		},

		async delete(id: string) {
			update((s) => ({ ...s, error: null }));
			try {
				await apiClient.deleteFavorite(id);
				update((s) => ({
					...s,
					items: s.items.filter((item) => item.id !== id),
					total: Math.max(0, s.total - 1),
				}));
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({ ...s, error: errorMsg }));
				await tauriNotifications.showError(`Error al eliminar favorito: ${errorMsg}`);
				throw error;
			}
		},

		async exportAll() {
			console.log('[Export] Iniciando exportación total...');
			update((s) => ({ ...s, loading: true }));

			try {
				let allExport: any[] = [];
				let currentPage = 1;
				let hasMore = true;

				while (hasMore) {
					console.log(`[Export] Solicitando página ${currentPage} (pageSize: 100)...`);
					const result = await apiClient.getFavorites({ page: currentPage, limit: 100 });
					console.log(`[Export] Recibidos ${result.items.length} elementos`);

					if (!result.items || result.items.length === 0) {
						hasMore = false;
					} else {
						allExport = [...allExport, ...result.items];
						hasMore = currentPage < result.totalPages;
						currentPage++;
					}
				}

				if (allExport.length === 0) {
					throw new Error("No hay favoritos para exportar.");
				}

				console.log(`[Export] Total recolectado: ${allExport.length} favoritos. Normalizando...`);

				const bookmarks = allExport.map(item => ({
					url: item.url,
					title: item.title || "Sin título",
					description: item.description || "",
					notes: item.notes || "",
					folder: item.folder?.name || "",
					tags: item.tags?.map((t: any) => typeof t === 'string' ? t : t.name) || []
				}));

				const exportData = { bookmarks };
				const jsonString = JSON.stringify(exportData, null, 2);

				console.log('[Export] Abriendo diálogo de guardado...');
				const saved = await invoke<boolean>('favorites_export', { jsonData: jsonString });

				console.log('[Export] Diálogo cerrado. Guardado:', saved);
				if (saved) {
					await tauriNotifications.showSuccess("Exportación completada con éxito");
				}
			} catch (error) {
				console.error('[Export Error]', error);
				const errorMsg = error instanceof Error ? error.message : String(error);
				await tauriNotifications.showError(`Error al exportar: ${errorMsg}`);
			} finally {
				update((s) => ({ ...s, loading: false }));
			}
		},

		async exportHtml() {
			console.log('[Export HTML] Iniciando exportación en formato marcadores...');
			update((s) => ({ ...s, loading: true }));

			try {
				const saved = await invoke<boolean>('favorites_export_html');

				console.log('[Export HTML] Resultado:', saved);
				if (saved) {
					await tauriNotifications.showSuccess("Exportación HTML completada con éxito");
				}
			} catch (error) {
				console.error('[Export HTML Error]', error);
				const errorMsg = error instanceof Error ? error.message : String(error);
				await tauriNotifications.showError(`Error al exportar: ${errorMsg}`);
			} finally {
				update((s) => ({ ...s, loading: false }));
			}
		},

		selectFolder(folderId: string | null) {
			update((s) => ({ ...s, selectedFolderId: folderId, selectedTag: null, searchQuery: '' }));
			invoke('preference_set', { key: 'selected_folder_id', value: folderId || '' }).catch(console.warn);
			invoke('preference_set', { key: 'selected_tag', value: '' }).catch(console.warn);
			this.fetchPage(true);
		},

		selectTag(tag: string | null) {
			update((s) => ({ ...s, selectedTag: tag, selectedFolderId: null, searchQuery: '' }));
			invoke('preference_set', { key: 'selected_tag', value: tag || '' }).catch(console.warn);
			invoke('preference_set', { key: 'selected_folder_id', value: '' }).catch(console.warn);
			this.fetchPage(true);
		},

		setSearchQuery(query: string) {
			update((s) => ({ ...s, searchQuery: query }));
		},

		async executeSearch() {
			await this.fetchPage(true);
		},

		setViewMode(mode: 'grid' | 'list') {
			update((s) => ({ ...s, viewMode: mode }));
			invoke('preference_set', { key: 'view_mode', value: mode }).catch((e) =>
				console.warn('[Favoritos] Error al persistir viewMode:', e)
			);
		},

		async restoreViewMode() {
			try {
				const saved = await invoke<string | null>('preference_get', { key: 'view_mode' });
				if (saved === 'grid' || saved === 'list') {
					update((s) => ({ ...s, viewMode: saved }));
				}
			} catch (e) {
				console.warn('[Favoritos] Error al restaurar viewMode:', e);
			}
		},

		async restoreNavigation() {
			try {
				const [folderId, tag] = await Promise.all([
					invoke<string | null>('preference_get', { key: 'selected_folder_id' }),
					invoke<string | null>('preference_get', { key: 'selected_tag' }),
				]);
				if (tag) {
					update((s) => ({ ...s, selectedTag: tag, selectedFolderId: null }));
				} else if (folderId) {
					update((s) => ({ ...s, selectedFolderId: folderId, selectedTag: null }));
				}
			} catch (e) {
				console.warn('[Favoritos] Error al restaurar navegación:', e);
			}
		},

		setItems(items: Favorito[]) {
			update((s) => ({ ...s, items }));
		},

		reset() {
			set(initialState);
		}
	};
}

export const favoritos = createFavoritosStore();

// Derived store for folder counts (real counts from API's bookmarkCount field)
export const folderCounts = derived(favoritos, ($favs) => {
	const counts: Record<string, number> = {};
	$favs.folders.forEach(folder => {
		counts[folder.id] = folder.bookmarkCount ?? 0;
	});
	return counts;
});

// Derived store for tag counts (real counts from API's bookmarkCount field)
export const tagCounts = derived(favoritos, ($favs) => {
	const counts: Record<string, number> = {};
	$favs.allTags.forEach(tag => {
		if (tag.name) {
			counts[tag.name] = tag.bookmarkCount ?? 0;
		}
	});
	return counts;
});
