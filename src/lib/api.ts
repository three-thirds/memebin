import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";


//Meme holds data which matches struct shape from Rust backend
export interface Meme {
  id: string;
  name: string;
  filename: string;
  extension: string;
  tags: string[];
  size_bytes: number;
  content_hash: string;
  favorite: boolean;
  use_count: number;
  last_used_at: string | null;
  created_at: string;
  updated_at: string;
}

export type MemeSort = 'created' | 'recent' | 'favorites' | 'name';

export interface LibraryStats {
  count: number;
  favorites: number;
  total_bytes: number;
}

export interface Binding {
  trigger: string;
  meme_id: string;
}


// Typed API Client

export const api = {

  /**
   * Initializes persistent storage directory at OS AppData location
   * Should be called once when the app boots
   * @returns The resolved absolute path to the memes storage directoy.
   */
  async ensureStorage(): Promise<string> {
    return await invoke<string>('ensure_storage');
  },

  /**
   * Retrives all saved memes from the persistent storage directory
   * @param sort - Sorting criteria: 'created'(default), 'recent', 'favourite' or 'name'
   */
  async listMemes(sort: MemeSort = 'created'): Promise<Meme[]> {
    return await invoke<Meme[]>('list_memes_sorted', { sort });
  },

  /**
   * Searches memes by title or tags using case-insensitive substring matching.
   * @param query - Search term entered by user in the seach bar
   */
  async searchMemes(query: string): Promise<Meme[]> {
    return await invoke<Meme[]>('search_memes', { query });
  },

  /**
   * Fetches metadada for a specific meme by it's unique ID
   */
  async getMeme(id: string): Promise<Meme> {
    return await invoke<Meme>('get_meme', { id });
  },

  /**
   * Resolves the absolute filesystem path for a meme's media file
   */
  async getMemePath(id: string): Promise<string> {
    return await invoke<string>('meme_path', { id });
  },


  /**
  * Saves raw binary image bytes into storage (used for clipboard paste events)
  * Automatically computes SHA-256 hash to prevent dupes
  *
  * @param bytes - Raw byte array of the image (Uint8Array or number[]).
  * @param extension - File format extension ('png', 'jpg', 'webp')
  * @param name - Optional display title for the meme.
  * @param tags - Optional array of categorization tags.
  */
  async saveMemeBytes(
    bytes: Uint8Array | number[],
    extension: string,
    name?: string,
    tags?: string[]
  ): Promise<Meme> {
    return await invoke<Meme>('save_meme_bytes', {
      bytes: Array.from(bytes),
      extension,
      name: name ?? null,
      tags: tags ?? null,
    })
  },

  /**
   * Delete a meme, removing its media file, sidecar JSON, hotkey bindings
   * @param id - UUID of the meme to be deleted
   */
  async deleteMeme(id: string): Promise<void> {
    await invoke('delete_meme', { id });
  },

  /**
   * Toggles favourite status of a meme
   */
  async setFavorite(id: string, favorite: boolean): Promise<Meme> {
    return await invoke<Meme>('set_favorite', { id, favorite });
  },

  /**
   * Increment usage counter and update last_used_at timestamp
   */
  async recordUse(id: string): Promise<Meme> {
    return await invoke<Meme>('record_use', { id });
  },

  /**
   * Retrieves overall library statistics (total meme count, favourites, storage size in bytes)
   */
  async getLibraryStats(): Promise<LibraryStats> {
    return await invoke<LibraryStats>('library_stats');
  },

  /**
   * Decodes an image file and sets it onto clipboard
   * @param filePath - Absolute path to the file
   */
  async copyToClipboard(filePath: string): Promise<void> {
    await invoke("copy_to_clipboard", { filePath });
  },

  /**
   * High-level launcher action stuff:
   * 1. Copy the meme to OS clipboard
   * 2. Records the meme usage count in storage
   * 3. Dismisses (hides) the launcher window immediately
   * @param meme - The meme object selected by user
   */
  async copyAndDismiss(meme: Meme): Promise<void> {
    //1. Resolve physical path and copy to clipboard
    const path = await this.getMemePath(meme.id);
    await this.copyToClipboard(path);

    try {
      await this.recordUse(meme.id);
    } catch { //don't block execution because tracking failed
    }

    //3. Hide launcher window
    const window = getCurrentWebviewWindow();
    await window.hide();
  },

  /**
   * Dismisses the launcher overlay without copying anything.
   */
  async dismiss(): Promise<void> {
    const window = getCurrentWebviewWindow();
    await window.hide();
  },

  async pasteFromClipboard(): Promise<Meme> {
    return await invoke<Meme>('paste_from_clipboard');
  },

  /**
     * Updates a meme's name and/or tags in storage.
     */
  async updateMeme(id: string, name?: string, tags?: string[]): Promise<Meme> {
    return await invoke<Meme>('update_meme', {
      id,
      name: name ?? null,
      tags: tags ?? null,
    });
  },
};
