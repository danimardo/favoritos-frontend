import { render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import FavoritoGrid from '$lib/components/favoritos/FavoritoGrid.svelte';
import FavoritoForm from '$lib/components/favoritos/FavoritoForm.svelte';
import { favoritos } from '$lib/stores/favoritos';
import { invoke } from '@tauri-apps/api/core';
import { tick } from 'svelte';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock database service
vi.mock('$lib/services/database', () => ({
  databaseService: {
    saveFavoritos: vi.fn().mockResolvedValue(undefined),
    getAllFavoritos: vi.fn().mockResolvedValue([]),
  },
}));

describe('Favoritos CRUD Integration', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    favoritos.reset();
  });

  it('T038: should list favorites and create a new one', async () => {
    const user = userEvent.setup();
    const mockList = [
      { id: '1', title: 'Existing Fav', url: 'https://old.com', domain: 'old.com', tags: [] },
    ];
    const newFav = { id: '2', title: 'New Fav', url: 'https://new.com', domain: 'new.com', tags: ['test'] };

    (invoke as any).mockImplementation((cmd: string, args: any) => {
      if (cmd === 'favorites_get') return Promise.resolve({ items: mockList, total: mockList.length, totalPages: 1, page: 1 });
      if (cmd === 'favorites_create') return Promise.resolve(newFav);
      if (cmd === 'folders_get') return Promise.resolve([]);
      if (cmd === 'tags_get') return Promise.resolve([]);
      if (cmd === 'metadata_fetch') return Promise.resolve({ title: '', description: '' });
      return Promise.resolve();
    });

    // Test Listing
    render(FavoritoGrid);
    await favoritos.fetchPage(true);
    await tick();

    expect(screen.getByText('Existing Fav')).toBeDefined();

    // Test Creation Form
    const onSave = vi.fn();
    render(FavoritoForm, { onSave });

    const urlInput = screen.getByLabelText('URL');
    const titleInput = screen.getByLabelText('Título');
    const tagsInput = screen.getByLabelText(/Etiquetas/);
    const submitBtn = screen.getByText('Crear');

    await user.type(urlInput, 'https://new.com');
    await user.type(titleInput, 'New Fav');
    await user.type(tagsInput, 'test{Enter}');
    
    await user.click(submitBtn);
    await tick();

    expect(invoke).toHaveBeenCalledWith('favorites_create', {
      favorite: { 
        url: 'https://new.com', 
        title: 'New Fav', 
        tags: ['test'],
        folder: undefined
      }
    });
    expect(onSave).toHaveBeenCalled();
  });
});
