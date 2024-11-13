<script lang="ts">
	import SidebarNav from '@/components/sidebar-nav.svelte';
	import { Separator } from '@/components/ui/separator';

	const settingsItems = [
		{
			title: 'Dados Bancários',
			href: '/settings'
		},
		{
			title: 'Endereço',
			href: '/settings/address'
		},
		{
			title: 'Sistema',
			href: '/settings/system'
		}
	];
	let { children } = $props();
</script>

<div class="space-y-6 pb-16 md:block">
	<div class="space-y-0.5">
		<h2 class="text-2xl font-bold tracking-tight">Ajustes</h2>
		<p class="text-muted-foreground">
			Gerencie dados bancários, endereços e configurações gerais do sistema.
		</p>
	</div>
	<Separator class="my-6" />
	<div class="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0">
		<aside class="-mx-4 lg:w-1/5">
			<SidebarNav items={settingsItems} />
		</aside>
		<div class="flex-1 lg:max-w-2xl">
			{@render children()}
		</div>
	</div>
</div>
