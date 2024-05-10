"use client";

import React from "react";
import SideBar from "@/components/Layout/SideBar";
import { useNav } from "@/context/navigation.context";

const AdminLayout = ({ children }: { children: React.ReactNode }) => {
	const { setSelectedScreen } = useNav();

	const handleScreenSelect = (screen: string) => {
		setSelectedScreen(screen);
	};
	return (
		<div className="flex flex-row justify-between min-h-screen p-4">
			<div>
				<SideBar onSelect={handleScreenSelect} />
			</div>
			<div className="flex-1 rounded-sm bg-gray-600">{children}</div>
		</div>
	);
};

export default AdminLayout;
