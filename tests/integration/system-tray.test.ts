import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { tauriWindow } from '$lib/services/tauri';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('System Tray Integration (Window behavior)', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('T080: should call window_hide_main command', async () => {
    await tauriWindow.hideMain();
    expect(invoke).toHaveBeenCalledWith('window_hide_main');
  });

  it('should call window_show_search command', async () => {
    await tauriWindow.showSearch();
    expect(invoke).toHaveBeenCalledWith('window_show_search');
  });
});
