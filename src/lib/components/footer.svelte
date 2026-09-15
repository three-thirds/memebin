<script lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
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

    <Dialog.Root>
    <Dialog.Trigger>
        <button 
        class="flex items-center gap-1.5 rounded-full border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground hover:bg-secondary cursor-pointer"
        onclick={() => {}}>
            <span>Actions</span>
            <kbd>Ctrl</kbd>
            <kbd>B</kbd>
        </button>
    </Dialog.Trigger>
    <Dialog.Content>
    <Dialog.Header>
    <Dialog.Title>Settings</Dialog.Title>
    <Dialog.Description>
        User Settings & Keybinds
    </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer class="sm:justify-start">
        <Dialog.Close class={buttonVariants({ variant: "secondary"})}>
            Close
        </Dialog.Close>
    </Dialog.Footer>
    <!-- Dialog Content acual stuff lmao -->
     
    </Dialog.Content>
    </Dialog.Root>
</footer>

