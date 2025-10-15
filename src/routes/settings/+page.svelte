<script lang="ts">
	import type { PageData } from './$types.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import BankForm from './BankForm.svelte';
	export let data: PageData;
</script>

<div class="space-y-6">
	<div>
		<h3 class="text-lg font-medium">Dados Bancarios</h3>
		<p class="text-muted-foreground text-sm">
			Configure os dados bancarios do beneficiario que serao usados para a geracao de invoices.
		</p>
	</div>
	<Separator />
	<BankForm data={data.form} />
</div>
