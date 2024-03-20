declare interface MenuItem {
	text: string;
	// other properties
}

export interface RouterOption {
	// router option properties
}

export interface Screen {
	text: string;
	screen_type: string;
	default_next_screen: string;
	service_code?: string;
	menu_items?: { [key: string]: MenuItem };
	function?: string;
	router_options?: RouterOption[];
	input_identifier?: string;
	input_type?: string;
	// Additional fields based on screen type
}

export default Screen;