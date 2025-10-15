import { z } from 'zod';

export const bankDataFormSchema = z.object({
	beneficiaryAccountName: z.string().min(5, {
		message: 'Nome do beneficiario deve contar ao menos 5 caracteres'
	}),
	bankName: z.string().min(5, {
		message: 'Nome do banco deve contar ao menos 5 caracteres'
	}),
	bankAddress: z.string().min(5, {
		message: 'Endereco do banco deve contar ao menos 5 caracteres'
	}),
	accountType: z.string({ required_error: 'Campo obrigatorio' }).min(1, {
		message: 'Selecione um tipo de conta'
	}),
	accountNumber: z.string().min(5, {
		message: 'Numero da conta deve contar ao menos 5 caracteres'
	}),
	wireRouting: z
		.string({
			required_error: 'Campo obrigatorio'
		})
		.min(5, {
			message: 'Wire Routing deve contar ao menos 5 caracteres'
		}),
	swiftCode: z.string().min(5, {
		message: 'Codigo swift deve contar ao menos 5 caracteres'
	})
});

export type BankDataFormSchema = typeof bankDataFormSchema;
