<script lang="ts">
	import { type Infer, type SuperValidated, superForm } from 'sveltekit-superforms';
	import SuperDebug from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { browser } from '$app/environment';
	import { addressFormSchema, type AddressFormSchema, brazilianStates } from './AddressForm';
	import { load } from '@tauri-apps/plugin-store';
	import * as Select from '@/components/ui/select';

	export let data: SuperValidated<Infer<AddressFormSchema>>;

	const form = superForm(data, {
		SPA: true,
		validators: zodClient(addressFormSchema),
		resetForm: false,
		async onUpdate({ form }) {
			if (form.valid) {
				const store = await load('settings.json', { autoSave: false });
				await store.set('addressSettings', form.data);
				await store.save();
			}
		}
	});

	const { form: formData, enhance } = form;
</script>

<form method="POST" class="space-y-8" use:enhance id="profile-form">
	<Form.Field {form} name="beneficiaryAddressLine1">
		<Form.Control>
			<Form.Label>Endereco - Linha 1</Form.Label>
			<Input bind:value={$formData.beneficiaryAddressLine1} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="beneficiaryAddressLine2">
		<Form.Control>
			<Form.Label>Endereco - Linha 2 (Opcional)</Form.Label>
			<Input bind:value={$formData.beneficiaryAddressLine2} />
		</Form.Control>
		<Form.Description>Complemento, apartamento, etc.</Form.Description>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="beneficiaryAddressZip">
		<Form.Control>
			<Form.Label>CEP</Form.Label>
			<Input bind:value={$formData.beneficiaryAddressZip} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="beneficiaryAddressCity">
		<Form.Control>
			<Form.Label>Cidade</Form.Label>
			<Input bind:value={$formData.beneficiaryAddressCity} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="beneficiaryAddressState">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Estado</Form.Label>
				<Select.Root type="single" bind:value={$formData.beneficiaryAddressState} name={props.name}>
					<Select.Trigger {...props}>
						{$formData.beneficiaryAddressState ?? 'Selecione o estado'}
					</Select.Trigger>
					<Select.Content>
						{#each brazilianStates as state}
							<Select.Item value={state.value} label={state.name} />
						{/each}
					</Select.Content>
				</Select.Root>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Button>Salvar</Form.Button>
</form>
