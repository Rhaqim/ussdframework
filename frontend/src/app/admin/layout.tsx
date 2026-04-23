"use client";

import React from "react";
import Layout from "@/components/Layout";

const AdminLayout = ({ children }: { children: React.ReactNode }) => {
	return (
		<Layout>
			<div className="p-6 min-h-full">{children}</div>
		</Layout>
	);
};

export default AdminLayout;
