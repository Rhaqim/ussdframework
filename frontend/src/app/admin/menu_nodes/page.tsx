import React from "react";

import Node from "@/components/Model/Node";
import ScreenEditPanel from "@/components/Model/Node/ScreenEditPanel";
import { NavigationProvider } from "@/context/navigation.context";

const MenuNode = () => {
	return (
		<NavigationProvider>
			{/* Pull out of the p-6 padding so the canvas fills all available space */}
			<div className="-m-6 relative" style={{ height: "calc(100vh - 3.5rem)" }}>
				<Node />
				<ScreenEditPanel />
			</div>
		</NavigationProvider>
	);
};

export default MenuNode;

