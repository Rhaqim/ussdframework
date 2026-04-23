import type { Edge } from "reactflow";

import Screen from "@/types/screen.type";

export const initialEdges = (screens: Screen[]): Edge[] =>
	screens.flatMap(screen => {
		const edges: Edge[] = [];

		// Default next screen (guard against empty/undefined)
		if (screen.default_next_screen) {
			edges.push({
				id: `default-${screen.name}->${screen.default_next_screen}`,
				source: screen.name,
				target: screen.default_next_screen,
				sourceHandle: `source-${screen.name}`,
				targetHandle: `target-${screen.default_next_screen}`,
				animated: true,
				type: "custom",
				data: { label: "Next" },
			});
		}

		// Menu item edges
		if (screen.menu_items) {
			screen.menu_items.forEach(item => {
				if (!item.next_screen) return;
				edges.push({
					id: `menu-${screen.name}-${item.option}->${item.next_screen}`,
					source: screen.name,
					target: item.next_screen,
					sourceHandle: `source-${screen.name}`,
					targetHandle: `target-${item.next_screen}`,
					animated: true,
					type: "custom",
					data: { label: `${item.option}. ${item.display_name}` },
				});
			});
		}

		// Router option edges
		if (screen.router_options) {
			screen.router_options.forEach((option, idx) => {
				if (!option.next_screen) return;
				edges.push({
					id: `router-${screen.name}-${idx}->${option.next_screen}`,
					source: screen.name,
					target: option.next_screen,
					sourceHandle: `source-${screen.name}`,
					targetHandle: `target-${option.next_screen}`,
					animated: true,
					type: "custom",
					data: { label: option.router_option },
				});
			});
		}

		return edges;
	});
