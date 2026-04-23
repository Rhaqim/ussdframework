"use client";

import React from "react";
import {
	BaseEdge,
	EdgeLabelRenderer,
	getBezierPath,
	type EdgeProps,
} from "reactflow";

export type CustomScreenEdgeData = {
	label?: string;
};

export function CustomEdge({
	id,
	sourceX,
	sourceY,
	targetX,
	targetY,
	sourcePosition,
	targetPosition,
	data,
	markerEnd,
	style,
}: EdgeProps<CustomScreenEdgeData>) {
	const [edgePath, labelX, labelY] = getBezierPath({
		sourceX,
		sourceY,
		sourcePosition,
		targetX,
		targetY,
		targetPosition,
	});

	return (
		<>
			<BaseEdge
				id={id}
				path={edgePath}
				markerEnd={markerEnd}
				style={{ stroke: "#475569", strokeWidth: 1.5, ...style }}
			/>
			{data?.label && (
				<EdgeLabelRenderer>
					<div
						style={{
							position: "absolute",
							transform: `translate(-50%, -50%) translate(${labelX}px,${labelY}px)`,
							pointerEvents: "all",
						}}
						className="nodrag nopan bg-slate-800 border border-slate-600 rounded px-1.5 py-0.5 text-[10px] text-slate-300 shadow-md"
					>
						{data.label}
					</div>
				</EdgeLabelRenderer>
			)}
		</>
	);
}
