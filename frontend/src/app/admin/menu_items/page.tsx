"use client";

import React, { useEffect, useState } from "react";

import { MenuItems } from "@/api/route";
import Table from "@/components/Model/Screen/Menu/Table";
import { MenuItem } from "@/types/screen.type";
import { LinkButton } from "@/components/UI/Button";

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
		<div className="flex flex-col space-y-4">
			<LinkButton className="w-60" href="/admin/menu_items/create">
				Create Menu Item
			</LinkButton>
			<Table data={menuItems} />
		</div>
	);
};

export default RouterOptionHomePage;
