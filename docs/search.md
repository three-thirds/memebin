# memebin search

Ranked search is what the overlay picker calls when someone types. The engine lives in `crates/meme-search` (no Tauri). Storage wraps it and exposes `search_memes`, `search_ranked`, and `list_tags`. Paths, paste, import, and delete stay in [storage.md](storage.md).

Search does not own picker chrome or OS hotkeys. System integration wires those and invokes these commands.

Inside the crate the code is split into `parse`, `normalize`, `matcher`, `boost`, `engine`, and `pipeline`. You do not need that layout to call search from the app — only the query language and commands below.

## Query language

Whitespace-separated tokens are AND: every text token must match the meme’s name or a tag (case-insensitive).

Quoted phrases stay one token. `"funny cat"` must match as that phrase against name/tags the same way a normal token does (substring / word / fuzzy rules below).

Negation: prefix with `-`. `-dog` drops memes that match `dog` strongly enough (substring or better). `-tag:nsfw` or `-#nsfw` excludes that tag. Negating `ext:` or `fav:` is ignored.

Filters you can mix into the same string:

- `tag:foo` or `#foo` — require a tag containing `foo`
- `ext:gif` — require that extension (leading dot optional)
- `fav:1`, `fav:true`, `fav:yes`, or `is:fav` / `is:favorite` / `is:favourite` — favorites only

An empty query, or filters with no text tokens, is browse mode: results ordered by usage, recency, and favorites, then capped by `limit` (default 50). It is not an unsorted dump of the whole library.

## How scoring works

For each text token the engine grades the name and each tag, then keeps the best weighted score. Name matches weigh slightly more than tag matches. Grades in order of strength: exact field, whole word (including CamelCase splits like `FunnyCat` → `funny` / `cat`), prefix, substring, then fuzzy (Jaro–Winkler / normalized Levenshtein; fuzzy kicks in around ~0.72–0.88 similarity and scores in a lower band).

Token scores are summed (AND). Soft boosts add a bit for favorites, log-scaled `use_count`, and recent `last_used_at`. If every token appears in the name, there is a small name-coverage bonus.

Browse mode (no text tokens) ranks mainly on popularity, recency, and favorite status.

Higher score means show first. Ties break on `use_count`, then `last_used_at`, then name.

## Options and hit shape

`SearchOptions` may include `limit` (default 50), `offset` for pagination, and `min_score` to drop weak text hits. `min_score` is not applied in browse mode.

The crate’s internal hit also carries `reasons` strings (like `name:exact` or `filter:tag:lol`) for debugging. The Tauri `search_ranked` wrapper returns `{ meme, score }` only — reasons are stripped at the storage boundary.

## Commands

`search_memes` takes `query` and returns ranked `Meme` objects with scores removed.

`search_ranked` takes `query` and optional `opts` (`limit` / `offset` / `min_score`) and returns `{ meme, score }[]`.

`list_tags` returns unique library tags, sorted, for chips. (The crate also has `suggest_tags` for prefix/fuzzy chip suggestions; that is not exposed as a Tauri command yet.)

Failures are string errors. Frontend args are usually camelCase (`opts`).

## Example

```ts
import { invoke } from "@tauri-apps/api/core";

const ranked = await invoke("search_ranked", {
  query: 'cat "funny dog" #animals -sad fav:1',
  opts: { limit: 20, offset: 0 },
});

const simple = await invoke("search_memes", { query: "cat is:fav ext:gif" });
const tags = await invoke("list_tags");
```

## Picker flow

On each keystroke, call `search_ranked` (or `search_memes` if you do not need scores). Render the list. On select, call `meme_path` from storage for the absolute file path, paste via system integration, then `record_use`. Use `list_tags` for chip filters next to free text.

Import, delete, bindings, and disk layout: [storage.md](storage.md).
