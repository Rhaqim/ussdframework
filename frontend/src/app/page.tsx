"use client";

import React, { useEffect, useState } from "react";
import ServiceForm from "@/components/Model/Service/Form";
import { Services } from "@/api/route";

export default function Home() {
	const [services, setServices] = useState([]);

	const getServices = async () => {
		Services.getAllServices().then(data => {
			console.log(data);
			setServices(data);
		});
	};

	return (
		<main className="bg-black flex min-h-screen flex-col items-center justify-between p-4">
			<div className="z-10 w-full items-center justify-between font-mono text-sm lg:flex">
				<ServiceForm />

				<button
					onClick={getServices}
					className="bg-white text-black p-4 m-4 rounded-lg"
				>
					Get Services
				</button>

				{services.map((service, idx) => (
					<div key={idx} className="bg-white p-4 m-4 rounded-lg">
						<h1 className="text-lg font-bold">{service}</h1>
					</div>
				))}
			</div>
		</main>
	);
}
