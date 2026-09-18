# memebin storage

Local GIF/media library for memebin. This is the contract for **system integration** and **backend** — you should not need to read the Rust sources to wire against storage.

## Ownership

| Role | Owns |
|------|------|
| **Storage** | Files on disk, metadata, library / bindings / manifest Tauri commands |
| **System integration** | Global hotkey registration, picker UI chrome, clipboard / paste into the focused app |
| **Backend** | Remote sync, accounts, packs — use the manifest shape below (no HTTP from storage) |

Storage never registers OS hotkeys and never touches the clipboard.

## On-disk layout

All data lives under Tauri’s `app_data_dir()` (not the git repo):

```text
<app_data_dir>/
  memes/
    <id>.<ext>     # media
    <id>.json      # Meme sidecar
  bindings.json    # local trigger → meme_id map
```

### Platform examples

Paths include the app identifier from `tauri.conf.json` (e.g. `com.threethirds.tauri-app`):

| OS | Typical root |
|----|----------------|
| Windows | `%APPDATA%\<identifier>\` |
| macOS | `~/Library/Application Support/<identifier>/` |
| Linux | `~/.local/share/<identifier>/` |

Call `ensure_storage` at startup; it creates `memes/` and returns that absolute path.

## Schemas

### Meme

| Field | Type | Notes |
|-------|------|--------|
| `id` | `string` | UUID |
| `name` | `string` | Display name |
| `filename` | `string` | e.g. `<id>.gif` |
| `extension` | `string` | `gif`, `webp`, `png`, `jpg`, `jpeg` |
| `tags` | `string[]` | Searchable |
| `size_bytes` | `number` | File size |
| `content_hash` | `string` | SHA-256 hex of file bytes (dedup key) |
| `favorite` | `boolean` | Picker star |
| `use_count` | `number` | Incremented by `record_use` |
| `last_used_at` | `string \| null` | RFC3339 UTC |
| `created_at` | `string` | RFC3339 UTC |
| `updated_at` | `string` | RFC3339 UTC |

Older sidecars missing newer fields are accepted with defaults (`tags: []`, `favorite: false`, `use_count: 0`, etc.).

### Binding

| Field | Type | Notes |
|-------|------|--------|
| `id` | `string` | UUID |
| `trigger` | `string` | Normalized (trim + lowercase), e.g. `lul` or `ctrl+shift+m` |
| `meme_id` | `string` | Must exist when setting |

### LibraryStats

```ts
{ count: number; favorites: number; total_bytes: number }
```

### RepairReport

```ts
{ removed_json: number; removed_media: number }
```

### Manifest (sync handoff)

```ts
{
  version: number;      // currently 1
  exported_at: string;  // RFC3339
  memes: Meme[];        // metadata only — no binary blobs
}
```

### MemeSort

`"created"` | `"recent"` | `"favorites"` | `"name"`

### ImportMode

`"merge"` — skip entries whose `id` or `content_hash` already exists; only write sidecars when the media file is already on disk (backend must supply files separately).

## Allowed media extensions

`gif`, `webp`, `png`, `jpg`, `jpeg` (case-insensitive). Other extensions are rejected.

## Dedup & repair

- **Dedup:** `save_meme` / `save_meme_bytes` hash file bytes. If that hash already exists, the existing `Meme` is returned (no second copy).
- **repair_orphans:** removes JSON without media and media without JSON; returns counts.

## TypeScript types

```ts
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

export interface Binding {
  id: string;
  trigger: string;
  meme_id: string;
}

export interface LibraryStats {
  count: number;
  favorites: number;
  total_bytes: number;
}

export interface RepairReport {
  removed_json: number;
  removed_media: number;
}

export interface Manifest {
  version: number;
  exported_at: string;
  memes: Meme[];
}

