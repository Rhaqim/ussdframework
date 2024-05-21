"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";

import { MenuItems } from "@/api/route";
import Table from "@/components/Model/Screen/Menu/Table";
import { MenuItem } from "@/types/screen.type";

const RouterOptionHomePage = () => {
	const data: MenuItem[] = [
		{
			id: 1,
			screen_name: "BuyAirtime",
			name: "Buy Airtime",
			option: "1",
			display_name: "Buy Airtime",
			next_screen: "DefaultNoneScreen",
		},
	];
	const [menuItems, setMenuItems] = useState<MenuItem[]>(data);

	useEffect(() => {
		MenuItems.getAll().then(data => {
			setMenuItems(data);
		});
	}, []);

	return (
		<div>
			<Link href="/admin/router_options/create">
				<p className="btn-primary">Create Router Option</p>
			</Link>
			<Table data={menuItems} />
		</div>
	);
};

export default RouterOptionHomePage;
