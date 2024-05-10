"use client";

import React, { useState } from "react";

interface Option {
	value: string;
	label: string;
}

interface DropdownProps {
	options: Option[];
	onSelect: (value: string) => void;
}

const Dropdown: React.FC<DropdownProps> = ({ options, onSelect }) => {
	const [searchTerm, setSearchTerm] = useState<string>("");
	const filteredOptions = options.filter(option =>
		option.label.toLowerCase().includes(searchTerm.toLowerCase())
	);

	const handleSelect = (value: string) => {
		onSelect(value);
	};

	return (
		<div className="relative text-black">
			<input
				type="text"
				placeholder="Search..."
				value={searchTerm}
				onChange={e => setSearchTerm(e.target.value)}
				className="px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500"
			/>
			<select
				onChange={e => handleSelect(e.target.value)}
				className="mt-1 block w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500"
			>
				{filteredOptions.map(option => (
					<option key={option.value} value={option.value}>
						{option.label}
					</option>
				))}
			</select>
		</div>
	);
};

export default Dropdown;
