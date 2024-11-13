import { superValidate } from 'sveltekit-superforms';
import type { PageLoad } from './$types';
import { zod } from 'sveltekit-superforms/adapters';
import { z } from 'zod';

const profileFormSchema = z.object({
	username: z
		.string()
		.min(2, 'Username must be at least 2 characters.')
		.max(30, 'Username must not be longer than 30 characters'),
	email: z.string({ required_error: 'Please select an email to display' }).email(),
	bio: z.string().min(4).max(160).default('I own a computer.'),
	urls: z.array(z.string().url()).default(['https://shadcn.com', 'https://twitter.com/shadcn'])
});

export const load: PageLoad = async () => {
	return {
		form: await superValidate(zod(profileFormSchema))
	};
};
