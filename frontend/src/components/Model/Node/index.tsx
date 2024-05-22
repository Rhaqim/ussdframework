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
} from "reactflow";

import "reactflow/dist/style.css";

import { MenuItems, RouterOptions, Screens } from "@/api/route";

import { CustomNode } from "@/components/Model/Node/nodes/CustomeNode";
import { CustomEdge } from "@/components/Model/Node/edges/CustomeEdge";

import { initialNodes } from "@/components/Model/Node/nodes";
import { initialEdges } from "@/components/Model/Node/edges";

import Screen, { ScreenType } from "@/types/screen.type";

export default function MenuNode() {
	const [nodes, setNodes, onNodesChange] = useNodesState([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState([]);

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

	useEffect(() => {
		async function loadScreens() {
			Screens.getAll().then((screens_: Screen[]) => {
				let screens: Screen[] = screens_;

				for (let i = 0; i < screens_.length; i++) {
					const req = {
						ScreenName: screens_[i].name,
					};

					if (screens_[i].screen_type === ScreenType.MENU) {
						MenuItems.getByQuery(req).then(items => {
							screens_[i].menu_items = items;
						});
					}

					if (screens_[i].screen_type === ScreenType.ROUTER) {
						RouterOptions.getByQuery(req).then(options => {
							screens_[i].router_options = options;
						});
					}
				}

				setNodes(initialNodes(screens));
				setEdges(initialEdges(screens));
			});
		}
		loadScreens();
	}, [setNodes, setEdges]);

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
