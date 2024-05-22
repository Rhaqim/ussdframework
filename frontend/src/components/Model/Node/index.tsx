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

import { Screens } from "@/api/route";

import { CustomNode } from "@/components/Model/Node/nodes/CustomeNode";
import { CustomEdge } from "@/components/Model/Node/edges/CustomeEdge";

import { initialNodes } from "@/components/Model/Node/nodes";
import { initialEdges } from "@/components/Model/Node/edges";

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
			Screens.getAll().then(data => {
				setNodes(initialNodes(data));
				setEdges(initialEdges(data));
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
