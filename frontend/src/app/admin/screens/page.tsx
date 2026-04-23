"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";
import { Screens } from "@/api/route";
import Screen, { ScreenType } from "@/types/screen.type";
import { LinkButton } from "@/components/UI/Button";

const TYPE_COLORS: Record<string, string> = {
	Initial:  "bg-slate-700/60 text-slate-300",
	Menu:     "bg-amber-600/20 text-amber-400",
	Input:    "bg-blue-600/20 text-blue-400",
	Function: "bg-emerald-600/20 text-emerald-400",
	Router:   "bg-orange-600/20 text-orange-400",
	Quit:     "bg-red-600/20 text-red-400",
};

const ALL_TYPES = ["All", ...Object.values(ScreenType)];

export default function ScreensPage() {
	const [screens, setScreens] = useState<Screen[]>([]);
	const [loading, setLoading] = useState(true);
	const [search, setSearch] = useState("");
	const [typeFilter, setTypeFilter] = useState("All");

	useEffect(() => {
		Screens.getAll()
			.then(setScreens)
			.catch(() => {})
			.finally(() => setLoading(false));
	}, []);

	const filtered = screens.filter(s => {
		const matchSearch =
			s.name.toLowerCase().includes(search.toLowerCase()) ||
			s.text.toLowerCase().includes(search.toLowerCase());
		const matchType = typeFilter === "All" || s.screen_type === typeFilter;
		return matchSearch && matchType;
	});

	return (
		<div className="space-y-5">
			{/* Toolbar */}
			<div className="flex flex-wrap items-center gap-3">
				<input
					type="text"
					placeholder="Search screens…"
					value={search}
					onChange={e => setSearch(e.target.value)}
					className="bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-100 placeholder-slate-500 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500/30 transition-colors w-56"
				/>
				<div className="flex flex-wrap gap-1.5">
					{ALL_TYPES.map(t => (
						<button
							key={t}
							onClick={() => setTypeFilter(t)}
							className={`text-xs px-3 py-1.5 rounded-md font-medium transition-colors border ${
								typeFilter === t
									? "bg-indigo-600 text-white border-indigo-600"
									: "bg-transparent text-slate-400 border-slate-700 hover:text-slate-200 hover:border-slate-600"
							}`}
						>
							{t}
						</button>
					))}
				</div>
				<div className="ml-auto">
					<LinkButton href="/admin/screens/create" size="sm">
						+ Create Screen
					</LinkButton>
				</div>
			</div>

			{/* Table */}
			<div className="bg-slate-900 border border-slate-800 rounded-xl overflow-hidden">
				{loading ? (
					<div className="px-5 py-12 text-center text-sm text-slate-500">Loading…</div>
				) : (
					<table className="w-full">
						<thead>
							<tr className="border-b border-slate-800">
								{["Name", "Type", "Display Text", "Next Screen", ""].map(h => (
									<th
										key={h}
										className="px-5 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider"
									>
										{h}
									</th>
								))}
							</tr>
						</thead>
						<tbody className="divide-y divide-slate-800">
							{filtered.length === 0 ? (
								<tr>
									<td colSpan={5} className="px-5 py-10 text-center text-sm text-slate-500">
										{screens.length === 0 ? (
											<>
												No screens yet.{" "}
												<Link href="/admin/screens/create" className="text-indigo-400 hover:underline">
													Create one
												</Link>
											</>
										) : (
											"No matching screens."
										)}
									</td>
								</tr>
							) : (
								filtered.map(s => (
									<tr key={s.name} className="hover:bg-slate-800/50 transition-colors">
										<td className="px-5 py-3.5 text-sm font-medium text-slate-200">{s.name}</td>
										<td className="px-5 py-3.5">
											<span className={`text-xs px-2 py-0.5 rounded-full font-medium ${TYPE_COLORS[s.screen_type] ?? "bg-slate-700 text-slate-300"}`}>
												{s.screen_type}
											</span>
										</td>
										<td className="px-5 py-3.5 text-sm text-slate-400 max-w-xs truncate">{s.text}</td>
										<td className="px-5 py-3.5 text-sm text-slate-500">{s.default_next_screen || "—"}</td>
										<td className="px-5 py-3.5 text-right">
											<Link
												href={`/admin/screens/${s.name}`}
												className="text-xs text-indigo-400 hover:text-indigo-300 transition-colors"
											>
												Edit
											</Link>
										</td>
									</tr>
								))
							)}
						</tbody>
					</table>
				)}
			</div>
			<div className="text-xs text-slate-600">
				{filtered.length} of {screens.length} screen{screens.length !== 1 ? "s" : ""}
			</div>
		</div>
	);
}

