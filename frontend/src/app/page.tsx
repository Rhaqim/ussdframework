"use client";

import ScreenForm from "@/components/Model/Screen/Form";

export default function Home() {
	return (
		<main className="bg-black flex min-h-screen flex-col items-center justify-between p-4">
			<div className="z-10 w-full items-center justify-between font-mono text-sm lg:flex">
				<ScreenForm />
			</div>
		</main>
	);
}
