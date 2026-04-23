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
	{ accent: string; badge: string; badgeText: string; dot: string; label: string }
> = {
	[ScreenType.INITIAL]: {
		accent: "border-slate-500",
		badge: "bg-slate-800 border-slate-600",
		badgeText: "text-slate-300",
		dot: "bg-slate-400",
		label: "Initial",
	},
	[ScreenType.MENU]: {
		accent: "border-amber-500",
		badge: "bg-amber-950 border-amber-700",
		badgeText: "text-amber-400",
		dot: "bg-amber-400",
		label: "Menu",
	},
	[ScreenType.INPUT]: {
		accent: "border-blue-500",
		badge: "bg-blue-950 border-blue-700",
		badgeText: "text-blue-400",
		dot: "bg-blue-400",
		label: "Input",
	},
	[ScreenType.FUNCTION]: {
		accent: "border-emerald-500",
		badge: "bg-emerald-950 border-emerald-700",
		badgeText: "text-emerald-400",
		dot: "bg-emerald-400",
		label: "Function",
	},
	[ScreenType.ROUTER]: {
		accent: "border-orange-500",
		badge: "bg-orange-950 border-orange-700",
		badgeText: "text-orange-400",
		dot: "bg-orange-400",
		label: "Router",
	},
	[ScreenType.QUIT]: {
		accent: "border-red-500",
		badge: "bg-red-950 border-red-700",
		badgeText: "text-red-400",
		dot: "bg-red-400",
		label: "Quit",
	},
};

export function CustomNode({ data, selected }: NodeProps<CustomScreenNodeData>) {
	const screen = data.screen;
	const ts =
		SCREEN_TYPE_STYLES[screen.screen_type] ?? SCREEN_TYPE_STYLES[ScreenType.INITIAL];

	const menuCount = screen.menu_items?.length ?? 0;
	const routerCount = screen.router_options?.length ?? 0;

	return (
		<div
			style={{ minWidth: 200, maxWidth: 240 }}
			className={`rounded-xl border-2 shadow-xl bg-slate-900 transition-all ${
				selected
					? `${ts.accent} ring-2 ring-offset-1 ring-offset-slate-900 ring-current shadow-[0_0_12px_2px_rgba(99,102,241,0.3)]`
					: "border-slate-700 hover:border-slate-500"
			}`}
		>
			{/* Coloured top stripe */}
			<div className={`h-1 rounded-t-xl ${ts.dot}`} />

			{/* Header */}
			<div className="flex items-center justify-between px-3 pt-2 pb-1">
				<span
					className={`inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full border text-[10px] font-semibold ${
						ts.badge
					} ${ts.badgeText}`}
				>
					<span className={`w-1.5 h-1.5 rounded-full ${ts.dot}`} />
					{ts.label}
				</span>

				{menuCount > 0 && (
					<span className="text-[10px] text-slate-500">{menuCount} opts</span>
				)}
				{routerCount > 0 && (
					<span className="text-[10px] text-slate-500">{routerCount} routes</span>
				)}
			</div>

			{/* Body */}
			<div className="px-3 pb-3">
				<p className="font-semibold text-sm text-slate-100 truncate">
					{screen.name}
				</p>
				{screen.text && (
					<p className="text-xs text-slate-400 mt-0.5 line-clamp-2 leading-snug">
						{screen.text}
					</p>
				)}
				{screen.default_next_screen && (
					<p className="text-[10px] text-slate-500 mt-1 truncate">
						→ {screen.default_next_screen}
					</p>
				)}
			</div>

			{/* Handles */}
			<Handle
				id={`target-${screen.name}`}
				type="target"
				position={Position.Top}
				className="!w-2 !h-2 !bg-slate-500 !border-slate-700"
			/>
			<Handle
				id={`source-${screen.name}`}
				type="source"
				position={Position.Bottom}
				className="!w-2 !h-2 !bg-slate-500 !border-slate-700"
			/>
		</div>
	);
}
