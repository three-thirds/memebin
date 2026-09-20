# memebin search

Ranked search is what the overlay picker calls when someone types. The engine lives in `crates/meme-search` (no Tauri). Storage wraps it and exposes `search_memes`, `search_ranked`, `list_tags`, and `suggest_tags`. Paths, paste, import, and delete stay in [storage.md](storage.md).

Search does not own picker chrome or OS hotkeys. System integration wires those and invokes these commands.

Inside the crate the code is split into `parse`, `normalize`, `matcher`, `boost`, `engine`, `synonyms`, and `pipeline`. You do not need that layout to call search from the app — only the query language and commands below.

## Query language

Whitespace-separated tokens are AND: every text group must match the meme’s name, an alias, or a tag (case-insensitive).

OR groups: `cat OR dog` or `cat | dog` (also `(cat | dog)`) form one group where any alternative may match. Mix freely with AND, e.g. `(cat | dog) meme` means (cat or dog) and meme.

Quoted phrases stay one token. `"funny cat"` must match as that phrase against name/aliases/tags the same way a normal token does (substring / word / fuzzy rules below).

Negation: prefix with `-`. `-dog` drops memes that match `dog` strongly enough (substring or better). `-tag:nsfw` or `-#nsfw` excludes that tag. Negating `ext:` or `fav:` is ignored.

Filters you can mix into the same string:

- `tag:foo` or `#foo` — require a tag containing `foo`
- `ext:gif` — require that extension (leading dot optional)
- `fav:1`, `fav:true`, `fav:yes`, or `is:fav` / `is:favorite` / `is:favourite` — favorites only

An empty query, or filters with no text tokens, is browse mode: results ordered by usage, recency, and favorites, then capped by `limit` (default 50). It is not an unsorted dump of the whole library.

## Aliases and synonyms

Each meme may store `aliases` (short names). Binding triggers are also folded into aliases at search time so a trigger string ranks like a nickname without a separate resolve call.

Optional local synonyms live in `app_data_dir()/search_synonyms.json`, shaped like `{ "lol": ["lul", "lmao"] }`. Missing or invalid files are ignored. Expansions become extra OR alternatives inside each text group (no network).

## How scoring works

For each text alternative the engine grades the name, each alias, and each tag, then keeps the best weighted score. Name weighs slightly more than alias, alias slightly more than tag. Grades in order of strength: exact field, whole word (including CamelCase splits like `FunnyCat` → `funny` / `cat`), prefix, substring, then fuzzy (Jaro–Winkler / normalized Levenshtein; fuzzy kicks in around ~0.72–0.88 similarity and scores in a lower band).

Within an OR group the best alternative wins; group scores are summed (AND across groups). Soft boosts add a bit for favorites, log-scaled `use_count`, and recent `last_used_at`. If every group has at least one alternative that appears in the name, there is a small name-coverage bonus.

Browse mode (no text tokens) ranks mainly on popularity, recency, and favorite status.

Default sort is relevance (higher score first). Ties break on `use_count`, then `last_used_at`, then name.

## Index and metrics

Text queries first consult an in-memory inverted index (name / alias / tag tokens → meme ids), then score only those candidates. Browse / filter-only queries still scan the library. Storage keeps the index in process memory and rebuilds it after save, delete, update, favorite changes, binding changes, repair, and manifest import (`record_use` does not rebuild).

Every `search_ranked` response includes `metrics`: `librarySize`, `candidates`, `scanned`, `matched`, `returned`, `ms`, and `usedIndex`. Use these to spot slow keystrokes as the library grows. Cooperative cancel for overlapping async searches is not wired yet (search commands are still synchronous).

## Options and hit shape

`SearchOptions` may include `limit` (default 50), `offset` for pagination, `min_score` to drop weak text hits, and `sort`: `relevance` (default), `recent`, or `name`. `min_score` is not applied in browse mode. `sort` reorders after filtering; relevance still uses the score tie-breakers above when scores are equal under other sorts.

`search_ranked` returns `{ hits, metrics }`. Each hit is `{ meme, score, reasons, highlights }`. `reasons` are short strings from the engine (for example `name:exact`, `alias:prefix`, `tag:prefix`, `filter:tag:lol`, `boost:favorite`). `highlights` are `{ field, start, end }` character ranges (`field` is `name`, `alias`, or `tag:<value>`). Older clients that expected a bare hit array should read `hits`.

## Commands

`search_memes` takes `query` and returns ranked `Meme` objects only (no scores / reasons / highlights / metrics).

`search_ranked` takes `query` and optional `opts` (`limit` / `offset` / `min_score` / `sort`) and returns `{ hits: [{ meme, score, reasons, highlights }], metrics }`.

`list_tags` returns every unique library tag, sorted, for static chips.

`suggest_tags` takes `prefix` and optional `limit` (default 20) and returns tags that prefix- or fuzzy-match that prefix — use this for live chip autocomplete as the user types a tag filter.

Failures are string errors. Frontend args are usually camelCase (`opts`).

## Example

```ts
import { invoke } from "@tauri-apps/api/core";

const ranked = await invoke("search_ranked", {
  query: '(cat | dog) "funny" #animals -sad fav:1',
  opts: { limit: 20, offset: 0, sort: "relevance" },
});
// ranked.hits[0] => { meme, score, reasons, highlights }
// ranked.metrics => { librarySize, candidates, scanned, matched, returned, ms, usedIndex }

const simple = await invoke("search_memes", { query: "cat is:fav ext:gif" });
const tags = await invoke("list_tags");
const chipHints = await invoke("suggest_tags", { prefix: "an", limit: 10 });

await invoke("update_meme", {
  id: memeId,
  aliases: ["db", "boyfriend"],
});
```

## Picker flow

On each keystroke, call `search_ranked` (or `search_memes` if you do not need scores/reasons/highlights). Use `ranked.hits` for the list and optionally show `metrics.ms` / `metrics.candidates` while tuning. Optionally surface `reasons` for “matched on name/tag” and use `highlights` to underline. For tag chips, call `suggest_tags` as the filter field changes, or `list_tags` for the full set. On select, call `meme_path` from storage for the absolute file path, paste via system integration, then `record_use`.

Import, delete, bindings, and disk layout: [storage.md](storage.md).
