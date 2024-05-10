"use client";

import React, { useState } from "react";

import FormProps from "@/types/form.type";

const Form: React.FC<FormProps> = ({ fields, onSubmit }) => {
	const [formData, setFormData] = useState<{ [key: string]: any }>({});

	const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setFormData({
			...formData,
			[e.target.name]: e.target.value,
		});
	};

	const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();
		onSubmit(formData);
	};

	return (
		<form onSubmit={handleSubmit}>
			{fields.map(field => (
				<div key={field.name} className="mb-4">
					<label
						htmlFor={field.name}
						className="block text-sm font-medium text-gray-700"
					>
						{field.label}
					</label>
					<input
						type={field.type}
						id={field.name}
						name={field.name}
						onChange={handleChange}
						className="mt-1 p-2 border border-gray-300 rounded-md w-full"
					/>
				</div>
			))}
			<button
				type="submit"
				className="bg-blue-500 text-white px-4 py-2 rounded-md"
			>
				Submit
			</button>
		</form>
	);
};

export default Form;
