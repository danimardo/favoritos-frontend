import { writable, get, derived } from 'svelte/store';
import type { Favorito, FavoritoInput, FavoritoUpdate, Carpeta } from '$lib/types/api';
import { databaseService } from '$lib/services/database';
import { apiClient } from '$lib/services/api-client';
import { tauriNotifications } from '$lib/services/tauri';
import { invoke } from '@tauri-apps/api/core';

interface FavoritosState {
	items: Favorito[];
	folders: Carpeta[];
	loading: boolean;
	error: string | null;
	page: number;
	hasMore: boolean;
	selectedFolderId: string | null;
	selectedTag: string | null;
	viewMode: 'grid' | 'list';
	searchQuery: string;
}

const initialState: FavoritosState = {
	items: [],
	folders: [],
	loading: false,
	error: null,
	page: 1,
	hasMore: true,
	selectedFolderId: null,
	selectedTag: null,
	viewMode: 'grid',
	searchQuery: ''
};

function createFavoritosStore() {
	const { subscribe, set, update } = writable<FavoritosState>(initialState);

	return {
		subscribe,

		async fetchAll() {
			update((s) => ({ ...s, loading: true, error: null, page: 1, items: [] }));
			try {
				const [allItems, folders] = await Promise.all([
					apiClient.getFavorites(1, 100),
					apiClient.getFolders()
				]);

				const items = allItems.filter(item => !item.isArchived && !item.isRead);
				
				update((s) => ({ 
					...s, 
					items, 
					folders,
					loading: false, 
					hasMore: allItems.length >= 100 
				}));
				
				await databaseService.saveFavoritos(items);
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				const localItems = await databaseService.getAllFavoritos();
				if (localItems.length > 0) {
					update((s) => ({ ...s, items: localItems, loading: false, error: null, hasMore: false }));
				} else {
					update((s) => ({ ...s, loading: false, error: errorMsg }));
					await tauriNotifications.showError(`Error de conexión: ${errorMsg}`);
				}
			}
		},

		async fetchNextPage() {
			const state = get(this);
			if (state.loading || !state.hasMore) return;

			update((s) => ({ ...s, loading: true }));
			try {
				const nextPage = state.page + 1;
				const allNewItems = await apiClient.getFavorites(nextPage, 100);
				const newItems = allNewItems.filter(item => !item.isArchived && !item.isRead);
				
				update((s) => ({
					...s,
					items: [...s.items, ...newItems],
					page: nextPage,
					loading: false,
					hasMore: allNewItems.length >= 100
				}));
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({ ...s, loading: false, error: errorMsg }));
				await tauriNotifications.showError(`Error de conexión (página ${state.page + 1}): ${errorMsg}`);
			}
		},

		async create(payload: FavoritoInput) {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				const newItem = await apiClient.createFavorite(payload);
				update((s) => ({
					...s,
					items: [newItem, ...s.items],
					loading: false
				}));
				return newItem;
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({ ...s, loading: false, error: errorMsg }));
				await tauriNotifications.showError(`Error al crear favorito: ${errorMsg}`);
				throw error;
			}
		},

		async update(id: string, payload: FavoritoUpdate) {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				const updatedItem = await apiClient.updateFavorite(id, payload);
				update((s) => ({
					...s,
					items: s.items.map((item) => (item.id === id ? updatedItem : item)),
					loading: false
				}));
				return updatedItem;
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({ ...s, loading: false, error: errorMsg }));
				await tauriNotifications.showError(`Error al actualizar favorito: ${errorMsg}`);
				throw error;
			}
		},

		async delete(id: string) {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				await apiClient.deleteFavorite(id);
				update((s) => ({
					...s,
					items: s.items.filter((item) => item.id !== id),
					loading: false
				}));
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({ ...s, loading: false, error: errorMsg }));
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
					const pageItems = await apiClient.getFavorites(currentPage, 100);
					console.log(`[Export] Recibidos ${pageItems.length} elementos`);

					if (!pageItems || pageItems.length === 0) {
						hasMore = false;
					} else {
						allExport = [...allExport, ...pageItems];
						if (pageItems.length < 100) {
							hasMore = false;
						} else {
							currentPage++;
						}
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
			update((s) => ({ ...s, selectedFolderId: folderId, selectedTag: null }));
			invoke('preference_set', { key: 'selected_folder_id', value: folderId || '' }).catch(console.warn);
			invoke('preference_set', { key: 'selected_tag', value: '' }).catch(console.warn);
		},

		selectTag(tag: string | null) {
			update((s) => ({ ...s, selectedTag: tag, selectedFolderId: null }));
			invoke('preference_set', { key: 'selected_tag', value: tag || '' }).catch(console.warn);
			invoke('preference_set', { key: 'selected_folder_id', value: '' }).catch(console.warn);
		},

		setSearchQuery(query: string) {
			update((s) => ({ ...s, searchQuery: query }));
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

// Derived store for filtered items by folder or tag
export const filteredByFolder = derived(favoritos, ($favs) => {
	if ($favs.selectedTag) {
		return $favs.items.filter(item =>
			item.tags?.some(t => {
				const tagName = typeof t === 'string' ? t : t.name;
				return tagName === $favs.selectedTag;
			})
		);
	}
	if (!$favs.selectedFolderId) return $favs.items;
	return $favs.items.filter(item =>
		item.folderId === $favs.selectedFolderId ||
		item.folder?.id === $favs.selectedFolderId
	);
});

// Derived store for folder counts
export const folderCounts = derived(favoritos, ($favs) => {
	const counts: Record<string, number> = {};
	$favs.items.forEach(item => {
		const fId = item.folderId || item.folder?.id;
		if (fId) {
			counts[fId] = (counts[fId] || 0) + 1;
		}
	});
	return counts;
});

// Derived store for tag counts (sorted by count desc)
export const tagCounts = derived(favoritos, ($favs) => {
	const counts: Record<string, number> = {};
	$favs.items.forEach(item => {
		item.tags?.forEach(t => {
			const name = typeof t === 'string' ? t : t.name;
			if (name) counts[name] = (counts[name] || 0) + 1;
		});
	});
	return counts;
});
