<script lang="ts">
	import type { PageData } from './$types.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import AddressForm from './AddressForm.svelte';
	export let data: PageData;
</script>

<div class="space-y-6">
	<div>
		<h3 class="text-lg font-medium">Endereco</h3>
		<p class="text-muted-foreground text-sm">
			Configure o endereco do beneficiario que sera utilizado para a geracao de invoices.
		</p>
	</div>
	<Separator />
	<AddressForm data={data.form} />
</div>
