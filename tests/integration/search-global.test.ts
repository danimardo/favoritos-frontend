import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import BuscadorGlobal from '$lib/components/favoritos/BuscadorGlobal.svelte';
import { favoritos } from '$lib/stores/favoritos';
import type { Favorito } from '$lib/types/api';
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

// Mock scrollIntoView for JSDOM
window.HTMLElement.prototype.scrollIntoView = vi.fn();

describe('BuscadorGlobal Integration', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    favoritos.reset();
  });

  it('T072: should filter items when typing and open on select', async () => {
    const user = userEvent.setup();
    const mockItems = [
      { id: '1', title: 'Work Mail', url: 'https://mail.work.com', domain: 'work.com', visitCount: 0, tags: ['work'], folder: 'Jobs' },
      { id: '2', title: 'GitHub', url: 'https://github.com', domain: 'github.com', visitCount: 0, tags: ['dev'], folder: 'Dev' },
      { id: '3', title: 'Reddit', url: 'https://reddit.com', domain: 'reddit.com', visitCount: 0, tags: ['fun'], folder: 'Personal' },
    ] as unknown as Favorito[];

    favoritos.setItems(mockItems);

    render(BuscadorGlobal);
    await tick();

    // Verify initial state (all items present)
    expect(screen.getByText('Work Mail')).toBeDefined();
    expect(screen.getByText('GitHub')).toBeDefined();
    expect(screen.getByText('Reddit')).toBeDefined();

    // Search for "work"
    const input = screen.getByPlaceholderText('Buscar favoritos...');
    await user.type(input, 'work');
    await tick();

    // Verify filtered state
    expect(screen.queryByText('Work Mail')).toBeDefined();
    expect(screen.queryByText('GitHub')).toBeNull();
    expect(screen.queryByText('Reddit')).toBeNull();

    // Select the item
    const item = screen.getByText('Work Mail');
    await user.click(item);
    
    expect(invoke).toHaveBeenCalledWith('favorites_visit', { id: '1' });
  });

  it('should show empty state when no results', async () => {
    const user = userEvent.setup();
    favoritos.setItems([
        { id: '1', title: 'Test', url: 'https://test.com', domain: 'test.com', visitCount: 0, tags: [] } as unknown as Favorito
    ]);

    render(BuscadorGlobal);
    await tick();

    const input = screen.getByPlaceholderText('Buscar favoritos...');
    await user.type(input, 'nothing matches this');
    await tick();

    expect(screen.getByText('No se encontraron favoritos.')).toBeDefined();
  });
});
