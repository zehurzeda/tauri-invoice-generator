import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	// Redirect to the generate page by default
	throw redirect(307, '/generate');
};

