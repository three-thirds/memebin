<script lang="ts">
  import "../app.css";
    let { children } = $props();
    import Navbar from "$lib/components/navbar.svelte";
    import Footer from "$lib/components/footer.svelte";
</script>

<div class="min-h-screen flex flex-col bg-background">
    <!-- <Navbar /> -->
    <main class="flex-1 bg-background p-3">
        {@render children()}
    </main>
    <Footer />
</div>