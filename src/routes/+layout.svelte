<script lang="ts">
  import "../app.css";
  import Footer from "$lib/components/footer.svelte";

  let { children } = $props();
</script>

<div
  class="h-screen w-screen flex flex-col overflow-hidden bg-background text-foreground select-none"
>
  <!-- Pinned Content Area (Search + Scrollable Cards) -->
  <main class="flex-1 min-h-0 flex flex-col overflow-hidden p-3 pb-0">
    {@render children()}
  </main>

  <!-- Pinned Sticky Footer -->
  <div class="shrink-0 border-t border-border/40">
    <Footer />
  </div>
</div>
