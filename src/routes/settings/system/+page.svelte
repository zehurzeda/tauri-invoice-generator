<script lang="ts">
	import type { PageData } from './$types.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import SystemForm from './SystemForm.svelte';
	export let data: PageData;
</script>

<div class="space-y-6">
	<div>
		<h3 class="text-lg font-medium">Sistema</h3>
		<p class="text-muted-foreground text-sm">Configuracoes gerais do sistema</p>
	</div>
	<Separator />
	<SystemForm data={data.form} />
</div>
