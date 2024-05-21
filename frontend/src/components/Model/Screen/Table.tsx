import React from "react";

import Screen from "@/types/screen.type";
import TableBase from "@/components/UI/Table";

const Table = ({ data }: { data: Screen[] }) => {
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
			onPress={() => {console.log("Pressed")}}
		/>
	);
};

export default Table;
