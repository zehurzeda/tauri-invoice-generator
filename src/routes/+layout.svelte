<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import LightSwitch from '@/components/LightSwitch.svelte';
	let { children } = $props();
</script>

<LightSwitch />
<ModeWatcher />
{@render children()}
