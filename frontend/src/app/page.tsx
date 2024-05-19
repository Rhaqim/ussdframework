import React from "react";
import Link from "next/link";

const homeLinks = [
	{
		title: "Screens",
		description: "Manage USSD Screens",
		href: "/admin/screens",
		icon: "M12 14l9-5-9-5 9 5z",
		color: "blue",
	},
	{
		title: "Services",
		description: "Manage USSD Services",
		href: "/admin/services",
		icon: "M12 14l9-5-9-5 9 5z",
		color: "blue",
	},
	{
		title: "Migrate",
		description: "Migrate USSD Data",
		href: "/admin/migrate",
		icon: "M12 14l9-5-9-5 9 5z",
		color: "blue",
	},
	{
		title: "Export",
		description: "Export USSD Data",
		href: "/admin/export",
		icon: "M12 14l9-5-9-5 9 5z",
		color: "blue",
	},
];

export default function Home() {
	return (
		<main className="bg-black w-full flex min-h-screen p-4">
			<div className="w-full flex flex-col items-center justify-center font-mono text-sm">
				<div className="text-center">
					<h1>USSD Framework</h1>
					<h2>Welcome to the USSD framework MenuBuilder Portal</h2>
				</div>

				{/* Link Grid */}
				<div className="grid grid-cols-2 gap-4 mt-4">
					{homeLinks.map((link, index) => (
						<HomeLinkCard key={index} {...link} />
					))}
				</div>
			</div>
		</main>
	);
}

const HomeLinkCard = ({
	title,
	description,
	href,
	icon,
	color,
}: {
	title: string;
	description: string;
	href: string;
	icon: string;
	color: string;
}) => {
	return (
		<Link
			href={href}
			className="bg-gray-800 rounded-md p-4 flex flex-col items-center justify-center"
		>
			<div className="bg-gray-700 rounded-full p-4">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					className={`h-12 w-12 text-${color}-500`}
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						strokeLinecap="round"
						strokeLinejoin="round"
						strokeWidth={2}
						d={icon}
					/>
				</svg>
			</div>
			<h3 className="text-lg font-bold mt-4">{title}</h3>
			<p className="text-sm text-gray-400">{description}</p>
		</Link>
	);
};
