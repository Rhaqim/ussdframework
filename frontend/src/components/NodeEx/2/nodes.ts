import type { Node, NodeTypes } from "reactflow";

import { CustomNode } from "./CustomeNode";

import Screen from "@/types/screen.type";

export const initialNodes = (screens: Screen[]) => screens.map((screen, idx) => ({
	id: screen.name,
	type: "screen",
	position: { x: idx * 100, y: idx * 200 },
	data: { screen },
})) satisfies Node[];

export const nodeTypes = {
	screen: CustomNode,
} satisfies NodeTypes;
