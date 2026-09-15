<script lang="ts">
    import { openUrl } from "@tauri-apps/plugin-opener";
</script>

<footer class="flex flex-col items-center gap-2 border-b border-border bg-background px-4 h-14">
<p class="font-bold pt-2">
    Made With ❤️ by 
    <a 
    class="text-muted-foreground hover:text-foreground transition-colors"
    href="https://github.com/three-thirds"
    onclick={(e) => { e.preventDefault(); openUrl("https://github.com/three-thirds"); }}>
    Three Thirds
    </a>
</p>
<p>
    For 
    <a 
    class="text-muted-foreground hover:text-foreground transition-colors"
    href="https://thirdspace.hackclub.com"
    onclick={(e) => { e.preventDefault(); openUrl("https://thirdspace.hackclub.com"); }}>
    Third Space
    </a>
</p>
</footer>