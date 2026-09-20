import { beforeEach, describe, expect, it, vi } from 'vitest';
import { api } from './api';
import * as core from '@tauri-apps/api/core';
import { resolve } from '$app/paths';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/webviewWindow', () => ({
  getCurrentWebviewWindow: () => ({
    hide: vi.fn().mockResolvedValue(undefined),
  }),
}));

describe('Memebin Frontend API SDK', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('ensureStorage calls correct Tauri command', async () => {
    vi.mocked(core.invoke).mockResolvedValue('/mock/app/data/memes');
    const result = await api.ensureStorage();

    expect(core.invoke).toHaveBeenCalledWith('ensure_storage');
    expect(result).toBe('/mock/app/data/memes');
  });

  it('listMemes forwards sort option to backend', async () => {
    vi.mocked(core.invoke).mockResolvedValue([]);
    await api.searchMemes('cat');

    expect(core.invoke).toHaveBeenCalledWith('search_memes', { query: 'cat' });
  });

  it("saveMemeBytes converts Uint8Array to number array", async () => {
    vi.mocked(core.invoke).mockResolvedValue({ id: 'test-uuid' });
    const bytes = new Uint8Array([1, 2, 3, 4]);

    await api.saveMemeBytes(bytes, 'png', 'Test Meme', ['tag1']);

    expect(core.invoke).toHaveBeenCalledWith('save_meme_bytes', {
      bytes: [1, 2, 3, 4],
      extension: 'png',
      name: 'Test Meme',
      tags: ['tag1']
    });
  });
});
