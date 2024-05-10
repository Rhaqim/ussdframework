export interface FormField {
	label: string;
	name: string;
	type: string;
	options?: { value: string; label: string }[];
}

declare interface FormProps {
	fields: FormField[];
	onSubmit: (data: any) => void;
}

export default FormProps;
