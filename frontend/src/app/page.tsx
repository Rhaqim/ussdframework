'use client';

import Form from "@/components/UI/Form";
import Link from "next/link";

export default function Home() {
	const fields = [
		{
			name: "name",
			label: "Name",
			type: "text",
			required: true,
		},
		{
			name: "email",
			label: "Email",
			type: "email",
			required: true,
		},
		{
			name: "password",
			label: "Password",
			type: "password",
			required: true,
		},
		{
			name: "dropdown",
			label: "Dropdown",
			type: "dropdown",
			options: [
				{ value: "1", label: "Option 1" },
				{ value: "2", label: "Option 2" },
				{ value: "3", label: "Option 3" },
			],
		},
	];

	const onSubmit = (data: any) => {
		console.log(data);
	};

	return (
		<main className="bg-black flex min-h-screen flex-col items-center justify-between p-4">
			<div className="z-10 w-full items-center justify-between font-mono text-sm lg:flex">
				<Form fields={fields} onSubmit={onSubmit} />
				{/* <Link href="/admin">
          <p className="text-blue-500">Admin</p>
        </Link>
        <Link href="/user">
          <p className="text-blue-500">User</p>
        </Link> */}
			</div>
		</main>
	);
}
