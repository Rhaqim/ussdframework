"use client";

import React, { useEffect, useState } from "react";
import ServiceForm from "@/components/Model/Service/Form";
import { Services } from "@/api/route";

export default function Home() {

	const [services, setServices] = useState([]);

	useEffect(() => {
		Services.getServiceList().then((data) => {
			console.log(data);
			setServices(data);
		});
	}, []);

	return (
		<main className="bg-black flex min-h-screen flex-col items-center justify-between p-4">
			<div className="z-10 w-full items-center justify-between font-mono text-sm lg:flex">
				<ServiceForm />
			</div>
		</main>
	);
}
