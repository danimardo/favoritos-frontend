// TypeScript interfaces for domain entities

export interface Favorito {
	id: string;
	url: string;
	title: string;
	description?: string;
	domain: string;
	faviconUrl?: string;
	faviconCached?: string;
	tags?: string[];
	folder?: string;
	visitCount: number;
	isArchived: boolean;
	isRead: boolean;
	createdAt: string;
	updatedAt: string;
}

export interface Carpeta {
	id: string;
	name: string;
	order: number;
	createdAt: string;
}

export interface Etiqueta {
	id: string;
	name: string;
	color?: string;
}

export interface ApiConfig {
	apiUrl: string;
	apiKey: string;
}

export interface ConfigResponse {
	configured: boolean;
	apiUrl: string | null;
	logLevel: string;
}


export interface ImportResult {
	imported: number;
	skipped: number;
	errors: string[];
}

export interface Paginacion {
	page: number;
	limit: number;
	total: number;
	pages: number;
}

export interface ApiResponse<T> {
	success: boolean;
	data?: T;
	message?: string;
}

export interface FavoritoInput {
	url: string;
	title: string;
	description?: string;
	tags?: string[];
	folder?: string;
}

export interface FavoritoUpdate extends Partial<FavoritoInput> {
	visitCount?: number;
}
