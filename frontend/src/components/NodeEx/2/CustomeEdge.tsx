"use client";

import React, { memo } from "react";
import type { EdgeProps, EdgeTypes } from "reactflow";

export type CustomScreenEdgeData = {
	edge: { id: string };
};

// export const CustomEdge = memo(function CustomScreenEdge({
// 	id,
// 	data,
// }: EdgeProps<CustomScreenEdgeData>) {
// 	const edge = data?.edge || { id: "" };

// 	return (
// 		<div className="react-flow__edge-default">
// 			<div>
// 				<h1>{edge.id}</h1>
// 			</div>
// 		</div>
// 	);
// });

export function CustomEdge({
	id,
	data,
}: EdgeProps<CustomScreenEdgeData>) {
	const edge = data?.edge || { id: "" };

	return (
		<div className="react-flow__edge-default">
			<div>
				<h1>{edge.id}</h1>
			</div>
		</div>
	);
}


