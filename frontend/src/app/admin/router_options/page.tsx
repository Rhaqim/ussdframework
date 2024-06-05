"use client";

import React, { useEffect, useState } from "react";

import { RouterOptions } from "@/api/route";
import Table from "@/components/Model/Screen/Router/Table";
import { RouterOption } from "@/types/screen.type";
import { LinkButton } from "@/components/UI/Button";

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
		<div className="flex flex-col space-y-4">
			<LinkButton className="w-60" href="/admin/router_options/create">
				Create Router Option
			</LinkButton>
			<Table data={routerOptions} />
		</div>
	);
};

export default RouterOptionHomePage;
