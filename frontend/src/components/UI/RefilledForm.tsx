'use client';

import React, { useState, useEffect } from 'react';

export interface FormField {
	label: string;
	name: string;
	type: string;
    value?: string;
	options?: { value: string; label: string }[];
}

interface FormProps<T = any> {
    fields: FormField[];
    onSubmit: (values: T) => void;
}

const ReusableForm: React.FC<FormProps> = ({ fields, onSubmit }) => {
    const [formData, setFormData] = useState<Record<string, any>>({});
    const [isChanged, setIsChanged] = useState(false);

    useEffect(() => {
        const initialFormData: Record<string, any> = {};
        fields.forEach(field => {
            initialFormData[field.name] = field.value || '';
        });
        setFormData(initialFormData);
    }, [fields]);

    // handle change, change the value of the form data
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const formName = e.target.name;
        const value = e.target.value;

        setFormData(prevFormData => ({
            ...prevFormData,
            [formName]: value,
        }));
        setIsChanged(true);
    }

    const handleSelect = (e: React.ChangeEvent<HTMLSelectElement>) => {
        setFormData(prevFormData => ({
            ...prevFormData,
            [e.target.name]: e.target.value,
        }));
        setIsChanged(true);
    }

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        onSubmit(formData);
        setIsChanged(false);
    };

    return (
        <form onSubmit={handleSubmit}>
            {fields.map(field => (
                <div key={field.name} className="mb-4">
                    <label
                        htmlFor={field.name}
                        className="block text-sm font-medium text-gray-100"
                    >
                        {field.label}
                    </label>
                    {field.type === "dropdown" ? (
                        <select
                            id={field.name}
                            name={field.name}
                            onChange={handleSelect}
                            value={formData[field.name]}
                            className="mt-1 p-2 border border-gray-300 text-black rounded-md w-full"
                        >
                            {field.options?.map(option => (
                                <option key={option.value} value={option.value}>
                                    {option.label}
                                </option>
                            ))}
                        </select>
                    ) : (
                        <input
                            type={field.type}
                            id={field.name}
                            name={field.name}
                            onChange={handleChange}
                            value={formData[field.name]}
                            className="mt-1 p-2 border border-gray-300 text-black rounded-md w-full"
                        />
                    )}
                </div>
            ))}
            {isChanged && (
                <button
                    type="submit"
                    className="bg-blue-500 text-white px-4 py-2 rounded-md"
                >
                    Save Changes
                </button>
            )}
        </form>
    );
};

export default ReusableForm;
