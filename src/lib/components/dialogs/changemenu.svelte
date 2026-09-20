<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import {buttonVariants } from "$lib/components/ui/button/index.js";
    import { IconTag } from '@tabler/icons-svelte-runes';
    
    let {open= $bindable(false), image, name, tag} = $props();
</script>

<Dialog.Root bind:open>
    <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content class="flex flex-col gap-4 p-6 px-6 pb-8">
            <Dialog.Header>
                <Dialog.Title>Edit</Dialog.Title>
                <Dialog.Description>
                    {name}
                </Dialog.Description>
            </Dialog.Header>
            <img src={image} alt={name} class="text-sm font-medium leading-none w-48 h-48 object-cover rounded-md" />
            <p 
            class="flex max-w-full min-w-0 items-center gap-1.5 overflow-hidden rounded-2xl border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground hover:bg-secondary cursor-pointer">
                <IconTag stroke={2} size={16.7} class="shrink-0"/>
                <kbd class="min-w-0 truncate">{tag}</kbd>
            </p>
            <input placeholder="Name" class="rounded-md border-2 p-2 min-w-0"/>
            <input placeholder="Description" class="rounded-md border-2 p-2 min-w-0"/>
            <input placeholder="Edit Tag" class="rounded-md border-2 p-2 min-w-0"/>
            <Dialog.Footer class="sm:justify-start">
                <Dialog.Close class={buttonVariants({ variant: "secondary" })}>
                    Close
                </Dialog.Close>
            </Dialog.Footer>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>