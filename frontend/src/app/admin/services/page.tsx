"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";

import { Services } from "@/api/route";
import Table from "@/components/Model/Service/Table";
import Service from "@/types/service.type";
import { LinkButton } from "@/components/UI/Button";

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
		Services.getAll().then(data => {
			setService(data);
		});
	}, []);

	return (
		<div className="flex flex-col space-y-4">
			<LinkButton className="w-60" href="/admin/services/create">
				Create Service
			</LinkButton>
			<Table data={service} />
		</div>
	);
};

export default ServiceHomePage;
