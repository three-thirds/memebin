<script lang="ts">
  import "../app.css";
    let { children } = $props();
    import Navbar from "$lib/components/navbar.svelte";
    import Footer from "$lib/components/footer.svelte";
</script>

<div class="relative flex min-h-screen flex-col overflow-hidden rounded-2xl border border-border bg-background/30 backdrop-blur-md">
    <!-- <Navbar /> -->
    <main class="flex-1 bg-background p-3 pb-15">
        {@render children()}
    </main>
    <Footer />
</div>