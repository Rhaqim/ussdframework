"use client";

import React from "react";
import { Handle, Position } from "reactflow";
import type { NodeProps } from "reactflow";

import Screen from "@/types/screen.type";

export type CustomScreenNodeData = {
	screen: Screen;
};

export function CustomNode({ data }: NodeProps<CustomScreenNodeData>) {
	const screen = data.screen;

	return (
		<div
			className="react-flow__node-default"
			style={{
				border: "1px solid black",
				padding: "10px",
				borderRadius: "5px",
			}}
		>
			<div>
				<h1>{screen.name}</h1>
				<div>
					<p>{screen.text}</p>
					<p>
						{screen.default_next_screen
							? `Default next screen: ${screen.default_next_screen}`
							: ""}
					</p>
				</div>
			</div>
			<Handle type="target" position={Position.Top} />
			<Handle type="source" position={Position.Bottom} />
		</div>
	);
}
