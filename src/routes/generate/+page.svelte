<script lang="ts">
	import { type Infer, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import { invoiceFormSchema, type InvoiceFormSchema } from './InvoiceForm';
	import { load as storeLoad } from '@tauri-apps/plugin-store';
	import { invoke } from '@tauri-apps/api/core';
	import { save } from '@tauri-apps/plugin-dialog';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const form = superForm(data.form, {
		SPA: true,
		validators: zodClient(invoiceFormSchema),
		resetForm: false,
		async onUpdate({ form }) {
			if (form.valid) {
				await generateInvoice(form.data);
			}
		}
	});

	const { form: formData, enhance } = form;

	let isGenerating = $state(false);
	let generationStatus = $state<{ type: 'success' | 'error'; message: string } | null>(null);
	let calculatedTotal = $derived(
		$formData.hourlyRate && $formData.hoursWorked
			? ($formData.hourlyRate * $formData.hoursWorked).toFixed(2)
			: '0.00'
	);

	async function generateInvoice(formValues: Infer<InvoiceFormSchema>) {
		isGenerating = true;
		generationStatus = null;

		try {
			// Check if settings are configured
			if (!data.bankData || !data.addressData) {
				throw new Error(
					'Por favor, configure os dados bancarios e de endereco nas configuracoes antes de gerar uma invoice.'
				);
			}

			// Calculate the next sequence number
			const store = await storeLoad('settings.json', { autoSave: false });
			const currentSequence = data.currentSequence + 1;

			// Generate the default filename from template
			let defaultFileName = 'invoice.pdf';
			if (formValues.filenameTemplate) {
				if (formValues.filenameTemplate.includes('{sequence}')) {
					const paddedSequence = currentSequence.toString().padStart(3, '0');
					defaultFileName =
						formValues.filenameTemplate.replace('{sequence}', paddedSequence) + '.pdf';
				} else {
					defaultFileName = formValues.filenameTemplate + '.pdf';
				}
			}

			// Open save dialog with the generated filename as default
			const filePath = await save({
				defaultPath: defaultFileName,
				filters: [
					{
						name: 'PDF',
						extensions: ['pdf']
					}
				]
			});

			if (!filePath) {
				// User cancelled
				isGenerating = false;
				return;
			}

			// Update sequence number in store
			await store.set('invoiceSequence', currentSequence);
			await store.save();

			// Get current date
			const currentDate = new Date().toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			});

			// Prepare invoice data for Rust backend
			const invoiceData = {
				client: {
					name: formValues.clientName,
					address_line1: formValues.clientAddressLine1,
					address_line2: formValues.clientAddressLine2 || null,
					email: formValues.clientEmail || null
				},
				bank_data: {
					beneficiary_account_name: data.bankData.beneficiaryAccountName,
					bank_name: data.bankData.bankName,
					bank_address: data.bankData.bankAddress,
					account_type: data.bankData.accountType,
					account_number: data.bankData.accountNumber,
					wire_routing: data.bankData.wireRouting,
					swift_code: data.bankData.swiftCode
				},
				address_data: {
					beneficiary_address_line1: data.addressData.beneficiaryAddressLine1,
					beneficiary_address_line2: data.addressData.beneficiaryAddressLine2 || null,
					beneficiary_address_state: data.addressData.beneficiaryAddressState,
					beneficiary_address_city: data.addressData.beneficiaryAddressCity,
					beneficiary_address_zip: data.addressData.beneficiaryAddressZip
				},
				service_description: formValues.serviceDescription,
				hourly_rate: formValues.hourlyRate,
				hours_worked: formValues.hoursWorked,
				invoice_number: currentSequence.toString().padStart(3, '0'),
				notes: formValues.notes || null,
				invoice_date: currentDate
			};

			// Call Tauri command to generate PDF
			const savedPath = await invoke<string>('generate_invoice', {
				invoiceData,
				filePath: filePath
			});

			// Save client data for next time
			await store.set('lastClientData', formValues);
			await store.save();

			generationStatus = {
				type: 'success',
				message: `Invoice gerada com sucesso: ${savedPath}`
			};

			// Open the generated PDF
			try {
				await invoke('open_pdf', { filePath: savedPath });
			} catch (openError) {
				console.error('Failed to open PDF:', openError);
				// Don't fail the whole operation if opening fails
			}

			// Update the sequence counter in the page data
			data.currentSequence = currentSequence;
		} catch (error) {
			console.error('Error generating invoice:', error);
			generationStatus = {
				type: 'error',
				message: `Erro ao gerar invoice: ${error}`
			};
		} finally {
			isGenerating = false;
		}
	}
</script>

