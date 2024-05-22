"use client";

import React from "react";
import type { EdgeProps } from "reactflow";

export type CustomScreenEdgeData = {
	edge: { id: string };
};

export function CustomEdge({ id, data }: EdgeProps<CustomScreenEdgeData>) {
	const edge = data?.edge || { id: "" };

	return (
		<div className="react-flow__edge-default">
			<div>
				<h1>{edge.id}</h1>
			</div>
		</div>
	);
}
