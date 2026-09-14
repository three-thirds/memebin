<script lang="ts">
    export const NavButtons = [
        {href: "https://google.com", text: "Upload"},
        {href: "google.com", text: "Upload"},
        {href: "google.com", text: "Upload"}
    ]
</script>

{#each NavButtons as button}
    <a href={button.href} class="text-white">{button.text}</a>
{/each}