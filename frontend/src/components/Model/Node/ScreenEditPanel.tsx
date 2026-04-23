"use client";

import React, { useEffect, useState } from "react";

import { Screens, MenuItems, RouterOptions } from "@/api/route";
import { useNav } from "@/context/navigation.context";
import Screen, { MenuItem, RouterOption, ScreenType } from "@/types/screen.type";

const LABEL: Record<string, string> = {
	name: "Name",
	text: "Text",
	screen_type: "Screen Type",
	default_next_screen: "Default Next Screen",
	service_code: "Service Code",
	function: "Function",
	input_identifier: "Input Identifier",
	input_type: "Input Type",
};

export default function ScreenEditPanel() {
	const { selectedScreen, setSelectedScreen } = useNav();
	const [screen, setScreen] = useState<Screen | null>(null);
	const [menuItems, setMenuItems] = useState<MenuItem[]>([]);
	const [routerOptions, setRouterOptions] = useState<RouterOption[]>([]);
	const [saving, setSaving] = useState(false);
	const [saved, setSaved] = useState(false);

	useEffect(() => {
		if (!selectedScreen) {
			setScreen(null);
			setMenuItems([]);
			setRouterOptions([]);
			return;
		}
		setScreen(null);
		setSaved(false);

		Screens.get(selectedScreen)
			.then((s: Screen) => {
				setScreen(s);
				const req = { ScreenName: s.name };
				if (s.screen_type === ScreenType.MENU) {
					MenuItems.getByQuery(req).then(setMenuItems).catch(() => setMenuItems([]));
				} else {
					setMenuItems([]);
				}
				if (s.screen_type === ScreenType.ROUTER) {
					RouterOptions.getByQuery(req).then(setRouterOptions).catch(() => setRouterOptions([]));
				} else {
					setRouterOptions([]);
				}
			})
			.catch(err => {
				console.error("Failed to load screen:", err);
				setScreen(null);
			});
	}, [selectedScreen]);

	const handleChange = (field: keyof Screen, value: string) => {
		if (!screen) return;
		setScreen({ ...screen, [field]: value });
	};

	const handleSave = async () => {
		if (!screen) return;
		setSaving(true);
		try {
			await Screens.update(screen);
			setSaved(true);
		} catch (err) {
			console.error("Failed to save screen:", err);
		} finally {
			setSaving(false);
		}
	};

	if (!selectedScreen) return null;

	return (
		<div className="fixed top-0 right-0 h-full w-96 bg-white shadow-2xl z-50 flex flex-col border-l border-gray-200 overflow-hidden">
			{/* Header */}
			<div className="flex items-center justify-between px-4 py-3 border-b border-gray-200 bg-gray-50">
				<h2 className="text-base font-semibold text-gray-800 truncate">
					{selectedScreen}
				</h2>
				<button
					onClick={() => setSelectedScreen("")}
					className="text-gray-500 hover:text-gray-800 text-xl leading-none"
					aria-label="Close panel"
				>
					×
				</button>
			</div>

			{/* Body */}
			<div className="flex-1 overflow-y-auto px-4 py-4 space-y-4">
				{!screen ? (
					<p className="text-sm text-gray-400">Loading…</p>
				) : (
					<>
						{/* Core fields */}
						{(
							[
								"name",
								"text",
								"default_next_screen",
								"service_code",
								"function",
								"input_identifier",
								"input_type",
							] as (keyof Screen)[]
						).map(field => {
							const val = screen[field];
							if (val === undefined && field !== "text" && field !== "default_next_screen")
								return null;
							return (
								<div key={field}>
									<label className="block text-xs font-medium text-gray-500 mb-1">
										{LABEL[field] ?? field}
									</label>
									<input
										className="w-full border border-gray-300 rounded px-2 py-1.5 text-sm text-gray-800 focus:outline-none focus:border-blue-400"
										value={(val as string) ?? ""}
										onChange={e => handleChange(field, e.target.value)}
										disabled={field === "name"}
									/>
								</div>
							);
						})}

						{/* Screen type (read-only badge) */}
						<div>
							<label className="block text-xs font-medium text-gray-500 mb-1">
								Screen Type
							</label>
							<span className="inline-block px-2 py-0.5 rounded text-xs font-semibold bg-gray-100 text-gray-700">
								{screen.screen_type}
							</span>
						</div>

						{/* Menu items (read-only summary) */}
						{menuItems.length > 0 && (
							<div>
								<label className="block text-xs font-medium text-gray-500 mb-1">
									Menu Items
								</label>
								<ul className="space-y-1">
									{menuItems.map(item => (
										<li
											key={item.id ?? item.option}
											className="text-xs bg-yellow-50 border border-yellow-200 rounded px-2 py-1"
										>
											<span className="font-semibold">{item.option}.</span>{" "}
											{item.display_name}{" "}
											<span className="text-gray-400">→ {item.next_screen}</span>
										</li>
									))}
								</ul>
							</div>
						)}

						{/* Router options (read-only summary) */}
						{routerOptions.length > 0 && (
							<div>
								<label className="block text-xs font-medium text-gray-500 mb-1">
									Router Options
								</label>
								<ul className="space-y-1">
									{routerOptions.map((opt, i) => (
										<li
											key={opt.id ?? i}
											className="text-xs bg-orange-50 border border-orange-200 rounded px-2 py-1"
										>
											<code className="font-mono">{opt.router_option}</code>{" "}
											<span className="text-gray-400">→ {opt.next_screen}</span>
										</li>
									))}
								</ul>
							</div>
						)}
					</>
				)}
			</div>

			{/* Footer */}
			{screen && (
				<div className="px-4 py-3 border-t border-gray-200 bg-gray-50 flex items-center gap-3">
					<button
						onClick={handleSave}
						disabled={saving}
						className="flex-1 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white text-sm font-medium rounded px-3 py-2 transition-colors"
					>
						{saving ? "Saving…" : "Save Changes"}
					</button>
					{saved && (
						<span className="text-xs text-green-600 font-medium">Saved ✓</span>
					)}
				</div>
			)}
		</div>
	);
}
