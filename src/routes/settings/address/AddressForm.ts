import { z } from 'zod';

export const addressFormSchema = z.object({
	beneficiaryAddressLine1: z.string().min(5, {
		message: 'Endereco do beneficiario deve contar ao menos 5 caracteres'
	}),
	beneficiaryAddressLine2: z.string().optional(),
	beneficiaryAddressState: z.string().min(1, {
		message: 'Selecione um estado'
	}),
	beneficiaryAddressCity: z.string().min(1, {
		message: 'Selecione uma cidade'
	}),
	beneficiaryAddressZip: z.string().min(1, {
		message: 'Digite um CEP'
	})
});

export type AddressFormSchema = typeof addressFormSchema;

export const brazilianStates = [
	{ name: 'Acre', value: 'AC' },
	{ name: 'Alagoas', value: 'AL' },
	{ name: 'Amapá', value: 'AP' },
	{ name: 'Amazonas', value: 'AM' },
	{ name: 'Bahia', value: 'BA' },
	{ name: 'Ceará', value: 'CE' },
	{ name: 'Distrito Federal', value: 'DF' },
	{ name: 'Espírito Santo', value: 'ES' },
	{ name: 'Goiás', value: 'GO' },
	{ name: 'Maranhão', value: 'MA' },
	{ name: 'Mato Grosso', value: 'MT' },
	{ name: 'Mato Grosso do Sul', value: 'MS' },
	{ name: 'Minas Gerais', value: 'MG' },
	{ name: 'Pará', value: 'PA' },
	{ name: 'Paraíba', value: 'PB' },
	{ name: 'Paraná', value: 'PR' },
	{ name: 'Pernambuco', value: 'PE' },
	{ name: 'Piauí', value: 'PI' },
	{ name: 'Rio de Janeiro', value: 'RJ' },
	{ name: 'Rio Grande do Norte', value: 'RN' },
	{ name: 'Rio Grande do Sul', value: 'RS' },
	{ name: 'Rondônia', value: 'RO' },
	{ name: 'Roraima', value: 'RR' },
	{ name: 'Santa Catarina', value: 'SC' },
	{ name: 'São Paulo', value: 'SP' },
	{ name: 'Sergipe', value: 'SE' },
	{ name: 'Tocantins', value: 'TO' }
];
