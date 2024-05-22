'use client';

import React from "react";

import { downloadFile, uploadFile } from "@/api/route";

export default function ExportPage() {
	const [file, setFile] = React.useState<File | null>(null);

	const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
		const files = e.target.files;
		if (files) {
			setFile(files[0]);
		}
	};

	const handleUpload = async () => {
		if (file) {
			await uploadFile(file);
		}
	};

	const handleDownload = async () => {
		await downloadFile();
	};

	return (
		<div className="flex flex-row items-center justify-center">
			<div className="flex flex-col items-center justify-center border border-gray-300 rounded-lg p-4">
				<input type="file" onChange={handleFileChange} />
				<button onClick={handleUpload}>Upload</button>
			</div>
			<div className="flex flex-col items-center justify-center border border-gray-300 rounded-lg p-4">
				<button onClick={handleDownload}>Download</button>
			</div>
		</div>
	);
}
