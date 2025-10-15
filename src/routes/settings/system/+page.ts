import { superValidate } from 'sveltekit-superforms';
import type { PageLoad } from './$types';
import { zod } from 'sveltekit-superforms/adapters';
import { load as storeLoad } from '@tauri-apps/plugin-store';
import type { z } from 'zod';
import { systemFormSchema, type SystemFormSchema } from './SystemForm';

export const load: PageLoad = async () => {
	const store = await storeLoad('settings.json', { autoSave: false });
	const systemData = await store.get<z.infer<SystemFormSchema>>('systemSettings');
	return {
		form: await superValidate(systemData, zod(systemFormSchema))
	};
};
