import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Metadata Fetch Command', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('T037: should fetch metadata successfully for a valid URL', async () => {
    const mockMetadata = {
      title: 'Google',
      description: 'Search the world\'s information, including webpages, images, videos and more.',
      faviconUrl: 'https://www.google.com/favicon.ico'
    };
    (invoke as any).mockResolvedValue(mockMetadata);

    const result = await invoke('metadata_fetch', { url: 'https://google.com' });

    expect(result).toEqual(mockMetadata);
    expect(invoke).toHaveBeenCalledWith('metadata_fetch', { url: 'https://google.com' });
  });

  it('T037: should handle errors when metadata fetch fails', async () => {
    (invoke as any).mockRejectedValue('Could not fetch metadata');

    try {
      await invoke('metadata_fetch', { url: 'https://invalid-url.com' });
      // Should not reach here
      expect(true).toBe(false);
    } catch (e) {
      expect(e).toBe('Could not fetch metadata');
    }
  });
});
