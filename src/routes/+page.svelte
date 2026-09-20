<script lang="ts">
  import { Input } from "$lib/components/ui/input/index.js";
  import { IconTag } from "@tabler/icons-svelte-runes";
  import Starred from "$lib/components/starred.svelte";
  import ChangeMenu from "$lib/components/dialogs/changemenu.svelte";
  import Image from "$lib/components/dialogs/image.svelte";
  import { api, type Meme } from "$lib/api";
  import { onMount } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

  let storageDir = $state("");
  let memes = $state<Meme[]>([]);
  let searchQuery = $state("");
  let changeopen = $state(false);
  let selectedIndex = $state(0);
  let selectedMeme = $state<Meme | null>(null);
  let searchInput = $state<HTMLInputElement | null>(null);
  let cardElements = $state<HTMLElement[]>([]);

  // 2. Automatically scroll whenever selectedIndex changes!
  $effect(() => {
    const el = cardElements[selectedIndex];
    if (el) {
      el.scrollIntoView({
        block: "nearest",
        inline: "nearest",
        behavior: "auto",
      });
    }
  });
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
      if (changeopen) return;

      if (e.key === "Escape") {
        await api.dismiss();
      } else if (e.key === "ArrowRight") {
        e.preventDefault();
        if (memes.length > 0) {
          selectedIndex = Math.min(selectedIndex + 1, memes.length - 1);
        }
      } else if (e.key === "ArrowLeft") {
        e.preventDefault();
        if (memes.length > 0) {
          selectedIndex = Math.max(selectedIndex - 1, 0);
        }
      } else if (e.key === "ArrowDown") {
        e.preventDefault();
        if (memes.length > 0) {
          selectedIndex = Math.min(selectedIndex + 2, memes.length - 1);
        }
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        if (memes.length > 0) {
          selectedIndex = Math.max(selectedIndex - 2, 0);
        }
      } else if (e.key === "Enter") {
        if (memes.length > 0 && memes[selectedIndex]) {
          e.preventDefault();
          await handleCopy(memes[selectedIndex]);
        }
      } else if (e.key === "/" && document.activeElement !== searchInput) {
        e.preventDefault();
        searchInput?.focus();
        searchInput?.select();
      }
    };

    const handlePaste = async (e: ClipboardEvent) => {
      e.preventDefault();
      try {
        await api.pasteFromClipboard();
        memes = await api.listMemes();
      } catch (err) {
        console.error("Native paste failed:", err);
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
    selectedIndex = 0;
    const query = (e.target as HTMLInputElement).value;
    searchQuery = query;

    if (query.trim() === "") {
      memes = await api.listMemes();
    } else {
      memes = await api.searchMemes(query);
    }
  }

  async function handleCopy(meme: Meme) {
    getCurrentWebviewWindow().hide();
    try {
      await api.copyAndDismiss(meme);
    } catch (err) {
      console.error("Failed to copy meme:", err);
    }
  }

  function handleRightClick(e: MouseEvent, meme: Meme) {
    e.preventDefault();
    selectedMeme = meme;
    changeopen = true;
  }
</script>

<svelte:window oncontextmenu={(e) => e.preventDefault()} />

<div class="flex flex-col h-full w-full max-h-full overflow-hidden flex-1">
  <!-- Pinned Search Header -->
  <div class="shrink-0 pb-3">
    <Input
      bind:ref={searchInput}
      placeholder="Search for anything... (Press / to focus)"
      class="rounded-md"
      value={searchQuery}
      oninput={handleSearch}
    />
  </div>

  <!-- Scrollable Grid Body (ONLY this scrolls) -->
  <div class="flex-1 overflow-y-auto min-h-0 p-1.5">
    <div class="grid grid-cols-2 gap-4 pb-2">
      {#each memes as meme, index (meme.id)}
        <div
          bind:this={cardElements[index]}
          role="button"
          tabindex="0"
          class="flex flex-row gap-2 rounded-lg border p-4 border-border transition-all min-w-0 cursor-pointer {index ===
          selectedIndex
            ? 'ring-2 ring-inset ring-primary bg-accent/80'
            : 'bg-background/50 hover:bg-background'}"
          onclick={() => handleCopy(meme)}
          oncontextmenu={(e) => handleRightClick(e, meme)}
          onkeydown={(e) => {
            if (e.key === "Enter" || e.key === " ") handleCopy(meme);
          }}
        >
          <Image
            image={convertFileSrc(`${storageDir}/${meme.filename}`)}
            name={meme.name}
            tag={meme.tags[0] ?? "Meme"}
          />

          <!-- svelte-ignore a11y_click_events_have_key_events -->
          <div
            role="button"
            tabindex="0"
            aria-haspopup="menu"
            onclick={(e) => e.stopPropagation()}
            class="flex min-w-0 flex-col gap-1 items-start justify-between border-l border-dashed p-2"
          >
            <div class="flex flex-col gap-1.5 w-full">
              <h3
                class="text-sm font-medium leading-snug truncate max-w-32 pb-0.5"
              >
                {meme.name}
              </h3>
              <div
                class="flex items-center justify-between text-xs text-muted-foreground"
              >
                <span><b>Uses:</b> {meme.use_count}</span>
                <Starred />
              </div>
            </div>
            {#if meme.tags.length > 0}
              <p
                class="flex max-w-full min-w-0 items-center gap-1.5 overflow-hidden rounded-2xl border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground hover:bg-secondary cursor-pointer"
              >
                <IconTag stroke={2} size={16.7} class="shrink-0" />
                <kbd class="max-w-16 truncate">{meme.tags[0]}</kbd>
              </p>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<!-- Modal Root (Mounted once, clean background) -->
{#if selectedMeme}
  <ChangeMenu
    bind:open={changeopen}
    id={selectedMeme.id}
    image={convertFileSrc(`${storageDir}/${selectedMeme.filename}`)}
    name={selectedMeme.name}
    tag={selectedMeme.tags[0] ?? "Meme"}
    onchange={async () => {
      memes = await api.listMemes();
    }}
  />
{/if}
