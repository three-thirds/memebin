# memebin storage

This is the local GIF library for memebin. If you are wiring the overlay picker, paste path, or sync, you should be able to do it from this page without reading Rust.

Storage owns files on disk, meme metadata, bindings, and the Tauri commands that talk to them. System integration owns the global hotkey, picker chrome, and clipboard/paste. Backend owns remote sync and accounts later, using the manifest shape described below. Storage never registers OS hotkeys and never touches the clipboard.

For ranked search (`search_memes`, `search_ranked`, `list_tags`, `suggest_tags`), see [search.md](search.md). Note: `search_ranked` returns `{ hits, metrics }` (not a bare array).

## Where files live

Everything sits under Tauri’s `app_data_dir()`, not in the git repo. There is a `memes/` folder of media files plus JSON sidecars, a top-level `bindings.json` that maps triggers to meme ids, and an optional `search_synonyms.json` for local search expansions (see [search.md](search.md)).

On Windows that root is typically under `%APPDATA%\<app identifier>\`. On macOS it is under `~/Library/Application Support/<identifier>/`. On Linux it is under `~/.local/share/<identifier>/`. The identifier comes from `tauri.conf.json` (for example `com.threethirds.tauri-app`).

Call `ensure_storage` at startup. It creates the memes directory if needed and returns that absolute path as a string.

Each meme is stored as `<id>.<ext>` next to `<id>.json`. The JSON is the source of truth for metadata.

## What a meme looks like

A meme has an `id` (UUID), a display `name`, a `filename` like `<id>.gif`, and an `extension`. It can have `tags`, optional `aliases` (short names scored by search), a `size_bytes` count, and a `content_hash` (SHA-256 hex of the file bytes, used for dedup). `favorite` is the picker star. `use_count` and optional `last_used_at` (RFC3339) track usage after paste. `created_at` and `updated_at` are also RFC3339 UTC.

Older sidecars that omit newer fields still load. Missing tags or aliases become an empty list, favorite defaults to false, use_count defaults to zero, and so on.

## Bindings

A binding is a small record with its own `id`, a `trigger` string, and a `meme_id`. Triggers are normalized (trim and lowercase), so `LUL` and `lul` are the same. The meme must already exist when you set a binding. Storage only maps trigger → meme; OS hotkey registration is not storage’s job.

## Allowed media

Saves accept `gif`, `webp`, `png`, `jpg`, and `jpeg` (case-insensitive). Anything else is rejected with a clear error string.

## Dedup and repair

When you save from a path or from raw bytes, storage hashes the file. If that hash already exists in the library, you get the existing meme back and no second copy is written.

`repair_orphans` cleans broken pairs: JSON with no media file, or media with no JSON. It returns how many of each it removed (`removed_json`, `removed_media`).

## Library commands

Failures come back as string errors. Tauri usually camelCases arguments from the frontend (`sourcePath`, `memeId`).

`ensure_storage` takes nothing and returns the memes directory path.

`save_meme` takes a `source_path` plus optional `name` and `tags`, copies the file in, writes a sidecar, and returns the `Meme`. `save_meme_bytes` does the same from a byte array and an `extension` (useful for drag-and-drop without a temp path).

`list_memes` returns everything sorted by created time descending. `list_memes_sorted` takes a `sort` of `created`, `recent`, `favorites`, or `name`.

`get_meme` loads one sidecar by `id`. `meme_path` returns the absolute filesystem path to the media file — that is the paste handoff. `delete_meme` removes media and JSON and also drops any bindings pointing at that id.

`update_meme` patches optional `name`, `tags`, and/or `aliases` and bumps `updated_at`. `set_favorite` sets the star. `record_use` increments `use_count` and sets `last_used_at`; call it after a successful paste.

`library_stats` returns `count`, `favorites`, and `total_bytes`. `repair_orphans` is described above.

For bindings: `list_bindings`, `set_binding(trigger, meme_id)`, `remove_binding(trigger)` (returns whether something was removed), `resolve_trigger` (optional meme), and `resolve_trigger_path` (optional absolute path in one shot).

For sync handoff with no HTTP in storage: `export_manifest` returns version `1`, an `exported_at` timestamp, and meme metadata only (no binary blobs). `import_manifest` takes that object and mode `merge`. Merge skips entries whose id or content hash already exists, and only writes sidecars when the media file is already on disk — the backend must supply files separately. It returns how many entries were imported.

Search commands live in [search.md](search.md).

## Typical flows

Import then paste: call `ensure_storage`, save with `save_meme` or `save_meme_bytes`, let the picker browse or search, take `meme_path` for the selected id so system integration can paste, then `record_use` when paste succeeds.

Hotkey then paste: system integration registers the OS hotkey. On fire, call `resolve_trigger_path`. If you get a path, paste it, then `record_use` when appropriate.

Favorites and recent: `set_favorite`, then `list_memes_sorted` with `favorites` or `recent`.

Backend sync later: one device `export_manifest`, backend stores metadata and moves binaries out of band, another device places media under `memes/` with the expected filenames and calls `import_manifest` with `merge`.

## Example invoke

```ts
import { invoke } from "@tauri-apps/api/core";

await invoke("ensure_storage");

const meme = await invoke("save_meme", {
  sourcePath: "C:/tmp/cat.gif",
  name: "Funny cat",
  tags: ["cat", "lol"],
});

const path = await invoke("meme_path", { id: meme.id });
await invoke("record_use", { id: meme.id });

await invoke("set_binding", { trigger: "lul", memeId: meme.id });
const pastePath = await invoke("resolve_trigger_path", { trigger: "lul" });
```

## What storage does not do

No global hotkey registration, tray UI, clipboard, or paste simulation. No HTTP, auth, or cloud client. No SQLite for now — JSON sidecars are enough.

Work for this area lands on the storage/search branches; rebase on `main` before you push.
