"use client";

import React from "react";
import { Handle, Position } from "reactflow";
import type { NodeProps } from "reactflow";

import Screen, { ScreenType } from "@/types/screen.type";

export type CustomScreenNodeData = {
	screen: Screen;
};

const SCREEN_TYPE_STYLES: Record<
	ScreenType,
	{ bg: string; text: string; label: string }
> = {
	[ScreenType.INITIAL]: {
		bg: "bg-gray-200",
		text: "text-gray-700",
		label: "Initial",
	},
	[ScreenType.MENU]: {
		bg: "bg-yellow-200",
		text: "text-yellow-800",
		label: "Menu",
	},
	[ScreenType.INPUT]: {
		bg: "bg-blue-200",
		text: "text-blue-800",
		label: "Input",
	},
	[ScreenType.FUNCTION]: {
		bg: "bg-green-200",
		text: "text-green-800",
		label: "Function",
	},
	[ScreenType.ROUTER]: {
		bg: "bg-orange-200",
		text: "text-orange-800",
		label: "Router",
	},
	[ScreenType.QUIT]: {
		bg: "bg-red-200",
		text: "text-red-800",
		label: "Quit",
	},
};

export function CustomNode({ data, selected }: NodeProps<CustomScreenNodeData>) {
	const screen = data.screen;
	const typeStyle =
		SCREEN_TYPE_STYLES[screen.screen_type] ?? SCREEN_TYPE_STYLES[ScreenType.INITIAL];

	const menuCount = screen.menu_items?.length ?? 0;
	const routerCount = screen.router_options?.length ?? 0;

	return (
		<div
			style={{ minWidth: 200, maxWidth: 240 }}
			className={`rounded-lg border-2 shadow-md bg-white transition-shadow ${
				selected ? "border-blue-500 shadow-blue-200" : "border-gray-300"
			}`}
		>
			{/* Header with type badge */}
			<div
				className={`flex items-center justify-between px-3 py-1.5 rounded-t-lg ${typeStyle.bg}`}
			>
				<span className={`text-xs font-semibold ${typeStyle.text}`}>
					{typeStyle.label}
				</span>
				{menuCount > 0 && (
					<span className="text-xs text-gray-500">{menuCount} options</span>
				)}
				{routerCount > 0 && (
					<span className="text-xs text-gray-500">{routerCount} routes</span>
				)}
			</div>

			{/* Body */}
			<div className="px-3 py-2">
				<p className="font-semibold text-sm text-gray-900 truncate">
					{screen.name}
				</p>
				{screen.text && (
					<p className="text-xs text-gray-500 mt-1 line-clamp-2">{screen.text}</p>
				)}
				{screen.default_next_screen && (
					<p className="text-xs text-gray-400 mt-1 truncate">
						→ {screen.default_next_screen}
					</p>
				)}
			</div>

			{/* Handles: target on top, source on bottom */}
			<Handle
				id={`target-${screen.name}`}
				type="target"
				position={Position.Top}
				className="w-2 h-2 bg-gray-400"
			/>
			<Handle
				id={`source-${screen.name}`}
				type="source"
				position={Position.Bottom}
				className="w-2 h-2 bg-blue-400"
			/>
		</div>
	);
}
