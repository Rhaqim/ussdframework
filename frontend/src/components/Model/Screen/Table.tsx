import React from "react";

import Screen from "@/types/screen.type";
import TableBase from "@/components/UI/Table";
import { useRouter } from "next/navigation";

const Table = ({ data }: { data: Screen[] }) => {
	const router = useRouter();

	return (
		<TableBase
			columns={[
				{ key: "id", title: "ID" },
				{ key: "name", title: "Name" },
				{ key: "text", title: "Text" },
				{ key: "screen_type", title: "Screen Type" },
				{ key: "default_next_screen", title: "Default Next Screen" },
				{ key: "service_code", title: "Service Code" },
				{ key: "function", title: "Function" },
				{ key: "input_identifier", title: "Input Identifier" },
				{ key: "input_type", title: "Input Type" },
			]}
			data={data}
			onPress={name => router.push(`/admin/screens/${name}`)}
		/>
	);
};

export default Table;
