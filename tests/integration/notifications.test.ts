import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { tauriNotifications } from '$lib/services/tauri';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Notifications Integration', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('T081: should call app_show_notification command', async () => {
    await tauriNotifications.show('Test Title', 'Test Body');
    
    expect(invoke).toHaveBeenCalledWith('app_show_notification', {
      title: 'Test Title',
      body: 'Test Body'
    });
  });

  it('should show error notification', async () => {
    await tauriNotifications.showError('Something went wrong');
    
    expect(invoke).toHaveBeenCalledWith('app_show_notification', {
      title: 'Error - Favoritos Desktop',
      body: 'Something went wrong'
    });
  });
});
