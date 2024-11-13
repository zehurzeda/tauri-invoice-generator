<script lang="ts" module>
	import Settings2 from 'lucide-svelte/icons/settings-2';
	import Receipt from 'lucide-svelte/icons/receipt';

	const data = {
		navMain: [
			{
				title: 'Gerar',
				url: '#',
				icon: Receipt,
				route: '/generate'
			},
			{
				title: 'Ajustes',
				url: '#',
				icon: Settings2,
				route: '/settings'
			}
		]
	};
</script>

<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import * as Tooltip from '$lib/components/ui/tooltip/index.js';
	import type { ComponentProps } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	let {
		ref = $bindable(null),
		collapsible = 'icon',
		...restProps
	}: ComponentProps<typeof Sidebar.Root> = $props();

	const sidebar = Sidebar.useSidebar();
</script>

<Sidebar.Root
	bind:ref
	collapsible={sidebar?.isMobile ? 'icon' : 'none'}
	class="overflow-hidden min-h-svh !w-[calc(var(--sidebar-width-icon)_+_1px)] border-r"
	{...restProps}
>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupContent class="px-1.5 md:px-0">
				<Sidebar.Menu>
					{#each data.navMain as item (item.title)}
						<Sidebar.MenuItem>
							{#if sidebar.isMobile}
								<Sidebar.MenuButton
									tooltipContentProps={{
										hidden: false
									}}
									onclick={() => {
										goto(item.route);
										sidebar.setOpenMobile(false);
									}}
									isActive={$page.url.pathname.split('/')[0] === item.route.split('/')[0]}
									class="px-2.5 md:px-2"
								>
									{#snippet tooltipContent()}
										{item.title}
									{/snippet}
									<item.icon />
									<span>{item.title}</span>
								</Sidebar.MenuButton>
							{:else}
								<Tooltip.Root>
									<Tooltip.Trigger>
										<a class="group" href={item.route}>
											<button
												class="group-hover:bg-sidebar-accent data-[active=true]:bg-sidebar-accent p-2 rounded-lg"
												aria-label={`Navigate to ${item.title}`}
												data-active={$page.url.pathname.split('/')[1] == item.route.split('/')[1]}
											>
												<item.icon />
											</button>
											<span class="text-xs">{item.title}</span>
										</a>
									</Tooltip.Trigger>
									<Tooltip.Content
										side="right"
										align="center"
										hidden={sidebar.state !== 'collapsed' || sidebar.isMobile}
									>
										{item.title}
									</Tooltip.Content>
								</Tooltip.Root>
							{/if}
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
