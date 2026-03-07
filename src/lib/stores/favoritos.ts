import { writable, get } from 'svelte/store';
import type { Favorito, FavoritoInput, FavoritoUpdate } from '$lib/types/api';
import { databaseService } from '$lib/services/database';
import { apiClient } from '$lib/services/api-client';
import { tauriNotifications } from '$lib/services/tauri';

interface FavoritosState {
	items: Favorito[];
	loading: boolean;
	error: string | null;
	page: number;
	hasMore: boolean;
}

const initialState: FavoritosState = {
	items: [],
	loading: false,
	error: null,
	page: 1,
	hasMore: true
};

function createFavoritosStore() {
	const { subscribe, set, update } = writable<FavoritosState>(initialState);

	return {
		subscribe,

		async fetchAll() {
			update((s) => ({ ...s, loading: true, error: null, page: 1, items: [] }));
			try {
				const allItems = await apiClient.getFavorites(1);
				// FR-021: Ignore isArchived and isRead in frontend
				const items = allItems.filter(item => !item.isArchived && !item.isRead);
				
				update((s) => ({ 
					...s, 
					items, 
					loading: false, 
					hasMore: allItems.length >= 50 
				}));
				
				// Sync to local DB
				await databaseService.saveFavoritos(items);
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				
				// If offline or error, try to load from local DB
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
				const allNewItems = await apiClient.getFavorites(nextPage);
				const newItems = allNewItems.filter(item => !item.isArchived && !item.isRead);
				
				update((s) => ({
					...s,
					items: [...s.items, ...newItems],
					page: nextPage,
					loading: false,
					hasMore: allNewItems.length >= 50
				}));
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({
					...s,
					loading: false,
					error: errorMsg
				}));
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
				update((s) => ({
					...s,
					loading: false,
					error: errorMsg
				}));
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
				update((s) => ({
					...s,
					loading: false,
					error: errorMsg
				}));
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
				update((s) => ({
					...s,
					loading: false,
					error: errorMsg
				}));
				await tauriNotifications.showError(`Error al eliminar favorito: ${errorMsg}`);
				throw error;
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