export type MemeSort = "created" | "recent" | "favorites" | "name";
export type ImportMode = "merge";
```

## Tauri commands

All commands return `Result` — failures are `string` error messages.

| Command | Args | Returns |
|---------|------|---------|
| `ensure_storage` | — | `string` (memes dir path) |
| `save_meme` | `source_path: string`, `name?: string`, `tags?: string[]` | `Meme` |
| `save_meme_bytes` | `bytes: number[]`, `extension: string`, `name?: string`, `tags?: string[]` | `Meme` |
| `list_memes` | — | `Meme[]` (created desc) |
| `list_memes_sorted` | `sort: MemeSort` | `Meme[]` |
| `get_meme` | `id: string` | `Meme` |
| `meme_path` | `id: string` | `string` absolute media path |
| `delete_meme` | `id: string` | `void` (also drops bindings to that id) |
| `search_memes` | `query: string` | `Meme[]` (name + tags, case-insensitive) |
| `update_meme` | `id: string`, `name?: string`, `tags?: string[]` | `Meme` |
| `record_use` | `id: string` | `Meme` |
| `set_favorite` | `id: string`, `favorite: boolean` | `Meme` |
| `library_stats` | — | `LibraryStats` |
| `repair_orphans` | — | `RepairReport` |
| `list_bindings` | — | `Binding[]` |
| `set_binding` | `trigger: string`, `meme_id: string` | `Binding` |
| `remove_binding` | `trigger: string` | `boolean` |
| `resolve_trigger` | `trigger: string` | `Meme \| null` |
| `resolve_trigger_path` | `trigger: string` | `string \| null` |
| `export_manifest` | — | `Manifest` |
| `import_manifest` | `manifest: Manifest`, `mode: ImportMode` | `number` (imported count) |

## Invoke examples

```ts
import { invoke } from "@tauri-apps/api/core";
import type { Binding, Manifest, Meme, MemeSort } from "./storage-types"; // or copy types above

await invoke<string>("ensure_storage");

const meme = await invoke<Meme>("save_meme", {
  sourcePath: "C:/tmp/cat.gif",
  name: "Funny cat",
  tags: ["cat", "lol"],
});

const fromBytes = await invoke<Meme>("save_meme_bytes", {
  bytes: Array.from(uint8Array),
  extension: "gif",
  name: "drag-drop",
  tags: [],
});

const all = await invoke<Meme[]>("list_memes");
const recent = await invoke<Meme[]>("list_memes_sorted", { sort: "recent" satisfies MemeSort });
const hits = await invoke<Meme[]>("search_memes", { query: "cat" });

const path = await invoke<string>("meme_path", { id: meme.id });
await invoke<Meme>("record_use", { id: meme.id }); // after successful paste

await invoke<Binding>("set_binding", { trigger: "lul", memeId: meme.id });
const pastePath = await invoke<string | null>("resolve_trigger_path", { trigger: "lul" });

const manifest = await invoke<Manifest>("export_manifest");
const imported = await invoke<number>("import_manifest", {
  manifest,
  mode: "merge",
});
```

Note: Tauri typically camelCases command arguments (`sourcePath`, `memeId`).

## Recommended flows

### Import → picker → paste

1. `ensure_storage`
2. `save_meme` or `save_meme_bytes`
3. Picker: `list_memes` / `list_memes_sorted` / `search_memes`
4. On select: `meme_path(id)` → system integration pastes that file
5. After paste succeeds: `record_use(id)`

### Hotkey → paste

1. System integration registers the OS hotkey (not storage)
2. On fire: `resolve_trigger_path(trigger)` → absolute path or `null`
3. Paste; then `record_use` if resolved

### Favorites / recent

- `set_favorite(id, true|false)`
- `list_memes_sorted` with `"favorites"` or `"recent"`

### Backend sync (later)

1. Client: `export_manifest`
2. Backend stores metadata; transfers binaries out of band
3. Other device: place media files under `memes/` with expected filenames, then `import_manifest(manifest, "merge")`

## Out of scope

- Global hotkey registration, tray, clipboard, paste simulation
- HTTP / auth / cloud clients
- SQLite (JSON sidecars for now)

## Branch workflow

Storage work targets the `meme-storage` branch. Rebase on `main` before pushing:

```bash
git fetch origin
git rebase origin/main
```
