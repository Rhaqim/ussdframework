"use client";

import React from "react";
import Header from "./Header";
import SideBar from "./SideBar";

import { useNav } from "@/context/navigation.context";

type LayoutProps = {
	children: React.ReactNode;
};

const Layout: React.FC<LayoutProps> = ({ children }) => {
	const { setSelectedScreen } = useNav();

	const handleScreenSelect = (screen: string) => {
		setSelectedScreen(screen);
	};
	return (
		<div className="flex h-screen">
			<SideBar onSelect={handleScreenSelect} />
			<div className="flex flex-col flex-1">
				<Header />
				<main className="flex-1 bg-gray-100 overflow-y-auto">
					{children}
				</main>
			</div>
		</div>
	);
};

export default Layout;
