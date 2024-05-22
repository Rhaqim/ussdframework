import type { Node, NodeTypes } from "reactflow";

import Screen from "@/types/screen.type";

export const initialNodes = (screens: Screen[]) => {
	console.log("Initial Nodes", screens);

	const nodes = screens.map((screen, idx) => ({
		id: screen.name,
		type: "screen",
		position: { x: idx * 100, y: idx * 200 },
		data: { screen },
	})) satisfies Node[];

	return nodes
	
}