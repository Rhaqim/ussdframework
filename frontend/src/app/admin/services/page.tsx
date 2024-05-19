"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";

import { Services } from "@/api/route";
import Table from "@/components/Model/Service/Table";
import Service from "@/types/service.type";

const ServiceHomePage = () => {
	const data: Service[] = [
		{
			id: 1,
			name: "BuyAirtime",
			function_name: "buy_airtime",
			function_url: "http://localhost:8000/api/buy_airtime",
			data_key: "amount",
			service_code: "#123",
		},
	];

	const [service, setService] = useState<Service[]>(data);

	useEffect(() => {
		Services.getAllServices().then(data => {
			setService(data);
		});
	}, []);

	return (
		<div>
			<Link href="/admin/services/create">
				<p className="btn-primary">Create Service</p>
			</Link>
			<Table data={service} />
		</div>
	);
};

export default ServiceHomePage;
