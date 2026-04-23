"use client";

import React, { useCallback, useEffect, useMemo } from "react";
import ReactFlow, {
	MiniMap,
	Controls,
	Background,
	useNodesState,
	useEdgesState,
	addEdge,
	BackgroundVariant,
	type NodeMouseHandler,
} from "reactflow";

// import "reactflow/dist/style.css";

import { MenuItems, RouterOptions, Screens } from "@/api/route";

import { CustomNode } from "@/components/Model/Node/nodes/CustomeNode";
import { CustomEdge } from "@/components/Model/Node/edges/CustomeEdge";

import { initialNodes } from "@/components/Model/Node/nodes";
import { initialEdges } from "@/components/Model/Node/edges";

import { useNav } from "@/context/navigation.context";
import Screen, { ScreenType } from "@/types/screen.type";

export default function MenuNode() {
	const [nodes, setNodes, onNodesChange] = useNodesState([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState([]);
	const { setSelectedScreen } = useNav();

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

	const onNodeClick: NodeMouseHandler = useCallback(
		(_event, node) => {
			setSelectedScreen(node.id);
		},
		[setSelectedScreen]
	);

	useEffect(() => {
		async function loadScreens() {
			try {
				const screens_: Screen[] = await Screens.getAll();

				// Fetch menu items and router options in parallel for all relevant screens
				await Promise.all(
					screens_.map(async screen => {
						const req = { ScreenName: screen.name };
						if (screen.screen_type === ScreenType.MENU) {
							screen.menu_items = await MenuItems.getByQuery(req);
						}
						if (screen.screen_type === ScreenType.ROUTER) {
							screen.router_options = await RouterOptions.getByQuery(req);
						}
					})
				);

				setNodes(initialNodes(screens_));
				setEdges(initialEdges(screens_));
			} catch (err) {
				console.error("Failed to load screens:", err);
			}
		}
		loadScreens();
	}, [setNodes, setEdges]);

	return (
		<div style={{ width: "90vw", height: "90vh", margin: "auto" }}>
			<ReactFlow
				nodes={nodes}
				edges={edges}
				nodeTypes={nodeTypes}
				edgeTypes={edgeTypes}
				onNodesChange={onNodesChange}
				onEdgesChange={onEdgesChange}
				onConnect={onConnect}
				onNodeClick={onNodeClick}
				fitView
			>
				<Controls />
				<MiniMap
					nodeColor={node => {
						const colorMap: Record<string, string> = {
							Initial: "#d1d5db",
							Menu: "#fde68a",
							Input: "#bfdbfe",
							Function: "#bbf7d0",
							Router: "#fed7aa",
							Quit: "#fecaca",
						};
						const screenType = (node.data as any)?.screen?.screen_type;
						return colorMap[screenType] ?? "#e5e7eb";
					}}
				/>
				<Background variant={BackgroundVariant.Cross} gap={12} size={1} />
			</ReactFlow>
		</div>
	);
}
