export const SideNavLinks = ({
	onSelect,
}: {
	onSelect: (screen: string) => void;
}) => [
	{
		href: "/admin/screens",
		onClick: () => onSelect("screens"),
		children: "Screens",
	},
	{
		href: "/admin/services",
		onClick: () => onSelect("services"),
		children: "Services",
	},
	{
		href: "/admin/build",
		onClick: () => onSelect("build"),
		children: "Build",
	},
	{
		href: "/admin/migration",
		onClick: () => onSelect("migrate"),
		children: "Migrate",
	},
	{
		href: "/admin/export",
		onClick: () => onSelect("export"),
		children: "Export",
	},
];
