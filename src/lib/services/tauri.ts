// Tauri service - Wrapper for Tauri commands and events
import { invoke } from '@tauri-apps/api/core';
import { emit, listen, type UnlistenFn } from '@tauri-apps/api/event';

// ==================== Window Commands ====================

export const tauriWindow = {
	async hideMain(): Promise<void> {
		return invoke<void>('window_hide_main');
	},

	async showSearch(): Promise<void> {
		return invoke<void>('window_show_search');
	}
};

// ==================== Notifications ====================

export const tauriNotifications = {
	async show(title: string, body: string): Promise<void> {
		return invoke<void>('app_show_notification', { title, body });
	},

	async showError(message: string): Promise<void> {
		return invoke<void>('app_show_notification', {
			title: 'Error - Favoritos Desktop',
			body: message
		});
	},

	async showSuccess(message: string): Promise<void> {
		return invoke<void>('app_show_notification', {
			title: 'Favoritos Desktop',
			body: message
		});
	}
};

// ==================== App Commands ====================

export const tauriApp = {
	async quit(): Promise<void> {
		return invoke<void>('app_quit');
	}
};

// ==================== Events ====================

export type NewFavoriteEvent = {
	onNewFavorite(callback: () => void): Promise<UnlistenFn>;
};

export const tauriEvents = {
	async onNewFavorite(callback: () => void): Promise<UnlistenFn> {
		return listen('open-new-favorite', () => callback());
	}
};

// ==================== Utility ====================

export async function openExternalUrl(url: string): Promise<void> {
	return invoke<void>('favorites_open_url', { url });
}
