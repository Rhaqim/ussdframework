"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";

import { RouterOptions } from "@/api/route";
import Table from "@/components/Model/Screen/Router/Table";
import { RouterOption } from "@/types/screen.type";

const RouterOptionHomePage = () => {
	const data: RouterOption[] = [
		{
			id: 1,
            screen_name: "BuyAirtime",
            router_option: "1",
            next_screen: "DefaultNoneScreen",
		},
	];
	const [routerOptions, setRouterOptions] = useState<RouterOption[]>(data);

	useEffect(() => {
		RouterOptions.getAll().then(data => {
			setRouterOptions(data);
		});
	}, []);

	return (
		<div>
			<Link href="/admin/router_options/create">
				<p className="btn-primary">Create Router Option</p>
			</Link>
			<Table data={routerOptions} />
		</div>
	);
};

export default RouterOptionHomePage;
