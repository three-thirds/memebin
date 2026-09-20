<script lang="ts">
    import { IconStar, IconStarFilled} from '@tabler/icons-svelte-runes';

    let starred = $state(false)
</script>

<button
    type="button"
    onclick={() => {starred = !starred}}>
        {#if starred}
            <IconStarFilled stroke={2} size={16.7}></IconStarFilled>
        {:else}
            <IconStar stroke={2} size={16.7}></IconStar>
        {/if}
</button>