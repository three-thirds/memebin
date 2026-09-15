<script lang="ts">
    import { Input } from "$lib/components/ui/input/index.js";

    export const Example = [
        {
            image: "Hello there", 
            name: "Hehe"
        },
        {
            image: "Hello there", 
            name: "Hehe"
        },
        {
            image: "Hello there", 
            name: "Hehe"
        },
        {
            image: "Hello there", 
            name: "Hehe"
        },
        {
            image: "Hello there", 
            name: "Hehe"
        },
        {
            image: "Hello there", 
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
    <p class="text-sm font-medium leading-none">{example.name}</p>
    <p class="text-sm text-muted-foreground">{example.image}</p>
</div>
{/each}
</div>
