import { describe, it, expect, vi, beforeEach } from 'vitest';
import { favoritos } from '$lib/stores/favoritos';
import type { Favorito } from '$lib/types/api';
import { invoke } from '@tauri-apps/api/core';
import { databaseService } from '$lib/services/database';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock database service
vi.mock('$lib/services/database', () => ({
  databaseService: {
    saveFavoritos: vi.fn(),
    getAllFavoritos: vi.fn(),
  },
}));

describe('Favoritos Store', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    favoritos.reset();
    (databaseService.saveFavoritos as any).mockResolvedValue(undefined);
    (databaseService.getAllFavoritos as any).mockResolvedValue([]);
  });

  it('T033: fetchAll should populate store with list of favorites', async () => {
    const mockList = [
      { id: '1', title: 'Google', url: 'https://google.com', category_id: null, tags: [] },
      { id: '2', title: 'GitHub', url: 'https://github.com', category_id: null, tags: [] },
    ];
    (invoke as any).mockResolvedValue(mockList);

    await favoritos.fetchAll();

    let state: any;
    favoritos.subscribe(s => state = s)();
    expect(state.items).toHaveLength(2);
    expect(state.items[0].title).toBe('Google');
    expect(invoke).toHaveBeenCalledWith('favorites_get', { page: 1, limit: undefined });
  });

  it('T034: create should add new favorite to store', async () => {
    const newFav = { id: '3', title: 'Svelte', url: 'https://svelte.dev', category_id: null, tags: [] };
    (invoke as any).mockResolvedValue(newFav);

    await favoritos.create({ title: 'Svelte', url: 'https://svelte.dev' });

    let state: any;
    favoritos.subscribe(s => state = s)();
    expect(state.items).toContainEqual(newFav);
    expect(invoke).toHaveBeenCalledWith('favorites_create', {
      favorite: { title: 'Svelte', url: 'https://svelte.dev' }
    });
  });

  it('T035: update should modify existing favorite in store', async () => {
    // Pre-populate
    const initialFav = { id: '1', title: 'Old Title', url: 'https://test.com', category_id: null, tags: [] } as unknown as Favorito;
    const updatedFav = { ...initialFav, title: 'New Title' };
    
    // Manual set for test setup
    favoritos.setItems([initialFav]);
    
    (invoke as any).mockResolvedValue(updatedFav);

    await favoritos.update('1', { title: 'New Title' });

    let state: any;
    favoritos.subscribe(s => state = s)();
    expect(state.items[0].title).toBe('New Title');
    expect(invoke).toHaveBeenCalledWith('favorites_update', {
      id: '1',
      favorite: { title: 'New Title' }
    });
  });

  it('T036: delete should remove favorite from store', async () => {
    const fav = { id: '1', title: 'Delete Me', url: 'https://test.com', category_id: null, tags: [] } as unknown as Favorito;
    favoritos.setItems([fav]);
    
    (invoke as any).mockResolvedValue(undefined);

    await favoritos.delete('1');

    let state: any;
    favoritos.subscribe(s => state = s)();
    expect(state.items).toHaveLength(0);
    expect(invoke).toHaveBeenCalledWith('favorites_delete', { id: '1' });
  });

  it('T037: should handle 401/403 errors and update error state', async () => {
    (invoke as any).mockRejectedValue('Unauthorized');

    await favoritos.fetchAll();

    let state: any;
    favoritos.subscribe(s => state = s)();
    expect(state.error).toBe('Unauthorized');
  });
});
