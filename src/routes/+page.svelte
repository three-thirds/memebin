<script lang="ts">
    import { Input } from "$lib/components/ui/input/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";

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
<div class="flex flex-row gap-2 rounded-lg border p-4 border-border bg-background/50 hover:bg-background transition-colors">
    <Dialog.Root>
        <Dialog.Trigger class="inline-block w-fit">
            <img src={example.image} alt={example.name} class="tw-24 h-24 object-cover rounded-md ring-1 ring-border transition-all duration-200 hover:ring-2 hover:ring-accent-foreground hover:scale-105 hover:brightness-110 cursor-pointer" />
        </Dialog.Trigger>
        <Dialog.Portal>
            <Dialog.Overlay />
            <Dialog.Content>
                <Dialog.Header>
                    <Dialog.Title>{example.name}</Dialog.Title>
                    <Dialog.Description>
                        Your Image up close :p
                    </Dialog.Description>
                </Dialog.Header>
                <img src={example.image} alt={example.name} class="text-sm font-medium leading-none w-48 h-48 object-cover" />
                <Dialog.Footer class="sm:justify-start">
                    <Dialog.Close class={buttonVariants({ variant: "secondary" })}>
                        Close
                    </Dialog.Close>
                </Dialog.Footer>
            </Dialog.Content>
        </Dialog.Portal>
    </Dialog.Root>
    Hi
</div>
{/each}
</div>
