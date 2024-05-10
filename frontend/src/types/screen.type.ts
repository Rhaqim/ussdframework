export interface MenuItem {
	id?: number;
	screen_id: number;
	name: string;
	option: string;
	display_name: string;
	next_screen: string;
}

export interface RouterOption {
	id?: number;
	screen_id: number;
	router_option: string;
	next_screen: string;
}

declare interface Screen {
	id?: number;
	name: string;
	text: string;
	screen_type: ScreenType;
	default_next_screen: string;
	service_code?: string;
	menu_items?: { [key: string]: MenuItem };
	function?: string;
	router_options?: RouterOption[];
	input_identifier?: string;
	input_type?: string;
	// Additional fields based on screen type
}

export enum ScreenType {
	INITIAL = "Initial",
	MENU = "Menu",
	INPUT = "Input",
	FUNCTION = "Function",
	ROUTER = "Router",
	QUIT = "Quit",
}

export default Screen;
