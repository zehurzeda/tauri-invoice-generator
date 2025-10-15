import { superValidate } from 'sveltekit-superforms';
import type { PageLoad } from './$types';
import { zod } from 'sveltekit-superforms/adapters';
import { invoiceFormSchema, type InvoiceFormSchema } from './InvoiceForm';
import { load as storeLoad } from '@tauri-apps/plugin-store';
import type { z } from 'zod';
import { type BankDataFormSchema } from '../settings/BankForm';
import { type AddressFormSchema } from '../settings/address/AddressForm';

export const load: PageLoad = async () => {
	const store = await storeLoad('settings.json', { autoSave: false });

	// Load saved settings
	const bankData = await store.get<z.infer<BankDataFormSchema>>('bankData');
	const addressData = await store.get<z.infer<AddressFormSchema>>('addressSettings');

	// Get the current invoice sequence number
	const currentSequence = (await store.get<number>('invoiceSequence')) || 0;

	// Load saved client data (if any)
	const savedClientData = await store.get<z.infer<InvoiceFormSchema>>('lastClientData');

	// Initialize form with saved client data or default values
	const defaultValues = savedClientData || {
		clientName: '',
		clientAddressLine1: '',
		clientAddressLine2: '',
		clientEmail: '',
		serviceDescription: 'Professional Services',
		hourlyRate: 0,
		hoursWorked: 0,
		filenameTemplate: 'INV_{sequence}',
		notes: ''
	};

	return {
		form: await superValidate(defaultValues, zod(invoiceFormSchema)),
		bankData,
		addressData,
		currentSequence
	};
};

