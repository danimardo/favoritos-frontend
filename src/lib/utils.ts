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
 * Filters a list of favorites based on a query string.
 * This should be fast (<50ms for 10,000 items ideally).
 */
export function filterFavoritos(favorites: Favorito[], query: string): Favorito[] {
	if (!query) return favorites;

	const lowerQuery = query.toLowerCase();

	return favorites.filter((f) => {
		const title = f.title?.toLowerCase() ?? '';
		const url = f.url?.toLowerCase() ?? '';
		const description = f.description?.toLowerCase() ?? '';
		const folder = f.folder?.toLowerCase() ?? '';
		const tags = f.tags?.map((t) => t.toLowerCase()) ?? [];

		return (
			title.includes(lowerQuery) ||
			url.includes(lowerQuery) ||
			description.includes(lowerQuery) ||
			folder.includes(lowerQuery) ||
			tags.some((t) => t.includes(lowerQuery))
		);
	});
}
