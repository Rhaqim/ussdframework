"use client";

import React, { useEffect, useState } from "react";

import { Services } from "@/api/route";
import Service from "@/types/service.type";

const ServicePage = ({ params }: { params: { id: string } }) => {
	const [service, setService] = useState<Service | null>(null);

    console.log("Params", params);

	useEffect(() => {
        // conver id to number
        const id = parseInt(params.id);

		Services.get(id).then(data => {
			setService(data);
		});
	}, [params.id]);

	return (
		<div className="container mx-auto">
			<h1 className="text-2xl font-semibold">{service?.name}</h1>
			<p className="text-lg">{service?.function_name}</p>
			<p className="text-lg">{service?.function_url}</p>
			<p className="text-lg">{service?.data_key}</p>
			<p className="text-lg">{service?.service_code}</p>
		</div>
	);
};

export default ServicePage;
