import { superValidate } from 'sveltekit-superforms';
import type { PageLoad } from './$types';
import { zod } from 'sveltekit-superforms/adapters';
import { addressFormSchema, type AddressFormSchema } from './AddressForm';
import { load as storeLoad } from '@tauri-apps/plugin-store';
import type { z } from 'zod';

export const load: PageLoad = async () => {
	const store = await storeLoad('settings.json', { autoSave: false });
	const addressData = await store.get<z.infer<AddressFormSchema>>('addressSettings');
	return {
		form: await superValidate(addressData, zod(addressFormSchema))
	};
};
