import React from "react";

import Node from "@/components/Model/Node";
import ScreenEditPanel from "@/components/Model/Node/ScreenEditPanel";
import { NavigationProvider } from "@/context/navigation.context";

const MenuNode = () => {
	return (
		<NavigationProvider>
			<div className="relative">
				<Node />
				<ScreenEditPanel />
			</div>
		</NavigationProvider>
	);
};

export default MenuNode;
