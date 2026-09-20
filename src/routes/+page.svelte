<script lang="ts">
    import { Input } from "$lib/components/ui/input/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import { IconTag } from '@tabler/icons-svelte-runes';
    import { IconStar,IconStarFilled} from '@tabler/icons-svelte-runes';
    import Starred from "$lib/components/starred.svelte";

    import ChangeMenu from "$lib/components/dialogs/changemenu.svelte";

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

    let changeopen = $state(false)

    function handleRightClick(e: MouseEvent) {
        e.preventDefault();
        changeopen = true;
    }

    let starred = $state(false)

    import Image from "$lib/components/dialogs/image.svelte";

</script>

<div class="flex flex-col items-stretch gap-8 sm:flex-row">
<Input placeholder="Search for anything..." class="rounded-md"/>
</div>

<div class="grid grid-cols-2 gap-4 pt-8">
{#each Example as example}
<div class="flex flex-row gap-2 rounded-lg border p-4 border-border bg-background/50 hover:bg-background transition-colors min-w-0">
    <Image image={example.image} name={example.name} tag={example.tag}/>
    
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

    <ChangeMenu bind:open={changeopen} image={example.image} name={example.name} tag={example.tag}/>
</div>
{/each}
</div>


