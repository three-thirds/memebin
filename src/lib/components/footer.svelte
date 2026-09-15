<script lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";
</script>

<footer class="flex h-14 items-center justify-between gap-3 bg-background px-4">
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

  <button 
  class="flex items-center gap-1.5 rounded-full border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground"
  onclick={() => {}}>
    <span>Actions</span>
    <kbd>Ctrl</kbd>
    <kbd>B</kbd>
  </button>
</footer>