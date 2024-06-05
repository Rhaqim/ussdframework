"use client";

import React, { useEffect, useState } from "react";

import { Screens } from "@/api/route";
import Table from "@/components/Model/Screen/Table";
import Screen, { ScreenType } from "@/types/screen.type";
import { LinkButton } from "@/components/UI/Button";

const ScreenHomePage = () => {
	const data: Screen[] = [
		{
			id: 1,
			name: "BuyAirtime",
			text: "Enter Amount You would like to send to the recipient",
			screen_type: ScreenType.INPUT,
			default_next_screen: "DefaultNoneScreen",
			service_code: "#123",
			function: "buy_airtime",
			input_identifier: "amount_to_send",
			input_type: "number",
		},
	];
	const [screen, setScreen] = useState<Screen[]>(data);

	useEffect(() => {
		Screens.getAll().then(data => {
			setScreen(data);
		});
	}, []);

	return (
		<div className="flex flex-col space-y-4">
			<LinkButton className="w-40" href="/admin/screens/create">
				Create Screen
			</LinkButton>
			<Table data={screen} />
		</div>
	);
};

export default ScreenHomePage;
