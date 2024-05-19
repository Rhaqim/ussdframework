"use client";

import React from "react";

import Layout from "@/components/Layout";

const AdminLayout = ({ children }: { children: React.ReactNode }) => {
	return (
		<Layout>
			<div className="flex-1 rounded-sm bg-gray-600 p-4 min-h-screen">
				{children}
			</div>
		</Layout>
	);
};

export default AdminLayout;
