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
