"use client";

import React, { createContext, useContext, useState } from "react";

interface NavigationContextType {
	selectedScreen: string | null;
	setSelectedScreen: (screen: string) => void;
}

const NavigationContext = createContext<NavigationContextType | undefined>(
	undefined
);

export const useNav = () => {
	const context = useContext(NavigationContext);
	if (!context) {
		throw new Error("useNavigation must be used within a NavigationProvider");
	}
	return context;
};

export const NavigationProvider = ({
	children,
}: {
	children: React.ReactNode;
}) => {
	const [selectedScreen, setSelectedScreen] = useState<string | null>(null);

	return (
		<NavigationContext.Provider value={{ selectedScreen, setSelectedScreen }}>
			{children}
		</NavigationContext.Provider>
	);
};
