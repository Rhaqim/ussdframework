import React from "react";
import Link from "next/link";

const Header = () => {
	return (
		<header className="bg-gray-900 text-white p-4">
			<Link href="/">
				<h1 className="text-lg font-bold">Admin Portal</h1>
			</Link>
		</header>
	);
};

export default Header;
