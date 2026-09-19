<script lang="ts">
    import { Input } from "$lib/components/ui/input/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import { IconTag } from '@tabler/icons-svelte-runes';
    import { IconStar,IconStarFilled} from '@tabler/icons-svelte-runes';
    import Starred from "$lib/components/starred.svelte";

    export const Example = [
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Diet Lmao",
            uses: 6,
            tag: "Group Chat"
        },
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Diet Lmao",
            uses: 6,
            tag: "Group Chat"
        },
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Diet Lmao",
            uses: 6,
            tag: "Group Chat"
        },

    ]

    let ChangeMenu = $state(false)

    function handleRightClick(e: MouseEvent) {
        e.preventDefault();
        ChangeMenu = true;
    }

    let starred = $state(false)

</script>

<div class="flex flex-col items-stretch gap-8 sm:flex-row">
<Input placeholder="Search for anything..." class="rounded-md"/>
</div>

<div class="grid grid-cols-2 gap-4 pt-8di">
{#each Example as example}
<div class="flex flex-row gap-2 rounded-lg border p-4 border-border bg-background/50 hover:bg-background transition-colors min-w-0">
    <Dialog.Root>
        <Dialog.Trigger class="inline-block w-fit shrink-0">
            <img src={example.image} alt={example.name} class="w-48 h-full object-cover rounded-md ring-1 ring-border transition-all duration-200 hover:ring-accent-foreground hover:brightness-110 cursor-pointer" />
        </Dialog.Trigger>
        <Dialog.Portal>
            <Dialog.Overlay />
            <Dialog.Content class="flex flex-col gap-4 p-6 px-6 pb-8">
                <Dialog.Header>
                    <Dialog.Title>{example.name}</Dialog.Title>
                    <Dialog.Description>
                        Your Image & Tags
                    </Dialog.Description>
                </Dialog.Header>
                <img src={example.image} alt={example.name} class="text-sm font-medium leading-none w-48 h-48 object-cover rounded-md" />
                <p 
                class="flex max-w-full min-w-0 items-center gap-1.5 overflow-hidden rounded-2xl border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground hover:bg-secondary cursor-pointer">
                    <IconTag stroke={2} size={16.7} class="shrink-0"/>
                    <kbd class="min-w-0 truncate">{example.tag}</kbd>
                </p>
                <Dialog.Footer class="sm:justify-start">
                    <Dialog.Close class={buttonVariants({ variant: "secondary" })}>
                        Close
                    </Dialog.Close>
                </Dialog.Footer>
            </Dialog.Content>
        </Dialog.Portal>
    </Dialog.Root>
    
    <div role="button" tabindex="0" aria-haspopup="menu" oncontextmenu={handleRightClick} class="flex min-w-0 flex-col gap-1 items-start justify-between border-l border-dashed p-2">
        <div>
            <h3 class="text-sm font-medium leading-none">{example.name}</h3>
            <h4 class="text-xs text-muted-foreground"><b>Uses:</b> {example.uses}</h4>
            <Starred />
        </div>
        <p 
        class="flex max-w-full min-w-0 items-center gap-1.5 overflow-hidden rounded-2xl border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground hover:bg-secondary cursor-pointer">
            <IconTag stroke={2} size={16.7} class="shrink-0"/>
            <kbd class="max-w-16 truncate">{example.tag}</kbd>
        </p>
    </div>

    <Dialog.Root bind:open={ChangeMenu}>
        <Dialog.Portal>
            <Dialog.Overlay />
            <Dialog.Content class="flex flex-col gap-4 p-6 px-6 pb-8">
                <Dialog.Header>
                    <Dialog.Title>Edit</Dialog.Title>
                    <Dialog.Description>
                        {example.name}
                    </Dialog.Description>
                </Dialog.Header>
                <img src={example.image} alt={example.name} class="text-sm font-medium leading-none w-48 h-48 object-cover rounded-md" />
                <p 
                class="flex max-w-full min-w-0 items-center gap-1.5 overflow-hidden rounded-2xl border border-border bg-secondary/60 px-2 py-1 text-xs font-medium text-secondary-foreground hover:bg-secondary cursor-pointer">
                    <IconTag stroke={2} size={16.7} class="shrink-0"/>
                    <kbd class="min-w-0 truncate">{example.tag}</kbd>
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
</div>
{/each}
</div>


