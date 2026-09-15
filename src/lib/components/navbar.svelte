<script lang="ts">
    export const NavButtons = [
        {href: "/Upload", text: "Upload"},
        {href: "/Browse", text: "Browse"},
        {href: "https://github.com/three-thirds/memebin/", text: "Github"}
    ]
</script>

<nav class="flex items-center gap-2 border-b border-border bg-background px-4 h-14">
{#each NavButtons as button}
<a href={button.href} class="px-4 py-2 rounded-full text-sm font-medium bg-secondary text-secondary-foreground hover:bg-accent hover:text-accent-foreground transition-colors">
{button.text}
</a>
{/each}
</nav>
