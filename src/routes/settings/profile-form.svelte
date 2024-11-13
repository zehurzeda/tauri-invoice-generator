<script lang="ts" context="module">
	import { z } from 'zod';
	export const profileFormSchema = z.object({
		username: z
			.string()
			.min(2, 'Username must be at least 2 characters.')
			.max(30, 'Username must not be longer than 30 characters'),
		email: z.string({ required_error: 'Please select an email to display' }).email(),
		bio: z.string().min(4).max(160).default('I own a computer.'),
		urls: z.array(z.string().url()).default(['https://shadcn.com', 'https://twitter.com/shadcn'])
	});
	export type ProfileFormSchema = typeof profileFormSchema;
</script>

<script lang="ts">
	import { type Infer, type SuperValidated, superForm } from 'sveltekit-superforms';
	import SuperDebug from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { tick } from 'svelte';
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { cn } from '$lib/utils.js';
	import { browser } from '$app/environment';

	export let data: SuperValidated<Infer<ProfileFormSchema>>;

	const form = superForm(data, {
		SPA: true,
		validators: zodClient(profileFormSchema)
	});

	const { form: formData, enhance } = form;

	function addUrl() {
		$formData.urls = [...$formData.urls, ''];

		tick().then(() => {
			const urlInputs = Array.from(
				document.querySelectorAll<HTMLElement>("#profile-form input[name='urls']")
			);
			const lastInput = urlInputs[urlInputs.length - 1];
			lastInput && lastInput.focus();
		});
	}

	$: selectedEmail = {
		label: $formData.email,
		value: $formData.email
	};
</script>

<form method="POST" class="space-y-8" use:enhance id="profile-form">
	<Form.Field {form} name="username">
		<Form.Control>
			<Form.Label>Username</Form.Label>
			<Input placeholder="@shadcn" bind:value={$formData.username} />
		</Form.Control>
		<Form.Description>
			This is your public display name. It can be your real name or a pseudonym. You can only change
			this once every 30 days.
		</Form.Description>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="bio">
		<Form.Control>
			<Form.Label>Bio</Form.Label>
			<Textarea bind:value={$formData.bio} />
		</Form.Control>
		<Form.Description>
			You can <span>@mention</span> other users and organizations to link to them.
		</Form.Description>
		<Form.FieldErrors />
	</Form.Field>
	<div>
		<Form.Fieldset {form} name="urls">
			<Form.Legend>URLs</Form.Legend>
			{#each $formData.urls as _, i}
				<Form.ElementField {form} name="urls[{i}]">
					<Form.Description class={cn(i !== 0 && 'sr-only')}>
						Add links to your website, blog, or social media profiles.
					</Form.Description>
					<Form.Control>
						<Input bind:value={$formData.urls[i]} />
					</Form.Control>
					<Form.FieldErrors />
				</Form.ElementField>
			{/each}
		</Form.Fieldset>
		<Button type="button" variant="outline" size="sm" class="mt-2" onclick={addUrl}>Add URL</Button>
	</div>

	<Form.Button>Update profile</Form.Button>
</form>

{#if browser}
	<SuperDebug data={$formData} />
{/if}
