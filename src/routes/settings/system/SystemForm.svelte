<script lang="ts">
	import { type Infer, type SuperValidated, superForm } from 'sveltekit-superforms';
	import SuperDebug from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import * as RadioGroup from '$lib/components/ui/radio-group/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { browser } from '$app/environment';
	import { systemFormSchema, type SystemFormSchema } from './SystemForm';
	import { load } from '@tauri-apps/plugin-store';
	import { setMode } from 'mode-watcher';

	export let data: SuperValidated<Infer<SystemFormSchema>>;

	const form = superForm(data, {
		SPA: true,
		validators: zodClient(systemFormSchema),
		resetForm: false,
		async onUpdate({ form }) {
			if (form.valid) {
				const store = await load('settings.json', { autoSave: false });
				await store.set('systemSettings', form.data);
				setMode(form.data.theme);
				await store.save();
			}
		}
	});

	const { form: formData, enhance } = form;
</script>

<form method="POST" class="space-y-8" use:enhance id="profile-form">
	<Form.Fieldset {form} name="theme">
		<Form.Legend>Theme</Form.Legend>
		<Form.Description>Select the theme for the dashboard.</Form.Description>
		<Form.FieldErrors />
		<RadioGroup.Root
			class="grid max-w-md grid-cols-2 gap-8 pt-2"
			orientation="horizontal"
			bind:value={$formData.theme}
		>
			<Form.Control>
				<Label class="[&:has([data-state=checked])>div]:border-primary">
					<RadioGroup.Item value="light" class="sr-only" />
					<div class="border-muted hover:border-accent items-center rounded-md border-2 p-1">
						<div class="space-y-2 rounded-sm bg-[#ecedef] p-2">
							<div class="space-y-2 rounded-md bg-white p-2 shadow-sm">
								<div class="h-2 w-[80px] rounded-lg bg-[#ecedef]"></div>
								<div class="h-2 w-[100px] rounded-lg bg-[#ecedef]"></div>
							</div>
							<div class="flex items-center space-x-2 rounded-md bg-white p-2 shadow-sm">
								<div class="h-4 w-4 rounded-full bg-[#ecedef]"></div>
								<div class="h-2 w-[100px] rounded-lg bg-[#ecedef]"></div>
							</div>
							<div class="flex items-center space-x-2 rounded-md bg-white p-2 shadow-sm">
								<div class="h-4 w-4 rounded-full bg-[#ecedef]"></div>
								<div class="h-2 w-[100px] rounded-lg bg-[#ecedef]"></div>
							</div>
						</div>
					</div>
					<span class="block w-full p-2 text-center font-normal"> Light </span>
				</Label>
			</Form.Control>
			<Form.Control>
				<Label class="[&:has([data-state=checked])>div]:border-primary">
					<RadioGroup.Item value="dark" class="sr-only" />
					<div
						class="border-muted bg-popover hover:bg-accent hover:text-accent-foreground items-center rounded-md border-2 p-1"
					>
						<div class="space-y-2 rounded-sm bg-slate-950 p-2">
							<div class="space-y-2 rounded-md bg-slate-800 p-2 shadow-sm">
								<div class="h-2 w-[80px] rounded-lg bg-slate-400"></div>
								<div class="h-2 w-[100px] rounded-lg bg-slate-400"></div>
							</div>
							<div class="flex items-center space-x-2 rounded-md bg-slate-800 p-2 shadow-sm">
								<div class="h-4 w-4 rounded-full bg-slate-400"></div>
								<div class="h-2 w-[100px] rounded-lg bg-slate-400"></div>
							</div>
							<div class="flex items-center space-x-2 rounded-md bg-slate-800 p-2 shadow-sm">
								<div class="h-4 w-4 rounded-full bg-slate-400"></div>
								<div class="h-2 w-[100px] rounded-lg bg-slate-400"></div>
							</div>
						</div>
					</div>
					<span class="block w-full p-2 text-center font-normal"> Dark </span>
				</Label>
			</Form.Control>
		</RadioGroup.Root>
	</Form.Fieldset>

	<Form.Button>Salvar</Form.Button>
</form>
