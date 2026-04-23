"use client";

import React from "react";
import Header from "./Header";
import SideBar from "./SideBar";

type LayoutProps = { children: React.ReactNode };

const Layout: React.FC<LayoutProps> = ({ children }) => {
	return (
		<div className="flex h-screen bg-slate-950">
			<SideBar />
			<div className="flex flex-col ml-60 flex-1 overflow-hidden">
				<Header />
				<main className="flex-1 overflow-y-auto">{children}</main>
			</div>
		</div>
	);
};

export default Layout;
