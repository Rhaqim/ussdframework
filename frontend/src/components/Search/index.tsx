import React from "react";

interface SearchProps {
	placeholder: string;
	onChange: (value: string) => void;
}

const Search: React.FC<SearchProps> = ({ placeholder, onChange }) => {
	const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		onChange(e.target.value);
	};

	return (
		<input
			type="text"
			placeholder={placeholder}
			onChange={handleChange}
			className="px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-blue-500"
		/>
	);
};

export default Search;
