<script lang="ts">
  import "../app.css";
    let { children } = $props();
    import Navbar from "$lib/components/navbar.svelte";
    import Footer from "$lib/components/footer.svelte";
</script>

<Navbar />
{@render children()}
<Footer />