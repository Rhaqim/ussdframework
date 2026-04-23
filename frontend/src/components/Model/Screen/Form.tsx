"use client";

import React, { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { Screens, Services, MenuItems, RouterOptions } from "@/api/route";
import Screen, { MenuItem, RouterOption, ScreenType } from "@/types/screen.type";
import Button from "@/components/UI/Button";

/* ─── Input style helpers ───────────────────────────────────────── */
const inputCls =
	"w-full bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-100 placeholder-slate-500 " +
	"focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500/30 transition-colors";
const labelCls = "block text-xs font-medium text-slate-400 mb-1";
const sectionCls = "bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-4";

/* ─── Section heading ───────────────────────────────────────────── */
const SectionTitle = ({ children }: { children: React.ReactNode }) => (
	<h3 className="text-sm font-semibold text-slate-200 border-b border-slate-800 pb-2 mb-4">
		{children}
	</h3>
);

/* ─── Empty row factories ───────────────────────────────────────── */
const emptyMenuItem = (screenName: string): MenuItem => ({
	screen_name: screenName,
	name: "",
	option: "",
	display_name: "",
	next_screen: "",
});

const emptyRouterOption = (screenName: string): RouterOption => ({
	screen_name: screenName,
	router_option: "",
	next_screen: "",
});

/* ─── Props ─────────────────────────────────────────────────────── */
interface ScreenFormProps {
	initialData?: Screen;
	initialMenuItems?: MenuItem[];
	initialRouterOptions?: RouterOption[];
	mode?: "create" | "edit";
}

/* ─── Component ─────────────────────────────────────────────────── */
export default function ScreenForm({
	initialData,
	initialMenuItems = [],
	initialRouterOptions = [],
	mode = "create",
}: ScreenFormProps) {
	const router = useRouter();

	/* Existing screen names (for select dropdowns) */
	const [allScreens, setAllScreens] = useState<string[]>([]);
	const [allServices, setAllServices] = useState<string[]>([]);

	useEffect(() => {
		Screens.getAll()
			.then((s: Screen[]) => setAllScreens(s.map(x => x.name)))
			.catch(() => {});
		Services.getAll()
			.then((s: any[]) => setAllServices(s.map(x => x.name)))
			.catch(() => {});
	}, []);

	/* Main screen state */
	const [form, setForm] = useState<Screen>(
		initialData ?? {
			name: "",
			text: "",
			screen_type: ScreenType.INITIAL,
			default_next_screen: "",
			service_code: "",
			input_identifier: "",
			input_type: "",
			function: "",
		}
	);

	/* Child entities */
	const [menuItems, setMenuItems] = useState<MenuItem[]>(
		initialMenuItems.length ? initialMenuItems : [emptyMenuItem(form.name)]
	);
	const [routerOptions, setRouterOptions] = useState<RouterOption[]>(
		initialRouterOptions.length ? initialRouterOptions : [emptyRouterOption(form.name)]
	);

	const [saving, setSaving] = useState(false);
	const [error, setError] = useState<string | null>(null);

	/* ── Helpers ── */
	const set = (field: keyof Screen, value: any) =>
		setForm(prev => ({ ...prev, [field]: value }));

	const isMenu = form.screen_type === ScreenType.MENU;
	const isInput = form.screen_type === ScreenType.INPUT;
	const isFunction = form.screen_type === ScreenType.FUNCTION;
	const isRouter = form.screen_type === ScreenType.ROUTER;

	/* ── Menu items ── */
	const addMenuItem = () =>
		setMenuItems(prev => [...prev, emptyMenuItem(form.name)]);

	const updateMenuItem = (i: number, field: keyof MenuItem, val: string) =>
		setMenuItems(prev =>
			prev.map((m, idx) => (idx === i ? { ...m, [field]: val } : m))
		);

	const removeMenuItem = (i: number) =>
		setMenuItems(prev => prev.filter((_, idx) => idx !== i));

	/* ── Router options ── */
	const addRouterOption = () =>
		setRouterOptions(prev => [...prev, emptyRouterOption(form.name)]);

	const updateRouterOption = (i: number, field: keyof RouterOption, val: string) =>
		setRouterOptions(prev =>
			prev.map((r, idx) => (idx === i ? { ...r, [field]: val } : r))
		);

	const removeRouterOption = (i: number) =>
		setRouterOptions(prev => prev.filter((_, idx) => idx !== i));

	/* ── Submit ── */
	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault();
		setError(null);
		setSaving(true);
		try {
			if (mode === "create") {
				await Screens.create(form);
				// Save child entities
				if (isMenu) {
					await Promise.all(
						menuItems
							.filter(m => m.option && m.display_name && m.next_screen)
							.map(m => MenuItems.create({ ...m, screen_name: form.name }))
					);
				}
				if (isRouter) {
					await Promise.all(
						routerOptions
							.filter(r => r.router_option && r.next_screen)
							.map(r => RouterOptions.create({ ...r, screen_name: form.name }))
					);
				}
			} else {
				await Screens.update(form);
			}
			router.push("/admin/screens");
		} catch (err: any) {
			setError(err?.message ?? "An error occurred");
		} finally {
			setSaving(false);
		}
	};

	/* ── Screen name select component ── */
	const ScreenSelect = ({
		value,
		onChange,
		placeholder,
	}: {
		value: string;
		onChange: (v: string) => void;
		placeholder?: string;
	}) => (
		<select
			value={value}
			onChange={e => onChange(e.target.value)}
			className={inputCls}
		>
			<option value="">{placeholder ?? "Select screen…"}</option>
			{allScreens.map(s => (
				<option key={s} value={s}>
					{s}
				</option>
			))}
		</select>
	);

	return (
		<form onSubmit={handleSubmit} className="space-y-6 max-w-2xl">
			{/* Basic info */}
			<div className={sectionCls}>
				<SectionTitle>Basic Information</SectionTitle>
				<div className="grid grid-cols-2 gap-4">
					<div className="col-span-2">
						<label className={labelCls}>Screen Name</label>
						<input
							type="text"
							value={form.name}
							onChange={e => set("name", e.target.value)}
							placeholder="e.g. MainMenu"
							className={inputCls}
							disabled={mode === "edit"}
							required
						/>
					</div>
					<div className="col-span-2">
						<label className={labelCls}>Display Text</label>
						<textarea
							value={form.text}
							onChange={e => set("text", e.target.value)}
							rows={3}
							placeholder="Text displayed to the user…"
							className={`${inputCls} resize-none`}
							required
						/>
					</div>
					<div>
						<label className={labelCls}>Screen Type</label>
						<select
							value={form.screen_type}
							onChange={e => set("screen_type", e.target.value as ScreenType)}
							className={inputCls}
							required
						>
							{Object.values(ScreenType).map(t => (
								<option key={t} value={t}>
									{t}
								</option>
							))}
						</select>
					</div>
					<div>
						<label className={labelCls}>Service Code</label>
						<input
							type="text"
							value={form.service_code ?? ""}
							onChange={e => set("service_code", e.target.value)}
							placeholder="e.g. *123#"
							className={inputCls}
						/>
					</div>
					{!isRouter && (
						<div className="col-span-2">
							<label className={labelCls}>Default Next Screen</label>
							<ScreenSelect
								value={form.default_next_screen}
								onChange={v => set("default_next_screen", v)}
								placeholder="Select next screen…"
							/>
						</div>
					)}
				</div>
			</div>

			{/* Menu type */}
			{isMenu && (
				<div className={sectionCls}>
					<SectionTitle>Menu Items</SectionTitle>
					<div className="space-y-3">
						{menuItems.map((item, i) => (
							<div
								key={i}
								className="bg-slate-800/60 border border-slate-700 rounded-lg p-4 space-y-3"
							>
								<div className="flex items-center justify-between mb-1">
									<span className="text-xs font-medium text-slate-400">
										Item {i + 1}
									</span>
									{menuItems.length > 1 && (
										<button
											type="button"
											onClick={() => removeMenuItem(i)}
											className="text-xs text-red-400 hover:text-red-300 transition-colors"
										>
											Remove
										</button>
									)}
								</div>
								<div className="grid grid-cols-2 gap-3">
									<div>
										<label className={labelCls}>Option #</label>
										<input
											type="text"
											value={item.option}
											onChange={e => updateMenuItem(i, "option", e.target.value)}
											placeholder="1"
											className={inputCls}
										/>
									</div>
									<div>
										<label className={labelCls}>Display Name</label>
										<input
											type="text"
											value={item.display_name}
											onChange={e =>
												updateMenuItem(i, "display_name", e.target.value)
											}
											placeholder="Buy Airtime"
											className={inputCls}
										/>
									</div>
									<div>
										<label className={labelCls}>Internal Name</label>
										<input
											type="text"
											value={item.name}
											onChange={e => updateMenuItem(i, "name", e.target.value)}
											placeholder="buy_airtime"
											className={inputCls}
										/>
									</div>
									<div>
										<label className={labelCls}>Next Screen</label>
										<ScreenSelect
											value={item.next_screen}
											onChange={v => updateMenuItem(i, "next_screen", v)}
										/>
									</div>
								</div>
							</div>
						))}
					</div>
					<Button
						type="button"
						variant="secondary"
						size="sm"
						onClick={addMenuItem}
					>
						+ Add Menu Item
					</Button>
				</div>
			)}

			{/* Input type */}
			{isInput && (
				<div className={sectionCls}>
					<SectionTitle>Input Settings</SectionTitle>
					<div className="grid grid-cols-2 gap-4">
						<div>
							<label className={labelCls}>Input Identifier</label>
							<input
								type="text"
								value={form.input_identifier ?? ""}
								onChange={e => set("input_identifier", e.target.value)}
								placeholder="e.g. phone_number"
								className={inputCls}
							/>
						</div>
						<div>
							<label className={labelCls}>Input Type</label>
							<select
								value={form.input_type ?? ""}
								onChange={e => set("input_type", e.target.value)}
								className={inputCls}
							>
								<option value="">Select type…</option>
								<option value="string">String</option>
								<option value="number">Number</option>
								<option value="date">Date</option>
								<option value="phone">Phone</option>
							</select>
						</div>
						<div>
							<label className={labelCls}>Validation Regex</label>
							<input
								type="text"
								value={(form as any).validation_regex ?? ""}
								onChange={e => set("validation_regex" as any, e.target.value)}
								placeholder="e.g. ^\d{10}$"
								className={`${inputCls} font-mono text-xs`}
							/>
						</div>
						<div>
							<label className={labelCls}>Max Length</label>
							<input
								type="number"
								value={(form as any).max_length ?? ""}
								onChange={e =>
									set(
										"max_length" as any,
										e.target.value ? parseInt(e.target.value) : ""
									)
								}
								placeholder="e.g. 12"
								className={inputCls}
							/>
						</div>
					</div>
				</div>
			)}

			{/* Function type */}
			{isFunction && (
				<div className={sectionCls}>
					<SectionTitle>Function Settings</SectionTitle>
					<div>
						<label className={labelCls}>Service Function</label>
						<select
							value={form.function ?? ""}
							onChange={e => set("function", e.target.value)}
							className={inputCls}
						>
							<option value="">Select service…</option>
							{allServices.map(s => (
								<option key={s} value={s}>
									{s}
								</option>
							))}
						</select>
					</div>
				</div>
			)}

			{/* Router type */}
			{isRouter && (
				<div className={sectionCls}>
					<SectionTitle>Router Options</SectionTitle>
					<div className="space-y-3">
						{routerOptions.map((opt, i) => (
							<div
								key={i}
								className="bg-slate-800/60 border border-slate-700 rounded-lg p-4 space-y-3"
							>
								<div className="flex items-center justify-between mb-1">
									<span className="text-xs font-medium text-slate-400">
										Option {i + 1}
									</span>
									{routerOptions.length > 1 && (
										<button
											type="button"
											onClick={() => removeRouterOption(i)}
											className="text-xs text-red-400 hover:text-red-300 transition-colors"
										>
											Remove
										</button>
									)}
								</div>
								<div className="grid grid-cols-2 gap-3">
									<div>
										<label className={labelCls}>Condition</label>
										<input
											type="text"
											value={opt.router_option}
											onChange={e =>
												updateRouterOption(i, "router_option", e.target.value)
											}
											placeholder="e.g. input == '1'"
											className={`${inputCls} font-mono text-xs`}
										/>
									</div>
									<div>
										<label className={labelCls}>Next Screen</label>
										<ScreenSelect
											value={opt.next_screen}
											onChange={v => updateRouterOption(i, "next_screen", v)}
										/>
									</div>
								</div>
							</div>
						))}
					</div>
					<div className="flex gap-3 mt-2">
						<Button
							type="button"
							variant="secondary"
							size="sm"
							onClick={addRouterOption}
						>
							+ Add Router Option
						</Button>
					</div>
					<div className="pt-2">
						<label className={labelCls}>Fallback (default_next_screen)</label>
						<ScreenSelect
							value={form.default_next_screen}
							onChange={v => set("default_next_screen", v)}
							placeholder="Fallback screen…"
						/>
					</div>
				</div>
			)}

			{/* Error */}
			{error && (
				<div className="text-sm text-red-400 bg-red-600/10 border border-red-600/20 rounded-lg px-4 py-3">
					{error}
				</div>
			)}

			{/* Actions */}
			<div className="flex items-center gap-3">
				<Button type="submit" disabled={saving}>
					{saving ? "Saving…" : mode === "edit" ? "Save Changes" : "Create Screen"}
				</Button>
				<Button
					type="button"
					variant="ghost"
					onClick={() => router.push("/admin/screens")}
				>
					Cancel
				</Button>
			</div>
		</form>
	);
}
