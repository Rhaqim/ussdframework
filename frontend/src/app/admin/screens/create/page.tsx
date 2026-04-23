import React from "react";
import ScreenForm from "@/components/Model/Screen/Form";

export default function CreateScreenPage() {
	return (
		<div className="max-w-2xl">
			<h2 className="text-lg font-semibold text-slate-100 mb-6">Create Screen</h2>
			<ScreenForm mode="create" />
		</div>
	);
}
