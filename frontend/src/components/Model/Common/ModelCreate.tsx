"use client";

import React, { useState, useEffect } from "react";
import { useRouter } from "next/router";
import Screen from "@/types/screen.type";

interface ModelCreateProps {
	onCreate: (screen: Screen) => void;
}

const ModelCreate: React.FC<ModelCreateProps> = ({ onCreate }) => {
	const [screen, setScreen] = useState<Screen>({
		name: "",
		text: "",
		screen_type: "",
		default_next_screen: "",
	});
	const [functions, setFunctions] = useState<string[]>([]);
	const router = useRouter();

	useEffect(() => {
		// Simulated fetch of functions from services
		const fetchFunctions = async () => {
			// Fetch functions from services and set them in state
			const fetchedFunctions: string[] = await fetchFunctionsFromService();
			setFunctions(fetchedFunctions);
		};

		fetchFunctions();
	}, []);

	const fetchFunctionsFromService = async (): Promise<string[]> => {
		// Mock implementation of fetching functions from services
		return ["Function1", "Function2", "Function3"]; // Replace with actual fetched data
	};

	const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		setScreen({ ...screen, [e.target.name]: e.target.value });
	};

	const handleFunctionChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
		setScreen({ ...screen, function: e.target.value });
	};

	const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		e.preventDefault();
		onCreate(screen);
		// Redirect to another page after creating model
		router.push("/model/list"); // Change the route as per your application
	};

	return (
		<div className="container mx-auto px-4 py-8">
			<h1 className="text-3xl font-semibold mb-4">Create New Screen</h1>
			<form onSubmit={handleSubmit} className="max-w-lg">
				<div className="mb-4">
					<label
						className="block text-gray-700 text-sm font-bold mb-2"
						htmlFor="text"
					>
						Text
					</label>
					<input
						type="text"
						id="text"
						name="text"
						value={screen.text}
						onChange={handleInputChange}
						className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
						placeholder="Enter text"
					/>
				</div>
				<div className="mb-4">
					<label
						className="block text-gray-700 text-sm font-bold mb-2"
						htmlFor="screen_type"
					>
						Screen Type
					</label>
					<input
						type="text"
						id="screen_type"
						name="screen_type"
						value={screen.screen_type}
						onChange={handleInputChange}
						className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
						placeholder="Enter screen type"
					/>
				</div>
				<div className="mb-4">
					<label
						className="block text-gray-700 text-sm font-bold mb-2"
						htmlFor="default_next_screen"
					>
						Default Next Screen
					</label>
					<input
						type="text"
						id="default_next_screen"
						name="default_next_screen"
						value={screen.default_next_screen}
						onChange={handleInputChange}
						className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
						placeholder="Enter default next screen"
					/>
				</div>
				<div className="mb-4">
					<label
						className="block text-gray-700 text-sm font-bold mb-2"
						htmlFor="function"
					>
						Function
					</label>
					<select
						id="function"
						name="function"
						value={screen.function}
						onChange={handleFunctionChange}
						className="w-full px-3 py-2 border rounded-md focus:outline-none focus:border-blue-500"
					>
						<option value="">Select Function</option>
						{functions.map(func => (
							<option key={func} value={func}>
								{func}
							</option>
						))}
					</select>
				</div>
				<button
					type="submit"
					className="bg-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-600 focus:outline-none focus:bg-blue-600"
				>
					Create
				</button>
			</form>
		</div>
	);
};

export default ModelCreate;
