<script lang="ts">
	import { type Infer, type SuperValidated, superForm } from 'sveltekit-superforms';
	import SuperDebug from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { browser } from '$app/environment';
	import { bankDataFormSchema, type BankDataFormSchema } from './BankForm';
	import { load } from '@tauri-apps/plugin-store';
	import * as Select from '@/components/ui/select';

	export let data: SuperValidated<Infer<BankDataFormSchema>>;

	const form = superForm(data, {
		SPA: true,
		validators: zodClient(bankDataFormSchema),
		resetForm: false,
		async onUpdate({ form }) {
			if (form.valid) {
				const store = await load('settings.json', { autoSave: false });
				await store.set('bankData', form.data);
				await store.save();
			}
		}
	});

	const { form: formData, enhance } = form;
</script>

<form method="POST" class="space-y-8" use:enhance id="profile-form">
	<Form.Field {form} name="beneficiaryAccountName">
		<Form.Control>
			<Form.Label>Nome do beneficiario</Form.Label>
			<Input bind:value={$formData.beneficiaryAccountName} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="bankName">
		<Form.Control>
			<Form.Label>Nome do Banco</Form.Label>
			<Input bind:value={$formData.bankName} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="bankAddress">
		<Form.Control>
			<Form.Label>Endereco do Banco</Form.Label>
			<Input bind:value={$formData.bankAddress} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="accountNumber">
		<Form.Control>
			<Form.Label>Numero da conta</Form.Label>
			<Input bind:value={$formData.accountNumber} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="accountType">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Tipo de conta</Form.Label>
				<Select.Root type="single" bind:value={$formData.accountType} name={props.name}>
					<Select.Trigger {...props}>
						{$formData.accountType ?? 'Selecione o tipo de conta'}
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="Checking" label="Checking" />
						<Select.Item value="Savings" label="Savings" />
					</Select.Content>
				</Select.Root>
			{/snippet}
		</Form.Control>
		<Form.Description>Checking = Conta Corrente, Savings = Poupanca.</Form.Description>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="wireRouting">
		<Form.Control>
			<Form.Label>Wire Routing</Form.Label>
			<Input bind:value={$formData.wireRouting} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="swiftCode">
		<Form.Control>
			<Form.Label>Codigo swift</Form.Label>
			<Input bind:value={$formData.swiftCode} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Button>Salvar</Form.Button>
</form>
