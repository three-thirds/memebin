<script lang="ts">
  import { Input } from "$lib/components/ui/input/index.js";
  import { IconTag } from "@tabler/icons-svelte-runes";
  import Starred from "$lib/components/starred.svelte";

  import ChangeMenu from "$lib/components/dialogs/changemenu.svelte";

  import Image from "$lib/components/dialogs/image.svelte";
  import { api, type Meme } from "$lib/api";
  import { onMount } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { capture } from "$lib/shortcuts.svelte";

  let storageDir = $state("");
  let memes = $state<Meme[]>([]);
  let searchQuery = $state("");
  let changeopen = $state(false);

  onMount(() => {
    async function init() {
      try {
        storageDir = await api.ensureStorage();
        memes = await api.listMemes();
      } catch (err) {
        console.error("Failed to load memes from backend:", err);
      }
    }
    init();

    const handleKeyDown = async (e: KeyboardEvent) => {
      if (e.key == "Escape") {
        await api.dismiss();
      }
    };

    const handlePaste = async (e: ClipboardEvent) => {
      e.preventDefault();
      try {
        console.log("[DEBUG] Triggering native OS clipboard paste...");
        const newMeme = await api.pasteFromClipboard();
        console.log("[DEBUG] Successfully saved native meme:", newMeme.name);

        // Refresh list!
        memes = await api.listMemes();
      } catch (err) {
        console.error("[DEBUG] Native paste failed:", err);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("paste", handlePaste, { capture: true });

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("paste", handlePaste, { capture: true });
    };
  });

  async function handleSearch(e: Event) {
    const query = (e.target as HTMLInputElement).value;
    searchQuery = query;

    if (query.trim() == "") {
      memes = await api.listMemes();
    } else {
      memes = await api.searchMemes(query);
    }
  }

  async function handleCopy(meme: Meme) {
    try {
      await api.copyAndDismiss(meme);
    } catch (err) {
      console.error("Failed to copy meme: ", err);
    }
  }

  function handleRightClick(e: MouseEvent) {
    e.preventDefault();
    changeopen = true;
  }
</script>

<div class="flex flex-col items-stretch gap-8 sm:flex-row">
  <Input
    placeholder="Search for anything..."
    class="rounded-md"
    value={searchQuery}
    oninput={handleSearch}
  />
</div>

<div class="grid grid-cols-2 gap-4 pt-8">
  {#each memes as meme}
    <div
      role="button"
      tabindex="0"
      class="flex flex-row gap-2 rounded-lg border p-4 border-border bg-background/50 hover:bg-background transition-colors min-w-0"
      onclick={() => handleCopy(meme)}
      onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " ") handleCopy(meme);
      }}
    >
      <Image
        image={convertFileSrc(`${storageDir}/${meme.filename}`)}
        name={meme.name}
        tag={meme.tags[0] ?? "Meme"}
      />

      <div
        role="button"
        tabindex="0"
        aria-haspopup="menu"
        oncontextmenu={handleRightClick}
        class="flex min-w-0 flex-col gap-1 items-start justify-between border-l border-dashed p-2"
      >
        <div>
          <h3 class="text-sm font-medium leading-none">{meme.name}</h3>
          <h4 class="text-xs text-muted-foreground">
            <b>Uses:</b>
            {meme.use_count}
          </h4>
          <Starred />
        </div>
        <p
          class="flex max-w-full min-w-0 items-center gap-1.5 overflow-hidden rounded-2xl border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground hover:bg-secondary cursor-pointer"
        >
          <IconTag stroke={2} size={16.7} class="shrink-0" />
          <kbd class="max-w-16 truncate">{meme.tags[0] ?? "Meme"}</kbd>
        </p>
      </div>

      <ChangeMenu
        bind:open={changeopen}
        image={convertFileSrc(`${storageDir}/${meme.filename}`)}
        name={meme.name}
        tag={meme.tags[0] ?? "Meme"}
      />
    </div>
  {/each}
</div>
