// API Client service - HTTP client for Favoritos API
import { invoke } from '@tauri-apps/api/core';
import type { Favorito, FavoritoInput, FavoritoUpdate, Carpeta, Etiqueta, ImportResult } from '$lib/types/api';

export const apiClient = {
	// Favorites
	async getFavorites(page?: number, limit?: number): Promise<Favorito[]> {
		return invoke<Favorito[]>('favorites_get', { page, limit });
	},

	async getFavoriteById(id: string): Promise<Favorito> {
		return invoke<Favorito>('favorites_get_by_id', { id });
	},

	async createFavorite(favorite: FavoritoInput): Promise<Favorito> {
		return invoke<Favorito>('favorites_create', { favorite });
	},

	async updateFavorite(id: string, favorite: FavoritoUpdate): Promise<Favorito> {
		return invoke<Favorito>('favorites_update', { id, favorite });
	},

	async deleteFavorite(id: string): Promise<void> {
		return invoke<void>('favorites_delete', { id });
	},

	async fetchMetadata(url: string): Promise<{ title?: string; description?: string; faviconUrl?: string }> {
		return invoke<{ title?: string; description?: string; faviconUrl?: string }>('metadata_fetch', { url });
	},

	async visitFavorite(id: string): Promise<Favorito> {
		return invoke<Favorito>('favorites_visit', { id });
	},

	async openUrl(url: string): Promise<void> {
		return invoke<void>('favorites_open_url', { url });
	},

	// Folders
	async getFolders(): Promise<Carpeta[]> {
		return invoke<Carpeta[]>('folders_get');
	},

	async createFolder(name: string): Promise<Carpeta> {
		return invoke<Carpeta>('folders_create', { name });
	},

	// Tags
	async getTags(): Promise<Etiqueta[]> {
		return invoke<Etiqueta[]>('tags_get');
	},

	async createTag(name: string, color?: string): Promise<Etiqueta> {
		return invoke<Etiqueta>('tags_create', { name, color });
	},

	// Import
	async importFavorites(data: string): Promise<ImportResult> {
		return invoke<ImportResult>('import_favorites', { data });
	}
};
