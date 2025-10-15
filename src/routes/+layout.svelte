<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import LightSwitch from '@/components/LightSwitch.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import AppSidebar from '@/components/app-sidebar.svelte';
	let { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider>
	<aside class="h-screen sticky top-0">
		<AppSidebar />
	</aside>
	<Sidebar.Inset>
		<!-- <header class="flex bg-background sticky top-0 shrink-0 items-center gap-2 border-b p-2"> -->
		<!-- 	<div class="flex items-center gap-2"> -->
		<!-- 		<Sidebar.Trigger class="md:hidden -ml-1" /> -->
		<!-- 	</div> -->
		<!-- 	<div class="ml-auto"> -->
		<!-- 		<LightSwitch /> -->
		<!-- 	</div> -->
		<!-- </header> -->
		<div class="h-full w-full p-10 py-4">
			{@render children()}
		</div>
	</Sidebar.Inset>
</Sidebar.Provider>
