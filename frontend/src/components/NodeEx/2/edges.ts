import type { Edge, EdgeTypes } from "reactflow";

import { CustomEdge } from "./CustomeEdge";
import Screen from "@/types/screen.type";

export async function fetchScreens(): Promise<Screen[]> {
	const response = await fetch("/api/screens");
	if (!response.ok) {
		throw new Error("Failed to fetch screens");
	}
	const screens: Screen[] = await response.json();
	return screens;
}

// const screens: Screen[] = await fetchScreens(); // Assume fetchScreens is a function that fetches screen data

export const initialEdges = (screens: Screen[]) =>
	screens.flatMap(screen => {
		const edges: Edge[] = [];
		if (screen.default_next_screen) {
			edges.push({
				id: `${screen.name}->${screen.default_next_screen}`,
				source: screen.name,
				target: screen.default_next_screen,
				animated: true,
				label: "Next",
				type: "custom",
			});
		}

		// map through screen.menu_items object and for the value get the next screen
		if (screen.menu_items) {
			screen.menu_items.forEach(item => {
				edges.push({
					id: `${screen.name}->${item.next_screen}`,
					source: screen.name,
					target: item.next_screen,
					animated: true,
					label: item.display_name,
					type: "custom",
				});
			});
		}
		if (screen.router_options) {
			screen.router_options.forEach(option => {
				if (option.next_screen) {
					edges.push({
						id: `${screen.name}->${option.next_screen}`,
						source: screen.name,
						target: option.next_screen,
						animated: true,
						label: "Next",
						type: "custom",
					});
				}
			});
		}
		return edges;
	}) satisfies Edge[];

export const edgeTypes = {
	custom: CustomEdge,
} satisfies EdgeTypes;
