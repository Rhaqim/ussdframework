import type { Edge } from "reactflow";

import { MenuItems, RouterOptions } from "@/api/route";

import Screen, { MenuItem, RouterOption, ScreenType } from "@/types/screen.type";

export const initialEdges = (screens: Screen[]) =>
	screens.flatMap(screen => {
		const edges: Edge[] = [];

		const req = {
			"ScreenName": screen.name,
		}

		if (screen.screen_type === ScreenType.MENU) {
			MenuItems.getByQuery(req).then((items: MenuItem[]) => {
				console.log(`Fetched menu items for ${screen.name}: ${items[0]}`);
				screen.menu_items = items;
			});
		}
	
		if (screen.screen_type === ScreenType.ROUTER) {
			RouterOptions.getByQuery(req).then((options: RouterOption[]) => {
				console.log(`Fetched router options for ${screen.name}: ${options}`);
				screen.router_options = options;
			});

		}

		// Handle all screens
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
		// Handle menu_items
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
		// handle router_options
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
