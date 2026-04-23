"use client";

import React from "react";
import { usePathname } from "next/navigation";

const PAGE_TITLES: Record<string, string> = {
	"/admin":                      "Dashboard",
	"/admin/menu_nodes":           "Visual Editor",
	"/admin/screens":              "Screens",
	"/admin/screens/create":       "Create Screen",
	"/admin/menu_items":           "Menu Items",
	"/admin/menu_items/create":    "Create Menu Item",
	"/admin/router_options":       "Router Options",
	"/admin/router_options/create":"Create Router Option",
	"/admin/services":             "Services",
	"/admin/services/create":      "Create Service",
	"/admin/export":               "Import / Export",
};

const Header = () => {
	const pathname = usePathname();
	const title =
		PAGE_TITLES[pathname] ??
		Object.entries(PAGE_TITLES).find(([k]) => pathname.startsWith(k))?.[1] ??
		"Admin";

	return (
		<header className="h-14 bg-slate-950 border-b border-slate-800 flex items-center px-6 shrink-0">
			<h1 className="text-sm font-semibold text-slate-100">{title}</h1>
			<div className="ml-auto">
				<span className="text-xs text-slate-600">USSD Framework</span>
			</div>
		</header>
	);
};

export default Header;
