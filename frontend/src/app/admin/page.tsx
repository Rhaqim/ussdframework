"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";
import { Screens, Services, MenuItems, RouterOptions } from "@/api/route";

/* ─── Stat card ─────────────────────────────────────────────────── */
function StatCard({
	label,
	value,
	href,
	color,
}: {
	label: string;
	value: number;
	href: string;
	color: string;
}) {
	return (
		<Link href={href} className="block">
			<div className="bg-slate-900 border border-slate-800 rounded-xl p-5 hover:border-slate-700 transition-colors group">
				<div className={`text-2xl font-bold mb-1 ${color}`}>{value}</div>
				<div className="text-sm text-slate-400 group-hover:text-slate-300 transition-colors">
					{label}
				</div>
			</div>
		</Link>
	);
}

/* ─── Quick action card ─────────────────────────────────────────── */
function ActionCard({
	href,
	label,
	description,
	icon,
}: {
	href: string;
	label: string;
	description: string;
	icon: React.ReactNode;
}) {
	return (
		<Link href={href} className="block">
			<div className="bg-slate-900 border border-slate-800 rounded-xl p-4 hover:border-indigo-600/30 hover:bg-indigo-600/5 transition-all group flex items-start gap-4">
				<div className="w-9 h-9 bg-indigo-600/15 rounded-lg flex items-center justify-center text-indigo-400 shrink-0 group-hover:bg-indigo-600/25 transition-colors">
					{icon}
				</div>
				<div>
					<div className="text-sm font-medium text-slate-200 mb-0.5">{label}</div>
					<div className="text-xs text-slate-500">{description}</div>
				</div>
			</div>
		</Link>
	);
}

const SCREEN_TYPE_COLORS: Record<string, string> = {
	Initial: "bg-slate-700 text-slate-200",
	Menu: "bg-amber-600/20 text-amber-400",
	Input: "bg-blue-600/20 text-blue-400",
	Function: "bg-emerald-600/20 text-emerald-400",
	Router: "bg-orange-600/20 text-orange-400",
	Quit: "bg-red-600/20 text-red-400",
};

export default function AdminHomePage() {
	const [counts, setCounts] = useState({ screens: 0, services: 0, menuItems: 0, routerOptions: 0 });
	const [recentScreens, setRecentScreens] = useState<any[]>([]);
	const [loading, setLoading] = useState(true);

	useEffect(() => {
		const load = async () => {
			try {
				const [screens, services, menuItems, routerOptions] = await Promise.all([
					Screens.getAll().catch(() => []),
					Services.getAll().catch(() => []),
					MenuItems.getAll().catch(() => []),
					RouterOptions.getAll().catch(() => []),
				]);
				setCounts({
					screens: screens.length,
					services: services.length,
					menuItems: menuItems.length,
					routerOptions: routerOptions.length,
				});
				setRecentScreens(screens.slice(-6).reverse());
			} finally {
				setLoading(false);
			}
		};
		load();
	}, []);

	return (
		<div className="max-w-5xl mx-auto space-y-8">
			{/* Stats */}
			<section>
				<h2 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3">
					Overview
				</h2>
				<div className="grid grid-cols-2 md:grid-cols-4 gap-4">
					<StatCard label="Screens" value={counts.screens} href="/admin/screens" color="text-indigo-400" />
					<StatCard label="Services" value={counts.services} href="/admin/services" color="text-emerald-400" />
					<StatCard label="Menu Items" value={counts.menuItems} href="/admin/menu_items" color="text-amber-400" />
					<StatCard label="Router Options" value={counts.routerOptions} href="/admin/router_options" color="text-orange-400" />
				</div>
			</section>

			{/* Quick actions */}
			<section>
				<h2 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3">
					Quick Actions
				</h2>
				<div className="grid grid-cols-1 md:grid-cols-3 gap-3">
					<ActionCard
						href="/admin/menu_nodes"
						label="Visual Editor"
						description="View and edit the USSD flow graph"
						icon={
							<svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
								<circle cx="4" cy="12" r="2" /><circle cx="20" cy="5" r="2" /><circle cx="20" cy="19" r="2" />
								<path d="M6 12h5l5-5.5M11 12l5 5.5" />
							</svg>
						}
					/>
					<ActionCard
						href="/admin/screens/create"
						label="Create Screen"
						description="Add a new USSD screen to the flow"
						icon={
							<svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
								<rect x="2" y="3" width="20" height="14" rx="2" /><path d="M12 10v4M10 12h4" />
							</svg>
						}
					/>
					<ActionCard
						href="/admin/export"
						label="Import / Export"
						description="Export to or import from JSON"
						icon={
							<svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.75}>
								<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
							</svg>
						}
					/>
				</div>
			</section>

			{/* Recent screens */}
			<section>
				<div className="flex items-center justify-between mb-3">
					<h2 className="text-xs font-semibold text-slate-500 uppercase tracking-wider">
						Recent Screens
					</h2>
					<Link href="/admin/screens" className="text-xs text-indigo-400 hover:text-indigo-300 transition-colors">
						View all →
					</Link>
				</div>
				<div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden">
					{loading ? (
						<div className="px-5 py-8 text-center text-sm text-slate-500">Loading…</div>
					) : recentScreens.length === 0 ? (
						<div className="px-5 py-8 text-center text-sm text-slate-500">
							No screens yet.{" "}
							<Link href="/admin/screens/create" className="text-indigo-400 hover:underline">
								Create one
							</Link>
						</div>
					) : (
						<table className="w-full">
							<thead>
								<tr className="border-b border-slate-800">
									<th className="px-5 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Name</th>
									<th className="px-5 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Type</th>
									<th className="px-5 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Next Screen</th>
									<th className="px-5 py-3 text-right text-xs font-medium text-slate-500 uppercase tracking-wider">Action</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-slate-800">
								{recentScreens.map((s: any) => (
									<tr key={s.name} className="hover:bg-slate-800/50 transition-colors">
										<td className="px-5 py-3.5 text-sm font-medium text-slate-200">{s.name}</td>
										<td className="px-5 py-3.5">
											<span className={`text-xs px-2 py-0.5 rounded-full font-medium ${SCREEN_TYPE_COLORS[s.screen_type] ?? "bg-slate-700 text-slate-300"}`}>
												{s.screen_type}
											</span>
										</td>
										<td className="px-5 py-3.5 text-sm text-slate-400">{s.default_next_screen ?? "—"}</td>
										<td className="px-5 py-3.5 text-right">
											<Link href={`/admin/screens/${s.name}`} className="text-xs text-indigo-400 hover:text-indigo-300 transition-colors">
												Edit
											</Link>
										</td>
									</tr>
								))}
							</tbody>
						</table>
					)}
				</div>
			</section>
		</div>
	);
}

