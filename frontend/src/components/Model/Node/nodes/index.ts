import dagre from "@dagrejs/dagre";
import type { Node } from "reactflow";

import Screen, { ScreenType } from "@/types/screen.type";

const NODE_WIDTH = 240;
const NODE_HEIGHT = 120;

/**
 * Builds all edges (default, menu, router) from a set of screens and returns
 * the list of [source, target] pairs used by the Dagre layout.
 */
function buildEdgePairs(screens: Screen[]): [string, string][] {
	const pairs: [string, string][] = [];
	for (const screen of screens) {
		if (screen.default_next_screen) {
			pairs.push([screen.name, screen.default_next_screen]);
		}
		if (screen.menu_items) {
			for (const item of screen.menu_items) {
				if (item.next_screen) pairs.push([screen.name, item.next_screen]);
			}
		}
		if (screen.router_options) {
			for (const opt of screen.router_options) {
				if (opt.next_screen) pairs.push([screen.name, opt.next_screen]);
			}
		}
	}
	return pairs;
}

/**
 * Lays out screens using the Dagre graph library (top-down) and returns
 * React Flow Node objects with computed positions.
 */
export const initialNodes = (screens: Screen[]): Node[] => {
	const g = new dagre.graphlib.Graph();
	g.setDefaultEdgeLabel(() => ({}));
	g.setGraph({ rankdir: "TB", ranksep: 80, nodesep: 40 });

	// Add every screen as a Dagre node
	const screenMap = new Map(screens.map(s => [s.name, s]));
	for (const screen of screens) {
		g.setNode(screen.name, { width: NODE_WIDTH, height: NODE_HEIGHT });
	}

	// Add all connections so Dagre knows the hierarchy
	for (const [source, target] of buildEdgePairs(screens)) {
		if (screenMap.has(source) && screenMap.has(target)) {
			g.setEdge(source, target);
		}
	}

	dagre.layout(g);

	return screens.map(screen => {
		const node = g.node(screen.name);
		return {
			id: screen.name,
			type: "screen",
			position: {
				x: node ? node.x - NODE_WIDTH / 2 : 0,
				y: node ? node.y - NODE_HEIGHT / 2 : 0,
			},
			data: { screen },
		};
	});
};