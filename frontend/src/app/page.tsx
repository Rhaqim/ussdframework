import React from "react";
import Link from "next/link";

const sections = [
	{
		title: "Screens",
		description: "Create and manage the USSD screens that make up your menu flow.",
		href: "/admin/screens",
		icon: (
			<svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
				<rect x="2" y="3" width="20" height="14" rx="2" />
				<path d="M8 21h8M12 17v4" />
			</svg>
		),
		color: "text-indigo-400",
		ring: "group-hover:border-indigo-600/40",
		bg: "group-hover:bg-indigo-600/5",
		iconBg: "bg-indigo-600/15 group-hover:bg-indigo-600/25",
	},
	{
		title: "Menu Items",
		description: "Define the numbered options displayed on menu screens.",
		href: "/admin/menu_items",
		icon: (
			<svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
				<path d="M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01" />
			</svg>
		),
		color: "text-amber-400",
		ring: "group-hover:border-amber-600/40",
		bg: "group-hover:bg-amber-600/5",
		iconBg: "bg-amber-600/15 group-hover:bg-amber-600/25",
	},
	{
		title: "Router Options",
		description: "Set up conditional routing rules between screens.",
		href: "/admin/router_options",
		icon: (
			<svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
				<path d="M4 6h16M4 12h10M4 18h7" />
				<polyline points="15 9 18 12 15 15" />
			</svg>
		),
		color: "text-orange-400",
		ring: "group-hover:border-orange-600/40",
		bg: "group-hover:bg-orange-600/5",
		iconBg: "bg-orange-600/15 group-hover:bg-orange-600/25",
	},
	{
		title: "Services",
		description: "Connect external services invoked by function-type screens.",
		href: "/admin/services",
		icon: (
			<svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
				<path d="M21 16V8a2 2 0 00-1-1.73l-7-4a2 2 0 00-2 0l-7 4A2 2 0 003 8v8a2 2 0 001 1.73l7 4a2 2 0 002 0l7-4A2 2 0 0021 16z" />
			</svg>
		),
		color: "text-emerald-400",
		ring: "group-hover:border-emerald-600/40",
		bg: "group-hover:bg-emerald-600/5",
		iconBg: "bg-emerald-600/15 group-hover:bg-emerald-600/25",
	},
	{
		title: "Import / Export",
		description: "Upload a JSON menu definition or download the current one.",
		href: "/admin/export",
		icon: (
			<svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
				<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
			</svg>
		),
		color: "text-sky-400",
		ring: "group-hover:border-sky-600/40",
		bg: "group-hover:bg-sky-600/5",
		iconBg: "bg-sky-600/15 group-hover:bg-sky-600/25",
	},
	{
		title: "Visual Editor",
		description: "View and edit the full USSD flow as an interactive graph.",
		href: "/admin/menu_nodes",
		icon: (
			<svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
				<circle cx="4" cy="12" r="2" /><circle cx="20" cy="5" r="2" /><circle cx="20" cy="19" r="2" />
				<path d="M6 12h5l5-5.5M11 12l5 5.5" />
			</svg>
		),
		color: "text-violet-400",
		ring: "group-hover:border-violet-600/40",
		bg: "group-hover:bg-violet-600/5",
		iconBg: "bg-violet-600/15 group-hover:bg-violet-600/25",
	},
];

