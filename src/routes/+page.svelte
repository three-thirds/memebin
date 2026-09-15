<script lang="ts">
    import { Input } from "$lib/components/ui/input/index.js";

    export const Example = [
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Hehe"
        },
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Hehe"
        },
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Hehe"
        },
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Hehe"
        },
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Hehe"
        },
        {
            image: "https://cdn.hackclub.com/01a0a4b5-fc23-7376-beb8-64e977a42f13/image.png", 
            name: "Hehe"
        }

    ]
</script>
 
<div class="flex flex-col items-start gap-8 sm:flex-row">
<Input placeholder="Search for anything..." class="rounded-md"/>
</div>

<div class="grid gap-4 pt-8">
{#each Example as example}
<div class="flex flex-col gap-2 rounded-lg border p-4">
    <img src={example.image} alt={example.name} class="text-sm font-medium leading-none w-24 h-24 object-cover" />
    <!-- <p class="text-sm text-muted-foreground">{example.image}</p> -->
</div>
{/each}
</div>
