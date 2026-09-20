<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import { shortcuts, formatShortcut, capture, reset} from "$lib/shortcuts.svelte";

    let { open= $bindable(false)} = $props();
</script>

<Dialog.Root bind:open>
<Dialog.Trigger>
    <button 
    class="flex items-center gap-1.5 rounded-full border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground hover:bg-secondary cursor-pointer"
    onclick={() => {}}>
        <span>Actions</span>
        {#each formatShortcut(shortcuts.settings) as key}
            <kbd>{key}</kbd>
        {/each}
    </button>
</Dialog.Trigger>
<Dialog.Content>
<Dialog.Header>
<Dialog.Title>Settings</Dialog.Title>
<Dialog.Description>
    User Settings
</Dialog.Description>
</Dialog.Header>
<!-- Dialog Content acual stuff lmao -->

<button type="button"
    onclick={() => (shortcuts.recording = true)}
    onkeydown={(e) => shortcuts.recording && capture(e)}
    onblur={() => (shortcuts.recording = false)}
    class="flex min-w-0 "
>
    {#if shortcuts.recording}
        <span>hi</span>
    {:else}
        {#each formatShortcut(shortcuts.settings) as key}
            <kbd>{key}</kbd>
        {/each}
    {/if}
</button>

<button onclick={() => reset()}>
    Reset
</button>
    


<Dialog.Footer class="sm:justify-start">
    <Dialog.Close class={buttonVariants({ variant: "secondary"})}>
        Close
    </Dialog.Close>
</Dialog.Footer>    
</Dialog.Content>
</Dialog.Root>