<div class="space-y-6">
	<div>
		<h3 class="text-lg font-medium">Gerar Invoice</h3>
		<p class="text-muted-foreground text-sm">
			Preencha os dados do cliente e as informacoes da invoice para gera-la em PDF.
		</p>
	</div>

	<Separator />

	{#if generationStatus}
		<Card.Card class={generationStatus.type === 'error' ? 'border-red-500' : 'border-green-500'}>
			<Card.CardContent class="pt-6">
				<p class={generationStatus.type === 'error' ? 'text-red-600' : 'text-green-600'}>
					{generationStatus.message}
				</p>
			</Card.CardContent>
		</Card.Card>
	{/if}

	<form method="POST" class="space-y-8" use:enhance>
		<Card.Card>
			<Card.CardHeader>
				<Card.CardTitle>Dados do Cliente</Card.CardTitle>
				<Card.CardDescription>Informacoes sobre quem vai pagar a invoice</Card.CardDescription>
			</Card.CardHeader>
			<Card.CardContent class="space-y-4">
				<Form.Field {form} name="clientName">
					<Form.Control>
						<Form.Label>Nome do Cliente</Form.Label>
						<Input bind:value={$formData.clientName} placeholder="Nome completo ou empresa" />
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>

				<Form.Field {form} name="clientAddressLine1">
					<Form.Control>
						<Form.Label>Endereco do Cliente - Linha 1</Form.Label>
						<Input
							bind:value={$formData.clientAddressLine1}
							placeholder="Endereco completo do cliente"
						/>
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>

				<Form.Field {form} name="clientAddressLine2">
					<Form.Control>
						<Form.Label>Endereco do Cliente - Linha 2 (Opcional)</Form.Label>
						<Input
							bind:value={$formData.clientAddressLine2}
							placeholder="Complemento, apartamento, etc."
						/>
					</Form.Control>
					<Form.FieldErrors />
				</Form.Field>

				<Form.Field {form} name="clientEmail">
					<Form.Control>
						<Form.Label>Email do Cliente (Opcional)</Form.Label>
						<Input
							bind:value={$formData.clientEmail}
							type="email"
							placeholder="cliente@exemplo.com"
						/>
					</Form.Control>
					<Form.Description>Email para contato do cliente</Form.Description>
					<Form.FieldErrors />
				</Form.Field>
			</Card.CardContent>
		</Card.Card>

		<Card.Card>
			<Card.CardHeader>
				<Card.CardTitle>Detalhes do Servico</Card.CardTitle>
				<Card.CardDescription>Informacoes sobre o trabalho realizado</Card.CardDescription>
			</Card.CardHeader>
			<Card.CardContent class="space-y-4">
				<Form.Field {form} name="serviceDescription">
					<Form.Control>
						<Form.Label>Descricao do Servico</Form.Label>
						<Input
							bind:value={$formData.serviceDescription}
							placeholder="Ex: Professional Services, Web Development, Consulting"
						/>
					</Form.Control>
					<Form.Description>Nome do servico que aparecera na invoice</Form.Description>
					<Form.FieldErrors />
				</Form.Field>

				<Form.Field {form} name="hourlyRate">
					<Form.Control>
						<Form.Label>Taxa Horaria ($)</Form.Label>
						<Input
							bind:value={$formData.hourlyRate}
							type="number"
							step="0.01"
							min="0"
							placeholder="0.00"
						/>
					</Form.Control>
					<Form.Description>Valor cobrado por hora de trabalho</Form.Description>
					<Form.FieldErrors />
				</Form.Field>

				<Form.Field {form} name="hoursWorked">
					<Form.Control>
						<Form.Label>Horas Trabalhadas</Form.Label>
						<Input
							bind:value={$formData.hoursWorked}
							type="number"
							step="0.25"
							min="0"
							placeholder="0.00"
						/>
					</Form.Control>
					<Form.Description>Numero de horas trabalhadas</Form.Description>
					<Form.FieldErrors />
				</Form.Field>

				<div class="rounded-lg border bg-muted p-4">
					<div class="flex items-center justify-between">
						<span class="font-medium text-lg">Total:</span>
						<span class="font-bold text-2xl">${calculatedTotal}</span>
					</div>
				</div>
			</Card.CardContent>
		</Card.Card>

		<Card.Card>
			<Card.CardHeader>
				<Card.CardTitle>Configuracoes da Invoice</Card.CardTitle>
				<Card.CardDescription>Informacoes adicionais</Card.CardDescription>
			</Card.CardHeader>
			<Card.CardContent class="space-y-4">
				<Form.Field {form} name="filenameTemplate">
					<Form.Control>
						<Form.Label>Template do Nome do Arquivo</Form.Label>
						<Input bind:value={$formData.filenameTemplate} placeholder={'INV_{sequence}'} />
					</Form.Control>
					<Form.Description>
						Use {'{'} sequence {'}'} para numeracao sequencial. Exemplo: INV_{'{'} sequence {'}'} gera
						INV_001, INV_002, etc. Proximo numero: {(data.currentSequence + 1)
							.toString()
							.padStart(3, '0')}
					</Form.Description>
					<Form.FieldErrors />
				</Form.Field>

				<Form.Field {form} name="notes">
					<Form.Control>
						<Form.Label>Notas Adicionais (Opcional)</Form.Label>
						<Textarea
							bind:value={$formData.notes}
							placeholder="Informacoes adicionais para incluir na invoice..."
							class="min-h-[100px]"
						/>
					</Form.Control>
					<Form.Description>Observacoes ou termos adicionais</Form.Description>
					<Form.FieldErrors />
				</Form.Field>
			</Card.CardContent>
		</Card.Card>

		<Form.Button disabled={isGenerating} class="w-full">
			{isGenerating ? 'Gerando Invoice...' : 'Gerar Invoice em PDF'}
		</Form.Button>
	</form>
</div>
