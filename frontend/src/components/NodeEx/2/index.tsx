"use client";

import React, { useCallback, useMemo } from "react";
import ReactFlow, {
	MiniMap,
	Controls,
	Background,
	useNodesState,
	useEdgesState,
	addEdge,
	BackgroundVariant,
	Edge,
	Node,
	EdgeTypes,
} from "reactflow";

import "reactflow/dist/style.css";

import { CustomNode } from "@/components/NodeEx/2/CustomeNode";
import { CustomEdge } from "@/components/NodeEx/2/CustomeEdge";

import Screen, { ScreenType } from "@/types/screen.type";

export async function fetchScreens(): Promise<Screen[]> {
	const response = await fetch("/api/screens");
	if (!response.ok) {
		throw new Error("Failed to fetch screens");
	}
	const screens: Screen[] = await response.json();
	return screens;
}

const screens: Screen[] = [
	{
		name: "Home",
		screen_type: ScreenType.INITIAL,
		text: "This is the home screen.",
		default_next_screen: "MainMenu",
	},
	{
		name: "MainMenu",
		screen_type: ScreenType.MENU,
		text: "Welcome to the main menu screen",
		menu_items: [
			{
				name: "About",
				next_screen: "About",
				option: "2",
				display_name: "About",
				screen_name: "MainMenu",
			},
			{
				name: "Contact",
				next_screen: "Contact",
				option: "3",
				display_name: "Contact",
				screen_name: "MainMenu",
			},
		],
		default_next_screen: "Home",
	},
	{
		name: "About",
		screen_type: ScreenType.INPUT,
		text: "This is the about screen.",
		input_identifier: "text",
		default_next_screen: "Contact",
	},
	{
		name: "Contact",
		screen_type: ScreenType.QUIT,
		text: "This is the contact screen.",
		default_next_screen: "MainMenu",
	},
];

// const screens = await fetchScreens();
const initialNodes = screens.map((screen, idx) => ({
	id: screen.name,
	type: "screen",
	position: { x: idx * 100, y: idx * 200 },
	data: { screen },
})) satisfies Node[];

const initialEdges = screens.flatMap(screen => {
	
	const edges = [];
	const id = `${screen.name}->${screen.default_next_screen}`;
	const source = screen.name;
	const target = screen.default_next_screen;

	edges.push({
		id: id,
		source: source,
		target: target,
		animated: true,
		label: "Next",
		type: "custom",
		data: { id: id },
	});

	// Handle menu items
	if (screen.menu_items) {
		screen.menu_items.forEach(item => {
			edges.push({
				id: `${screen.name}->${item.next_screen}`,
				source: screen.name,
				target: item.next_screen,
				animated: true,
				label: item.display_name,
				type: "custom",
				data: { id: `${screen.name}->${item.next_screen}` },
			});
		});
	}
	// Handle router options
	if (screen.router_options) {
		screen.router_options.forEach(option => {
			if (option.next_screen) {
				edges.push({
					id: `${screen.name}->${option.next_screen}`,
					source: screen.name,
					target: option.next_screen,
					animated: true,
					label: "Next",
					type: "custom",
					data: { id: `${screen.name}->${option.next_screen}` },
				});
			}
		});
	}
	return edges;
}) satisfies Edge[];

export default function App() {
	const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
	const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

	// useEffect(() => {
	// 	async function loadScreens() {

	// 		setNodes(initialNodes);
	// 		setEdges(initialEdges);
	// 	}

	// 	loadScreens();
	// }, [setNodes, setEdges]);

	const nodeTypes = useMemo(
		() => ({
			screen: CustomNode,
		}),
		[]
	);

	const edgeTypes = useMemo(
		() => ({
			custom: CustomEdge,
		}),
		[]
	);

	const onConnect = useCallback(
		(params: any) => setEdges(eds => addEdge(params, eds)),
		[setEdges]
	);

	return (
		<div style={{ width: "90vw", height: "90vh" }}>
			<ReactFlow
				nodes={nodes}
				edges={edges}
				nodeTypes={nodeTypes}
				edgeTypes={edgeTypes}
				onNodesChange={onNodesChange}
				onEdgesChange={onEdgesChange}
				onConnect={onConnect}
			>
				<Controls />
				<MiniMap />
				<Background variant={BackgroundVariant.Cross} gap={12} size={1} />
			</ReactFlow>
		</div>
	);
}
