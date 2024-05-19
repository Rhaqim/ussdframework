"use client";

import React from "react";

const AdminLayout = ({ children }: { children: React.ReactNode }) => {
	return (
		<div className="flex-1 rounded-sm bg-gray-600 p-4 min-h-screen">
			{children}
		</div>
	);
};

export default AdminLayout;
