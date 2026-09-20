<script lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";
    import UserSettings from "$lib/components/dialogs/usersettings.svelte";
    import {shortcuts, matches } from "$lib/shortcuts.svelte";


    let settingsOpen = $state(false);

    function scanKeys(e: KeyboardEvent) {
        if (shortcuts.recording) return;
        if (matches(e, shortcuts.settings)) {
            e.preventDefault();
            settingsOpen = true;
        }
    }
</script>

<svelte:window onkeydown={scanKeys}></svelte:window>


<footer class="absolute right-0 bottom-0 left-0 flex items-center justify-between gap-2 border-t bg-background/50 p-2 backdrop-blur-md">
  <p class="text-xs text-muted-foreground">
    Made With ❤️ by
    <a
      class="text-foreground transition-colors hover:text-muted-foreground"
      href="https://github.com/three-thirds"
      onclick={(e) => {
        e.preventDefault();
        openUrl("https://github.com/three-thirds");
      }}
    >
      Three Thirds
    </a>
  </p>

<UserSettings bind:open={settingsOpen} />

</footer>

