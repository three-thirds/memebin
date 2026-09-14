<script lang="ts">
  import "../app.css";
    let { children } = $props();
    import Navbar from "$lib/components/navbar.svelte";
</script>

<Navbar />
{@render children()}