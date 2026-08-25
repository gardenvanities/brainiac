<script lang="ts">
import "../app.css";
import AppShell from "../components/layout/AppShell.svelte";

let { children } = $props();
</script>

<AppShell />
{@render children()}
