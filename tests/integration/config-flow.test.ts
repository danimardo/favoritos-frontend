import { render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ConfigForm from '$lib/components/favoritos/ConfigForm.svelte';
import { configuracion } from '$lib/stores/configuracion';
import { invoke } from '@tauri-apps/api/core';
import { tick } from 'svelte';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('ConfigFlow Integration', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    configuracion.reset();
  });

  it('T022: full config flow from input to save', async () => {
    const user = userEvent.setup();
    (invoke as any).mockImplementation((cmd: string, args: any) => {
      console.log(`Mock invoke called: ${cmd}`, args);
      if (cmd === 'config_validate') return Promise.resolve(true);
      if (cmd === 'config_save') return Promise.resolve();
      return Promise.resolve();
    });

    render(ConfigForm);

    const urlInput = screen.getByLabelText('API URL');
    const keyInput = screen.getByLabelText('API Key');
    const validateBtn = screen.getByText('Validar Conexión');
    const saveBtn = screen.getByText('Guardar');

    await user.type(urlInput, 'https://api.example.com');
    await user.type(keyInput, 'sk-123');
    await tick();

    console.log('Clicking validate button...');
    await user.click(validateBtn);
    await tick();
    await tick(); // Double tick for async handler

    await waitFor(() => {
      console.log('Checking for success message...');
      const successMsg = screen.queryByText(/Conexión exitosa/i);
      if (successMsg) console.log('Found it!');
      expect(successMsg).not.toBeNull();
    }, { timeout: 4000 });

    await user.click(saveBtn);
    await tick();

    expect(invoke).toHaveBeenCalledWith('config_save', {
      apiUrl: 'https://api.example.com',
      apiKey: 'sk-123'
    });
  });
});
