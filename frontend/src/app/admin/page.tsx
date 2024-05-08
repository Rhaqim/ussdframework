"use client";

import React, { useState } from "react";

type Service = {
	id: number;
	name: string;
};

const AdminHomePage = () => {
	const [data, setData] = useState<Service[] | null>(null);

	const fetchData = async () => {
		try {
			const response = await fetch("/api/services");
			if (!response.ok) {
				throw new Error("Failed to fetch data");
			}
			const jsonData = await response.json();
			setData(jsonData);
		} catch (error) {
			console.error("Error fetching data:", error);
		}
	};

	return (
		<div>
			<h1>Welcome to My Database Visualizer</h1>
			<button onClick={fetchData}>Fetch Data</button>
			{data && data.length > 0 && (
				<div>
					<h2>Data:</h2>
					<ul>
						{data.map(item => (
							<li key={item.id}>{item.name}</li>
						))}
					</ul>
				</div>
			)}
		</div>
	);
};

export default AdminHomePage;
