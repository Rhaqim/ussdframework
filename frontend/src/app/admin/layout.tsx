"use client";

import React from "react";
import SideNav from "@/components/Navigation";
import { useNav } from "@/context/navigation.context";

const AdminLayout = ({ children }: { children: React.ReactNode }) => {
	const { setSelectedScreen } = useNav();

	const handleScreenSelect = (screen: string) => {
		setSelectedScreen(screen);
	};
	return (
		<div className="flex flex-row justify-between min-h-screen p-4">
			<div>
				<SideNav onSelect={handleScreenSelect} />
			</div>
			<div className="flex-1 rounded-sm bg-gray-600">{children}</div>
		</div>
	);
};

export default AdminLayout;
