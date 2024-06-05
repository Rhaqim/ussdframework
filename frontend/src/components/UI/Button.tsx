import React from "react";
import Link from "next/link";

interface ButtonProps {
	onClick?: () => void;
	className?: string;
	children: React.ReactNode;
}

const Button: React.FC<ButtonProps> = ({ onClick, className, children }) => {
	return (
		<button
			onClick={onClick}
			className={`bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ${className}`}
		>
			{children}
		</button>
	);
};

export const LinkButton: React.FC<ButtonProps & { href: string }> = ({
	href,
	onClick,
	className,
	children,
}) => {
	return (
		<Link href={href}>
			<p
				onClick={onClick}
				className={`bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-md ${className}`}
			>
				{children}
			</p>
		</Link>
	);
}

export default Button;
