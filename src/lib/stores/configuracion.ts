// Configuration store - Manages API configuration state
import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ConfigResponse } from '$lib/types/api';

interface ConfiguracionState {
	configured: boolean;
	apiUrl: string | null;
	loading: boolean;
	error: string | null;
}

const initialState: ConfiguracionState = {
	configured: false,
	apiUrl: null,
	loading: false,
	error: null
};

function createConfiguracionStore() {
	const { subscribe, set, update } = writable<ConfiguracionState>(initialState);

	return {
		subscribe,

		async init() {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				const response = await invoke<ConfigResponse>('config_get');
				update((s) => ({
					...s,
					configured: response.configured,
					apiUrl: response.apiUrl ?? null,
					loading: false
				}));
			} catch (error) {
				update((s) => ({
					...s,
					loading: false,
					error: error instanceof Error ? error.message : String(error)
				}));
			}
		},

		async save(apiUrl: string, apiKey: string) {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				await invoke('config_save', { apiUrl, apiKey });
				update((s) => ({
					...s,
					configured: true,
					apiUrl,
					loading: false
				}));
			} catch (error) {
				update((s) => ({
					...s,
					loading: false,
					error: error instanceof Error ? error.message : String(error)
				}));
				throw error;
			}
		},

		async validate(apiUrl: string, apiKey: string): Promise<boolean> {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				const valid = await invoke<boolean>('config_validate', { apiUrl, apiKey });
				update((s) => ({ ...s, loading: false }));
				return valid;
			} catch (error) {
				update((s) => ({
					...s,
					loading: false,
					error: error instanceof Error ? error.message : String(error)
				}));
				return false;
			}
		},

		async clear() {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				await invoke('config_clear');
				set(initialState);
			} catch (error) {
				update((s) => ({
					...s,
					loading: false,
					error: error instanceof Error ? error.message : String(error)
				}));
				throw error;
			}
		},

		getConfigured(): boolean {
			return get({ subscribe }).configured;
		},

		reset() {
			set(initialState);
		}
	};
}

export const configuracion = createConfiguracionStore();

// Derived store for checking if app is ready
export const isConfigured = derived(configuracion, ($config) => $config.configured);
