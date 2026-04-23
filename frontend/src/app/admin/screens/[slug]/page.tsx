"use client";

import React, { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import { Screens, MenuItems, RouterOptions } from "@/api/route";
import type Screen from "@/types/screen.type";
import { MenuItem, RouterOption, ScreenType } from "@/types/screen.type";
import ScreenForm from "@/components/Model/Screen/Form";

export default function EditScreenPage() {
	const params = useParams();
	const slug = decodeURIComponent(params.slug as string);

	const [screen, setScreen] = useState<Screen | null>(null);
	const [menuItems, setMenuItems] = useState<MenuItem[]>([]);
	const [routerOptions, setRouterOptions] = useState<RouterOption[]>([]);
	const [loading, setLoading] = useState(true);
	const [notFound, setNotFound] = useState(false);

	useEffect(() => {
		const load = async () => {
			try {
				const s: Screen = await Screens.get(slug);
				setScreen(s);
				if (s.screen_type === ScreenType.MENU) {
					const mi = await MenuItems.getByQuery({ ScreenName: slug }).catch(() => []);
					setMenuItems(mi);
				}
				if (s.screen_type === ScreenType.ROUTER) {
					const ro = await RouterOptions.getByQuery({ ScreenName: slug }).catch(() => []);
					setRouterOptions(ro);
				}
			} catch {
				setNotFound(true);
			} finally {
				setLoading(false);
			}
		};
		load();
	}, [slug]);

	if (loading) {
		return (
			<div className="text-sm text-slate-500 py-12 text-center">Loading…</div>
		);
	}
	if (notFound || !screen) {
		return (
			<div className="text-sm text-red-400 py-12 text-center">
				Screen &ldquo;{slug}&rdquo; not found.
			</div>
		);
	}

	return (
		<div className="max-w-2xl">
			<h2 className="text-lg font-semibold text-slate-100 mb-1">{screen.name}</h2>
			<p className="text-sm text-slate-500 mb-6">Edit screen configuration</p>
			<ScreenForm
				initialData={screen}
				initialMenuItems={menuItems}
				initialRouterOptions={routerOptions}
				mode="edit"
			/>
		</div>
	);
}