export default function Home() {
	return (
		<div className="min-h-screen bg-slate-950 text-slate-200 flex flex-col">
			{/* Hero */}
			<header className="border-b border-slate-800 bg-slate-950/80 backdrop-blur-sm sticky top-0 z-10">
				<div className="max-w-5xl mx-auto px-6 py-4 flex items-center justify-between">
					<div className="flex items-center gap-2.5">
						<div className="w-7 h-7 rounded-md bg-indigo-600 flex items-center justify-center">
							<svg className="w-4 h-4 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
								<path d="M22 16.92v3a2 2 0 01-2.18 2 19.79 19.79 0 01-8.63-3.07 19.5 19.5 0 01-6-6A19.79 19.79 0 012.12 4.18 2 2 0 014.11 2h3a2 2 0 012 1.72c.127.96.361 1.903.7 2.81a2 2 0 01-.45 2.11L8.09 9.91a16 16 0 006 6l1.27-1.27a2 2 0 012.11-.45c.907.339 1.85.573 2.81.7A2 2 0 0122 16.92z" />
							</svg>
						</div>
						<span className="text-sm font-semibold tracking-tight">USSD Framework</span>
					</div>
					<Link
						href="/admin"
						className="text-xs font-medium px-3 py-1.5 rounded-md bg-indigo-600 hover:bg-indigo-500 text-white transition-colors"
					>
						Dashboard →
					</Link>
				</div>
			</header>

			<main className="flex-1 max-w-5xl mx-auto w-full px-6 py-16 space-y-16">
				{/* Hero text */}
				<section className="text-center space-y-4">
					<div className="inline-flex items-center gap-2 text-xs font-medium px-3 py-1 rounded-full border border-indigo-600/30 bg-indigo-600/10 text-indigo-400 mb-2">
						MenuBuilder Portal
					</div>
					<h1 className="text-4xl font-bold tracking-tight text-white">
						Build USSD menus without writing code
					</h1>
					<p className="text-slate-400 max-w-2xl mx-auto text-base leading-relaxed">
						Design screens, define navigation flows, connect services, and export your
						menu definition — all from a single interface.
					</p>
					<div className="flex items-center justify-center gap-3 pt-2">
						<Link
							href="/admin"
							className="px-4 py-2 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-medium transition-colors"
						>
							Open Dashboard
						</Link>
						<Link
							href="/admin/screens/create"
							className="px-4 py-2 rounded-lg border border-slate-700 hover:border-slate-600 text-slate-300 hover:text-white text-sm font-medium transition-colors"
						>
							Create a screen
						</Link>
					</div>
				</section>

				{/* Feature grid */}
				<section>
					<h2 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-4">
						What you can do
					</h2>
					<div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3">
						{sections.map((s) => (
							<Link key={s.href} href={s.href} className="group block">
								<div
									className={`h-full bg-slate-900 border border-slate-800 rounded-xl p-5 transition-all ${s.ring} ${s.bg}`}
								>
									<div
										className={`w-9 h-9 rounded-lg flex items-center justify-center mb-4 ${s.iconBg} ${s.color} transition-colors`}
									>
										{s.icon}
									</div>
									<div className="text-sm font-medium text-slate-200 mb-1">{s.title}</div>
									<div className="text-xs text-slate-500 leading-relaxed">{s.description}</div>
								</div>
							</Link>
						))}
					</div>
				</section>

				{/* How it works */}
				<section>
					<h2 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-4">
						How it works
					</h2>
					<div className="grid grid-cols-1 md:grid-cols-3 gap-4">
						{[
							{ step: "1", title: "Define screens", body: "Create Initial, Menu, Input, Function, Router, and Quit screens to model your USSD flow." },
							{ step: "2", title: "Link them together", body: "Set default next screens, menu options, and router conditions to connect everything." },
							{ step: "3", title: "Test & export", body: "POST requests to /ussd and use the export page to generate a portable JSON definition." },
						].map(({ step, title, body }) => (
							<div key={step} className="bg-slate-900 border border-slate-800 rounded-xl p-5">
								<div className="w-7 h-7 rounded-full bg-slate-800 text-slate-400 text-xs font-bold flex items-center justify-center mb-3">
									{step}
								</div>
								<div className="text-sm font-medium text-slate-200 mb-1">{title}</div>
								<div className="text-xs text-slate-500 leading-relaxed">{body}</div>
							</div>
						))}
					</div>
				</section>
			</main>

			<footer className="border-t border-slate-800 py-6 text-center text-xs text-slate-600">
				USSD Framework — MenuBuilder Portal
			</footer>
		</div>
	);
}
