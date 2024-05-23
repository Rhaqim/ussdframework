import type { Node } from "reactflow";

import Screen from "@/types/screen.type";

const calculateNodePositions = (
	nodes: Screen[],
	nodeMap: Map<string, Screen>,
	startNode: Screen,
	visited: Set<string>,
	depth: number = 0,
	yOffset: number = 0
): Node[] => {
	if (visited.has(startNode.name)) return [];

	visited.add(startNode.name);

	const x = depth * 300; // Horizontal space between levels
	const y = yOffset;

	const node: Node = {
		id: startNode.name,
		type: "screen",
		position: { x, y },
		data: { screen: startNode },
	};

	const nextNode = nodeMap.get(startNode.default_next_screen!);

	const childNodes = nextNode
		? calculateNodePositions(
				nodes,
				nodeMap,
				nextNode,
				visited,
				depth + 1,
				yOffset + 100
		  )
		: [];

	return [node, ...childNodes];
};

export const initialNodes = (screens: Screen[]) => {
	console.log("Initial Nodes", screens);

	// Create a map of node names to nodes for quick lookup
	const nodeMap = new Map(screens.map(screen => [screen.name, screen]));

	// Identify root nodes (nodes that are not the default_next_screen of any other node)
	const rootNodes = screens.filter(
		screen => !screens.some(s => s.default_next_screen === screen.name)
	);

	// Calculate positions starting from each root node
	let yOffset = 0;
	const nodes = rootNodes.flatMap(rootNode => {
		const visited = new Set<string>();
		const positionedNodes = calculateNodePositions(
			screens,
			nodeMap,
			rootNode,
			visited,
			0,
			yOffset
		);
		yOffset += positionedNodes.length * 100; // Adjust vertical spacing for each root subtree
		return positionedNodes;
	});

	return nodes;
};