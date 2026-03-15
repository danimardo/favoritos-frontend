import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import type { Favorito } from '$lib/types/api';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;

export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;

export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
  ref?: U | null;
};

/**
 * Normalizes a string for accent-insensitive and case-insensitive comparison.
 * Removes diacritics (á→a, é→e, ñ→n, ü→u, etc.) and converts to lowercase.
 */
export function normalize(str: string): string {
	return str.normalize('NFD').replace(/[\u0300-\u036f]/g, '').toLowerCase();
}

/**
 * Filters a list of favorites based on a query string.
 * This should be fast (<50ms for 10,000 items ideally).
 */
export function filterFavoritos(favorites: Favorito[], query: string): Favorito[] {
	if (!query) return favorites;

	const normalizedQuery = normalize(query);

	return favorites.filter((f) => {
		const title = normalize(f.title ?? '');
		const url = normalize(f.url ?? '');
		const description = normalize(f.description ?? '');
		const folder = normalize(typeof f.folder === 'string' ? f.folder : f.folder?.name ?? '');
		const tags = f.tags?.map((t) => normalize(typeof t === 'string' ? t : t.name ?? '')) ?? [];

		return (
			title.includes(normalizedQuery) ||
			url.includes(normalizedQuery) ||
			description.includes(normalizedQuery) ||
			folder.includes(normalizedQuery) ||
			tags.some((t) => t.includes(normalizedQuery))
		);
	});
}
