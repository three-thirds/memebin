# MemeBin IPC specs

Just a simple documentation for Tauri Commands

## System Commands

### copy_to_clipboard(file_path: string)

Copies the image from `file_path` to OS clipboard

- Invoked by - Svelte Frontend when someone presses Enter on a meme or maybe mouse click, or right click copy.. Lots of options

#### Parameters

- `file_path` : String
  - Absolute Path to the saved image

A side note: TS side can just pass camelCase like `filePath`, Rust will serialize it to snake_case like `file_path`

#### Returns

- `Promise<void>` on Success
- Rejects with a error message string in case of missing file or OS issue

#### Example Usage

```typescript
import { invoke } from "@tauri-apps/api/core";

try {
  await invoke("copy_to_clipboard", { filePath: meme.filePath });
  console.log("Meme copied to clipboard!");
} catch (err) {
  console.error("Failed to copy meme:", err);
}
```
