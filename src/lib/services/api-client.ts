// API Client service - HTTP client for Favoritos API
import { invoke } from '@tauri-apps/api/core';
import type {
	Favorito,
	FavoritoInput,
	FavoritoUpdate,
	Carpeta,
	Etiqueta,
	ImportResult,
	AuthResponse,
	User,
	PaginatedFavoritos
} from '$lib/types/api';

export const apiClient = {
	// Auth
	async login(apiUrl: string, email: string, password: string): Promise<AuthResponse> {
		return invoke<AuthResponse>('auth_login', { apiUrl, email, password });
	},

	async register(apiUrl: string, email: string, password: string, name?: string): Promise<AuthResponse> {
		return invoke<AuthResponse>('auth_register', { apiUrl, email, password, name });
	},

	async getProfile(): Promise<User> {
		return invoke<User>('user_get_profile');
	},

	// Favorites
	async getFavorites(params: {
		page?: number;
		limit?: number;
		search?: string;
		folder?: string;
		tags?: string;
	} = {}): Promise<PaginatedFavoritos> {
		return invoke<PaginatedFavoritos>('favorites_get', params);
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

	async deleteTag(id: string): Promise<void> {
		return invoke<void>('tags_delete', { id });
	},

	// Import
	async importFavorites(data: string): Promise<ImportResult> {
		return invoke<ImportResult>('import_favorites', { data });
	}
};
