<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import { shortcuts, formatShortcut, capture, reset} from "$lib/shortcuts.svelte";
    import {cn} from "$lib/utils.js";
    import {Switch } from "$lib/components/ui/switch/index.js";
    import { IconMoon, IconMoonFilled } from '@tabler/icons-svelte-runes';
    import { IconSun, IconSunFilled } from '@tabler/icons-svelte-runes';
    import {mode, setMode} from "mode-watcher";

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
<Dialog.Content class="sm:max-w-lg">
<Dialog.Header>
<Dialog.Title>Settings</Dialog.Title>
<Dialog.Description>
    Manage Preferences etc.
</Dialog.Description>
</Dialog.Header>
<!-- Dialog Content acual stuff lmao -->

<section class="flex flex-col gap-3">
    <h3 class="text-sx font-semibold tracking-wider text-muted-foreground uppercase">
        Shortcuts
    </h3>

    <!-- Open Menu Settings -->
    <div class="flex items-center justify-between gap-4 rounded-lg border bg-card/50 p-3">
        <div class="min-w-0">
            <p class="text-sm font-medium leading-none">Open Settings</p>
            <p class="mt-1.5 text-xs text-muted-foreground">
                Click the box, then press a new combination that includes at least one mod key :p
            </p>
        </div>

        <div class="flex shrink-0 items-center gap-2">
            <button type="button"
                onclick={() => (shortcuts.recording = true)}
                onkeydown={(e) => shortcuts.recording && capture(e)}
                onblur={() => (shortcuts.recording = false)}
                class="col-span-3 flex items-center gap-1.5 rounded-md border p-1 text-sm font-medium hover:cursor-pointer text-foreground transition-colors hover:bg-accent hover:text-accent-foreground focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            >
                {#if shortcuts.recording}
                    <span>Press Keys ... <span class="text-xs">(Esc to cancel)</span></span>
                {:else}
                    {#each formatShortcut(shortcuts.settings) as key}
                        <kbd>{key}</kbd>
                    {/each}
                {/if}
            </button>

            <button onclick={() => reset()}
            class={cn(buttonVariants({ variant: "outline" }), "rounded-md")}>
                Reset
            </button>
        </div>
    </div>


    <!-- Dark / Light mode toggle -->
    <div class="flex items-center justify-between gap-4 rounded-lg border bg-card/50 p-3">
        <div class="min-w-0">
            <p class="text-sm font-medium leading-none">Color Mode</p>
            <p class="mt-1.5 text-xs text-muted-foreground">
                Dark Mode / Light Mode
            </p>
        </div>

        <div class="flex items-center gap-2">
            {#if mode.current === "light"}
                <IconSunFilled size={16} class="text-muted-foreground" />
            {:else}
                <IconSun size={16} class="text-muted-foreground" />
            {/if}

            <Switch 
            checked={mode.current === "dark"}
            onCheckedChange={(checked: boolean) => setMode(checked ? "dark" : "light")}/>
            {#if mode.current === "dark"}
                <IconMoonFilled size={16} class="text-muted-foreground" />
            {:else}
                <IconMoon size={16} class="text-muted-foreground" />
            {/if}
        </div>

    </div>

</section>
    


<Dialog.Footer class="sm:justify-start">
    <Dialog.Close class={buttonVariants({ variant: "secondary"})}>
        Close
    </Dialog.Close>
</Dialog.Footer>    
</Dialog.Content>
</Dialog.Root>