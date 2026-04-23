"use client";

import React, { useState } from "react";
import { downloadFile, uploadFile, Screens, Services, MenuItems, RouterOptions } from "@/api/route";
import Button from "@/components/UI/Button";

/* ─── Build USSDMenu JSON from live data ─────────────────────────── */
async function buildMenuJson() {
	const [screens, services, menuItemsList, routerOptionsList] = await Promise.all([
		Screens.getAll().catch(() => []),
		Services.getAll().catch(() => []),
		MenuItems.getAll().catch(() => []),
		RouterOptions.getAll().catch(() => []),
	]);

	const menus: Record<string, any> = {};
	for (const s of screens) {
		const items = menuItemsList
			.filter((m: any) => m.screen_name === s.name)
			.reduce((acc: any, m: any) => {
				acc[m.option] = {
					display_name: m.display_name,
					next_screen: m.next_screen,
				};
				return acc;
			}, {});
		const routers = routerOptionsList
			.filter((r: any) => r.screen_name === s.name)
			.reduce((acc: any, r: any) => {
				acc[r.router_option] = r.next_screen;
				return acc;
			}, {});

		menus[s.name] = {
			text: s.text,
			screen_type: s.screen_type,
			default_next_screen: s.default_next_screen,
			...(s.service_code ? { service_code: s.service_code } : {}),
			...(Object.keys(items).length ? { menu_items: items } : {}),
			...(s.function ? { function: s.function } : {}),
			...(Object.keys(routers).length ? { router_options: routers } : {}),
			...(s.input_identifier ? { input_identifier: s.input_identifier } : {}),
			...(s.input_type ? { input_type: s.input_type } : {}),
		};
	}

	const svcMap: Record<string, any> = {};
	for (const sv of services) {
		svcMap[sv.name] = {
			function_name: sv.function_name,
			...(sv.function_url ? { function_url: sv.function_url } : {}),
			data_key: sv.data_key,
			...(sv.service_code ? { service_code: sv.service_code } : {}),
		};
	}

	return { menus, services: svcMap };
}

export default function ExportPage() {
	const [importFile, setImportFile] = useState<File | null>(null);
	const [importPreview, setImportPreview] = useState<string | null>(null);
	const [exportPreview, setExportPreview] = useState<string | null>(null);
	const [loadingExport, setLoadingExport] = useState(false);
	const [loadingImport, setLoadingImport] = useState(false);
	const [message, setMessage] = useState<{ text: string; type: "ok" | "err" } | null>(null);

	/* ── Export ── */
	const handlePreview = async () => {
		setLoadingExport(true);
		setMessage(null);
		try {
			const json = await buildMenuJson();
			setExportPreview(JSON.stringify(json, null, 2));
		} catch {
			setMessage({ text: "Failed to build preview.", type: "err" });
		} finally {
			setLoadingExport(false);
		}
	};

	const handleDownload = async () => {
		setMessage(null);
		try {
			await downloadFile();
			setMessage({ text: "Downloaded successfully.", type: "ok" });
		} catch {
			setMessage({ text: "Download failed.", type: "err" });
		}
	};

	/* ── Import ── */
	const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		const f = e.target.files?.[0] ?? null;
		setImportFile(f);
		setImportPreview(null);
		if (f) {
			const reader = new FileReader();
			reader.onload = ev => setImportPreview(ev.target?.result as string ?? null);
			reader.readAsText(f);
		}
	};

	const handleUpload = async () => {
		if (!importFile) return;
		setLoadingImport(true);
		setMessage(null);
		try {
			await uploadFile(importFile);
			setMessage({ text: "Import successful.", type: "ok" });
			setImportFile(null);
			setImportPreview(null);
		} catch {
			setMessage({ text: "Import failed.", type: "err" });
		} finally {
			setLoadingImport(false);
		}
	};

	return (
		<div className="max-w-5xl space-y-6">
			{/* Status message */}
			{message && (
				<div
					className={`text-sm px-4 py-3 rounded-lg border ${
						message.type === "ok"
							? "bg-emerald-600/10 border-emerald-600/20 text-emerald-400"
							: "bg-red-600/10 border-red-600/20 text-red-400"
					}`}
				>
					{message.text}
				</div>
			)}

			<div className="grid grid-cols-1 md:grid-cols-2 gap-6">
				{/* ── Export column ── */}
				<div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-4">
					<div>
						<h2 className="text-sm font-semibold text-slate-200 mb-1">Export</h2>
						<p className="text-xs text-slate-500">
							Download the current USSD flow as a JSON file compatible with the backend.
						</p>
					</div>
					<div className="flex gap-3">
						<Button variant="secondary" size="sm" onClick={handlePreview} disabled={loadingExport}>
							{loadingExport ? "Loading…" : "Preview JSON"}
						</Button>
						<Button size="sm" onClick={handleDownload}>
							Download JSON
						</Button>
					</div>
					{exportPreview && (
						<pre className="bg-slate-950 border border-slate-800 rounded-lg p-4 text-xs text-slate-300 overflow-auto max-h-96 font-mono whitespace-pre">
							{exportPreview}
						</pre>
					)}
				</div>

				{/* ── Import column ── */}
				<div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-4">
					<div>
						<h2 className="text-sm font-semibold text-slate-200 mb-1">Import</h2>
						<p className="text-xs text-slate-500">
							Upload a JSON file to import screens, services, and relationships.
						</p>
					</div>
					<label className="flex flex-col items-center justify-center border-2 border-dashed border-slate-700 rounded-xl p-6 cursor-pointer hover:border-indigo-600/50 transition-colors">
						<svg className="w-8 h-8 text-slate-600 mb-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
							<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
						</svg>
						<span className="text-sm text-slate-400">
							{importFile ? importFile.name : "Click to select a JSON file"}
						</span>
						<span className="text-xs text-slate-600 mt-1">.json files only</span>
						<input
							type="file"
							accept=".json,application/json"
							onChange={handleFileChange}
							className="hidden"
						/>
					</label>
					{importFile && (
						<Button size="sm" onClick={handleUpload} disabled={loadingImport}>
							{loadingImport ? "Importing…" : "Import File"}
						</Button>
					)}
					{importPreview && (
						<pre className="bg-slate-950 border border-slate-800 rounded-lg p-4 text-xs text-slate-300 overflow-auto max-h-72 font-mono whitespace-pre">
							{importPreview}
						</pre>
					)}
				</div>
			</div>
		</div>
	);
}

