import { SideNavLinks } from "@/constants/links";
import Link from "next/link";

export default function SideBar({
	onSelect,
}: {
	onSelect: (screen: string) => void;
}) {
	return (
		<div className="flex flex-col items-center bg-gray-900 justify-evenly w-40 fixed h-full">
			{SideNavLinks({ onSelect }).map((link, index) => (
				<NavBarItem key={index} href={link.href} onClick={link.onClick}>
					{link.children}
				</NavBarItem>
			))}
		</div>
	);
}

function NavBarItem({
	href,
	onClick,
	children,
}: {
	href: string;
	onClick: () => void;
	children: React.ReactNode;
}) {
	return (
		<Link
			href={href}
			onClick={onClick}
			className="text-base font-medium hover:text-gray-700 cursor-pointer"
		>
			{children}
		</Link>
	);
}
