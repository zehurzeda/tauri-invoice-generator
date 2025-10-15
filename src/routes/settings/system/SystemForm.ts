import { z } from 'zod';

export const systemFormSchema = z.object({
	theme: z.enum(['light', 'dark'], {
		required_error: 'Please select a theme.'
	})
});

export type SystemFormSchema = typeof systemFormSchema;
