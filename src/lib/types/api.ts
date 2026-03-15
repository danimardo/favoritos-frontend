// TypeScript interfaces for domain entities

export interface Favorito {
	id: string;
	url: string;
	title: string;
	description?: string;
	notes?: string;
	domain: string;
	faviconUrl?: string;
	faviconCached?: string;
	folderId?: string;
	folder?: {
		id: string;
		name: string;
	};
	tags?: {
		id: string;
		name: string;
		color?: string;
	}[];
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
	bookmarkCount?: number;
	createdAt: string;
}

export interface Etiqueta {
	id: string;
	name: string;
	color?: string;
	bookmarkCount?: number;
}

export interface User {
	id: string;
	email: string;
	name?: string;
	isActive: boolean;
	createdAt: string;
}

export interface AuthResponse {
	user: User;
	apiKey: string;
}

export interface ApiConfig {
	apiUrl: string;
	apiKey: string;
	user?: User;
}

export interface ConfigResponse {
	configured: boolean;
	apiUrl: string | null;
	logLevel: string;
}


export interface ImportResult {
	imported: number;
	updated: number;
	skipped: number;
	total: number;
	errors: string[];
}

export interface Paginacion {
	page: number;
	limit: number;
	total: number;
	pages: number;
}

export interface PaginatedFavoritos {
	items: Favorito[];
	total: number;
	totalPages: number;
	page: number;
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
	notes?: string;
	tags?: string[];
	folder?: string;
}

export interface FavoritoUpdate extends Partial<FavoritoInput> {
	visitCount?: number;
}
