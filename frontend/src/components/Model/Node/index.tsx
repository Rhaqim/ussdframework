"use client";

import React, { useCallback, useEffect, useMemo, useState } from "react";
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

// @ts-ignore
import "reactflow/dist/style.css";

import { MenuItems, RouterOptions, Screens } from "@/api/route";

import { CustomNode } from "@/components/Model/Node/nodes/CustomeNode";
import { CustomEdge } from "@/components/Model/Node/edges/CustomeEdge";

import { initialNodes } from "@/components/Model/Node/nodes";
import { initialEdges } from "@/components/Model/Node/edges";

import { useNav } from "@/context/navigation.context";
import Screen, { MenuItem, RouterOption } from "@/types/screen.type";

export default function MenuNode() {
	const [nodes, setNodes, onNodesChange] = useNodesState([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState([]);
	const [loading, setLoading] = useState(true);
	const [screenCount, setScreenCount] = useState(0);
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
			setLoading(true);
			try {
				// 3 parallel requests instead of 1 + N per-screen requests
				const [screens_, allMenuItems, allRouterOptions] = await Promise.all([
					Screens.getAll() as Promise<Screen[]>,
					MenuItems.getAll() as Promise<MenuItem[]>,
					RouterOptions.getAll() as Promise<RouterOption[]>,
				]);

				// Group by screen_name client-side
				const menuItemsByScreen = allMenuItems.reduce<Record<string, MenuItem[]>>(
					(acc, item) => { (acc[item.screen_name] ??= []).push(item); return acc; },
					{}
				);
				const routerOptionsByScreen = allRouterOptions.reduce<Record<string, RouterOption[]>>(
					(acc, opt) => { (acc[opt.screen_name] ??= []).push(opt); return acc; },
					{}
				);

				for (const screen of screens_) {
					screen.menu_items = menuItemsByScreen[screen.name] ?? [];
					screen.router_options = routerOptionsByScreen[screen.name] ?? [];
				}

				setScreenCount(screens_.length);
				setNodes(initialNodes(screens_));
				setEdges(initialEdges(screens_));
			} catch (err) {
				console.error("Failed to load screens:", err);
			} finally {
				setLoading(false);
			}
		}
		loadScreens();
	}, [setNodes, setEdges]);

	return (
		<div style={{ width: "100%", height: "100%" }} className="relative bg-slate-950">
			{/* Loading overlay */}
			{loading && (
				<div className="absolute inset-0 z-20 flex items-center justify-center bg-slate-950/80 backdrop-blur-sm">
					<div className="flex flex-col items-center gap-3">
						<div className="w-8 h-8 border-2 border-slate-700 border-t-indigo-500 rounded-full animate-spin" />
						<p className="text-sm text-slate-400">Loading screens…</p>
					</div>
				</div>
			)}

			{/* Empty state overlay */}
			{!loading && screenCount === 0 && (
				<div className="absolute inset-0 z-20 flex items-center justify-center">
					<div className="text-center space-y-4">
						<div className="w-16 h-16 mx-auto rounded-2xl bg-slate-800 border border-slate-700 flex items-center justify-center text-3xl">
							🗺️
						</div>
						<div>
							<p className="text-slate-200 font-semibold">No screens yet</p>
							<p className="text-slate-500 text-sm mt-1">Create your first screen to start building the flow</p>
						</div>
						<a
							href="/admin/screens/create"
							className="inline-block bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-medium px-4 py-2 rounded-lg transition-colors"
						>
							+ New Screen
						</a>
					</div>
				</div>
			)}

			{/* Toolbar */}
			<div className="absolute top-3 left-3 z-10 flex items-center gap-2">
				<a
					href="/admin/screens/create"
					className="text-xs bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-1.5 rounded-lg font-medium transition-colors"
				>
					+ New Screen
				</a>
				<button
					onClick={() => {
						// React Flow fitView is not directly accessible here; handled by fitView prop
						window.dispatchEvent(new Event("rf:fitview"));
					}}
					className="text-xs bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 px-3 py-1.5 rounded-lg font-medium transition-colors"
				>
					Fit View
				</button>
			</div>

			{/* Legend */}
			<div className="absolute bottom-16 right-3 z-10 bg-slate-900/90 backdrop-blur border border-slate-800 rounded-xl p-3 text-xs space-y-1.5">
				<div className="text-slate-500 font-medium mb-2 uppercase tracking-wider text-[10px]">Screen Types</div>
				{[
					{ label: "Initial",  color: "bg-slate-500" },
					{ label: "Menu",     color: "bg-amber-500" },
					{ label: "Input",    color: "bg-blue-500" },
					{ label: "Function", color: "bg-emerald-500" },
					{ label: "Router",   color: "bg-orange-500" },
					{ label: "Quit",     color: "bg-red-500" },
				].map(t => (
					<div key={t.label} className="flex items-center gap-2">
						<span className={`w-2.5 h-2.5 rounded-full ${t.color} shrink-0`} />
						<span className="text-slate-400">{t.label}</span>
					</div>
				))}
			</div>

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
				style={{ background: "#020617" }}
			>
				<Controls className="!bg-slate-900 !border-slate-700 !rounded-lg [&>button]:!bg-slate-900 [&>button]:!text-slate-400 [&>button:hover]:!bg-slate-800 [&>button]:!border-slate-700" />
				<MiniMap
					className="!bg-slate-900 !border-slate-700"
					nodeColor={node => {
						const colorMap: Record<string, string> = {
							Initial: "#6b7280",
							Menu: "#f59e0b",
							Input: "#3b82f6",
							Function: "#10b981",
							Router: "#f97316",
							Quit: "#ef4444",
						};
						const screenType = (node.data as any)?.screen?.screen_type;
						return colorMap[screenType] ?? "#475569";
					}}
				/>
				<Background variant={BackgroundVariant.Cross} gap={16} size={1} color="#1e293b" />
			</ReactFlow>
		</div>
	);
}

