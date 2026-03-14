// Configuration store - Manages API configuration state
import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ConfigResponse, User, AuthResponse } from '$lib/types/api';
import { apiClient } from '$lib/services/api-client';

interface ConfiguracionState {
	configured: boolean;
	ready: boolean;
	apiUrl: string | null;
	logLevel: string;
	user: User | null;
	loading: boolean;
	error: string | null;
}

const initialState: ConfiguracionState = {
	configured: false,
	ready: false,
	apiUrl: null,
	logLevel: 'info',
	user: null,
	loading: false,
	error: null
};

function createConfiguracionStore() {
	const { subscribe, set, update } = writable<ConfiguracionState>(initialState);

	return {
		subscribe,

		async init() {
			console.log('[ConfigStore] Inicializando...');
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				const response = await invoke<ConfigResponse>('config_get');
				console.log('[ConfigStore] Configuración cargada:', response);
				
				let user = null;
				let configured = response.configured;

				if (configured) {
					try {
						console.log('[ConfigStore] Intentando obtener perfil...');
						user = await apiClient.getProfile();
						console.log('[ConfigStore] Perfil obtenido:', user);
					} catch (error) {
						console.warn('[ConfigStore] No se pudo obtener el perfil (sesión expirada):', error);
						configured = false;
					}
				}

				update((s) => ({
					...s,
					configured,
					ready: true,
					apiUrl: response.apiUrl ?? null,
					logLevel: response.logLevel || 'info',
					user,
					loading: false
				}));
			} catch (error) {
				console.error('[ConfigStore] Error crítico en init:', error);
				update((s) => ({
					...s,
					loading: false,
					ready: true,
					error: error instanceof Error ? error.message : String(error)
				}));
			} finally {
				console.log('[ConfigStore] Inicialización finalizada.');
			}
		},

		async login(apiUrl: string, email: string, password: string) {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				const response = await apiClient.login(apiUrl, email, password);
				// Guardamos la configuración (URL y API Key obtenida)
				await invoke('config_save', { 
					apiUrl, 
					apiKey: response.apiKey, 
					logLevel: get({ subscribe }).logLevel 
				});
				
				update((s) => ({
					...s,
					configured: true,
					apiUrl,
					user: response.user,
					loading: false
				}));
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({
					...s,
					loading: false,
					error: errorMsg
				}));
				throw error;
			}
		},

		async register(apiUrl: string, email: string, password: string, name?: string) {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				const response = await apiClient.register(apiUrl, email, password, name);
				// Guardamos la configuración
				await invoke('config_save', { 
					apiUrl, 
					apiKey: response.apiKey, 
					logLevel: get({ subscribe }).logLevel 
				});
				
				update((s) => ({
					...s,
					configured: true,
					apiUrl,
					user: response.user,
					loading: false
				}));
			} catch (error) {
				const errorMsg = error instanceof Error ? error.message : String(error);
				update((s) => ({
					...s,
					loading: false,
					error: errorMsg
				}));
				throw error;
			}
		},

		async logout() {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				await invoke('config_clear');
				// Conservar apiUrl y logLevel tras cerrar sesión
				update((s) => ({ ...initialState, ready: true, apiUrl: s.apiUrl, logLevel: s.logLevel }));
			} catch (error) {
				update((s) => ({
					...s,
					loading: false,
					error: error instanceof Error ? error.message : String(error)
				}));
				throw error;
			}
		},

		async saveSettings(apiUrl: string, logLevel: string, apiKey?: string) {
			update((s) => ({ ...s, loading: true, error: null }));
			try {
				await invoke('config_update_settings', { apiUrl, apiKey: apiKey || null, logLevel });
				update((s) => ({
					...s,
					apiUrl,
					logLevel,
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
export const isConfigured = derived(configuracion, ($config) => $config.configured && $config.user !== null);
export const currentUser = derived(configuracion, ($config) => $config.user);
