<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";

    let { open= $bindable(false)} = $props();
</script>

<Dialog.Root bind:open>
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