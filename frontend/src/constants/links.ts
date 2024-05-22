export const SideNavLinks = ({
	onSelect,
}: {
	onSelect: (screen: string) => void;
}) => [
	{
		href: "/admin/menu_nodes",
		onClick: () => onSelect("menu_nodes"),
		children: "Nodes",
	},
	{
		href: "/admin/screens",
		onClick: () => onSelect("screens"),
		children: "Screens",
	},
	{
		href: "/admin/router_options",
		onClick: () => onSelect("router_options"),
		children: "Router Options",
	},
	{
		href: "/admin/menu_items",
		onClick: () => onSelect("menu_items"),
		children: "Menu Items",
	},
	{
		href: "/admin/services",
		onClick: () => onSelect("services"),
		children: "Services",
	},
	{
		href: "/admin/migrate",
		onClick: () => onSelect("migrate"),
		children: "Migrate",
	},
	{
		href: "/admin/export",
		onClick: () => onSelect("export"),
		children: "Export",
	},
];
