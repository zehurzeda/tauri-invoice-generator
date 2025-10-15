import { z } from 'zod';

export const invoiceFormSchema = z.object({
	clientName: z.string().min(3, {
		message: 'Nome do cliente deve conter ao menos 3 caracteres'
	}),
	clientAddressLine1: z.string().min(5, {
		message: 'Endereco do cliente deve conter ao menos 5 caracteres'
	}),
	clientAddressLine2: z.string().optional(),
	clientEmail: z.string().email({ message: 'Email invalido' }).optional().or(z.literal('')),
	serviceDescription: z.string().min(3, {
		message: 'Descricao do servico deve conter ao menos 3 caracteres'
	}),
	hourlyRate: z.number().positive({
		message: 'Taxa horaria deve ser maior que zero'
	}),
	hoursWorked: z.number().positive({
		message: 'Horas trabalhadas deve ser maior que zero'
	}),
	filenameTemplate: z.string().min(1, {
		message: 'Template do nome do arquivo e obrigatorio'
	}),
	notes: z.string().optional()
});

export type InvoiceFormSchema = typeof invoiceFormSchema;

