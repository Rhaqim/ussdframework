import React from "react";
import Link from "next/link";

interface ButtonProps {
	onClick?: () => void;
	className?: string;
	children: React.ReactNode;
	variant?: "primary" | "secondary" | "danger" | "ghost";
	size?: "sm" | "md";
	disabled?: boolean;
	type?: "button" | "submit" | "reset";
}

const VARIANTS = {
	primary: "bg-indigo-600 hover:bg-indigo-700 text-white border border-transparent",
	secondary: "bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700",
	danger: "bg-red-600/10 hover:bg-red-600/20 text-red-400 border border-red-600/30",
	ghost: "bg-transparent hover:bg-slate-800 text-slate-400 hover:text-slate-200 border border-transparent",
};
const SIZES = {
	sm: "text-xs px-3 py-1.5 rounded-md",
	md: "text-sm px-4 py-2 rounded-lg",
};

const Button: React.FC<ButtonProps> = ({
	onClick,
	className = "",
	children,
	variant = "primary",
	size = "md",
	disabled,
	type = "button",
}) => (
	<button
		type={type}
		onClick={onClick}
		disabled={disabled}
		className={`font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${VARIANTS[variant]} ${SIZES[size]} ${className}`}
	>
		{children}
	</button>
);

export const LinkButton: React.FC<ButtonProps & { href: string }> = ({
	href,
	className = "",
	children,
	variant = "primary",
	size = "md",
}) => (
	<Link
		href={href}
		className={`inline-block font-medium transition-colors ${VARIANTS[variant]} ${SIZES[size]} ${className}`}
	>
		{children}
	</Link>
);

export default Button;
