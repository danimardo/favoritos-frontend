// Favicon service - Manages favicon caching
import { invoke } from '@tauri-apps/api/core';

const faviconCache = new Map<string, string>();

export const favicons = {
	/**
	 * Get cached favicon for a domain
	 * Returns local path if cached, null otherwise
	 */
	async get(domain: string): Promise<string | null> {
		// Check memory cache first
		if (faviconCache.has(domain)) {
			return faviconCache.get(domain) ?? null;
		}

		try {
			const path = await invoke<string | null>('cache_favicon_get', { domain });
			if (path) {
				faviconCache.set(domain, path);
			}
			return path;
		} catch {
			return null;
		}
	},

	/**
	 * Fetch and cache favicon for a domain
	 * Returns local path after caching
	 */
	async fetch(domain: string): Promise<string | null> {
		try {
			const path = await invoke<string>('cache_favicon_set', { domain });
			faviconCache.set(domain, path);
			return path;
		} catch {
			return null;
		}
	},

	/**
	 * Get or fetch favicon for a domain
	 */
	async getOrFetch(domain: string): Promise<string | null> {
		// Check memory cache
		if (faviconCache.has(domain)) {
			return faviconCache.get(domain) ?? null;
		}

		// Try to get from cache
		const cached = await this.get(domain);
		if (cached) {
			return cached;
		}

		// Fetch from network
		return this.fetch(domain);
	},

	/**
	 * Extract domain from URL
	 */
	extractDomain(url: string): string | null {
		try {
			const urlObj = new URL(url);
			return urlObj.hostname;
		} catch {
			return null;
		}
	},

	/**
	 * Get Google favicon URL (fallback)
	 */
	getGoogleFaviconUrl(domain: string, size: number = 128): string {
		return `https://www.google.com/s2/favicons?domain=${domain}&sz=${size}`;
	},

	/**
	 * Clear memory cache
	 */
	clearMemoryCache() {
		faviconCache.clear();
	}
};
