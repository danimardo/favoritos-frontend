import { describe, it, expect } from 'vitest';
import type { Favorito } from '$lib/types/api';
import { filterFavoritos } from '$lib/utils';

describe('Search Utility', () => {
  const mockFavorites: Favorito[] = Array.from({ length: 1000 }).map((_, i) => ({
    id: String(i),
    url: `https://example${i}.com`,
    title: i % 2 === 0 ? `Favorite ${i} Title` : `Bookmark ${i}`,
    description: `Description for ${i}`,
    domain: 'example.com',
    visitCount: 0,
    isArchived: false,
    isRead: false,
    tags: i % 3 === 0 ? ['tag1', 'tag2'] : ['other'],
    folder: i % 5 === 0 ? 'Work' : 'Personal',
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  }));

  it('T071: should filter favorites by title', () => {
    const startTime = performance.now();
    const result = filterFavoritos(mockFavorites, 'Favorite 10');
    const endTime = performance.now();

    expect(result.length).toBeGreaterThan(0);
    expect(result.every(f => f.title.toLowerCase().includes('favorite 10'))).toBe(true);
    expect(endTime - startTime).toBeLessThan(50);
  });

  it('should filter favorites by url', () => {
    const result = filterFavoritos(mockFavorites, 'example50.com');
    expect(result.length).toBeGreaterThan(0);
    expect(result.every(f => f.url.toLowerCase().includes('example50.com'))).toBe(true);
  });

  it('should filter favorites by tags', () => {
    const result = filterFavoritos(mockFavorites, 'tag1');
    expect(result.length).toBeGreaterThan(0);
    expect(result.every(f => f.tags?.includes('tag1'))).toBe(true);
  });

  it('should filter favorites by folder', () => {
    const result = filterFavoritos(mockFavorites, 'Work');
    // Folder match might be partial or full
    expect(result.length).toBeGreaterThan(0);
  });

  it('should be case-insensitive', () => {
    const result1 = filterFavoritos(mockFavorites, 'FAVORITE');
    const result2 = filterFavoritos(mockFavorites, 'favorite');
    expect(result1.length).toBe(result2.length);
    expect(result1.length).toBeGreaterThan(0);
  });

  it('should return all favorites when query is empty', () => {
    const result = filterFavoritos(mockFavorites, '');
    expect(result.length).toBe(mockFavorites.length);
  });
});
