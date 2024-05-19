"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";

import { Screens } from "@/api/route";
import Table from "@/components/Model/Screen/Table";
import Screen, { ScreenType } from "@/types/screen.type";

const ScreenHomePage = () => {
	const data: Screen[] = [
		{
			id: 1,
			name: "BuyAirtime",
			text: "Enter Amount",
			screen_type: ScreenType.INPUT,
			default_next_screen: "DefaultNoneScreen",
			service_code: "#123",
			function: "buy_airtime",
			input_identifier: "amount",
		},
	];
	const [screen, setScreen] = useState<Screen[]>(data);

	useEffect(() => {
		Screens.getAllScreens().then(data => {
			setScreen(data);
		});
	}, []);

	return (
		<div>
			<Link href="/admin/screens/create">
				<p className="btn-primary">Create Screen</p>
			</Link>
			<Table data={screen} />
		</div>
	);
};

export default ScreenHomePage;
