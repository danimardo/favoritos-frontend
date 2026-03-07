import { describe, it, expect, vi, beforeEach } from 'vitest';
import { configuracion } from '$lib/stores/configuracion';
import { invoke } from '@tauri-apps/api/core';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Configuracion Store', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    configuracion.reset();
  });

  it('T019: init should return null when no config exists', async () => {
    (invoke as any).mockResolvedValue({ configured: false, apiUrl: null });

    await configuracion.init();

    let state: any;
    configuracion.subscribe(s => state = s)();
    expect(state.apiUrl).toBeNull();
    expect(state.configured).toBe(false);
    expect(invoke).toHaveBeenCalledWith('config_get');
  });

  it('T020: save should save and update store', async () => {
    (invoke as any).mockResolvedValue(undefined);

    await configuracion.save('https://api.test.com', 'test-key');

    let state: any;
    configuracion.subscribe(s => state = s)();
    expect(state.apiUrl).toBe('https://api.test.com');
    expect(state.configured).toBe(true);
    expect(invoke).toHaveBeenCalledWith('config_save', {
      apiUrl: 'https://api.test.com',
      apiKey: 'test-key',
    });
  });

  it('T021: validate should return success result', async () => {
    (invoke as any).mockResolvedValue(true);

    const result = await configuracion.validate('https://api.test.com', 'test-key');

    expect(result).toBe(true);
    expect(invoke).toHaveBeenCalledWith('config_validate', {
      apiUrl: 'https://api.test.com',
      apiKey: 'test-key',
    });
  });
});
