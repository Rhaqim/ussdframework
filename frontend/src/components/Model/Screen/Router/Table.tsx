import React from "react";

import { RouterOption } from "@/types/screen.type";
import TableBase from "@/components/UI/Table";

const Table = ({ data }: { data: RouterOption[] }) => {
	return (
		<TableBase
			columns={[
				{ key: "id", title: "ID" },
        { key: "screen_name", title: "Screen" },
        { key: "router_option", title: "Router Option" },
        { key: "next_screen", title: "Next Screen" },
			]}
			data={data}
			onPress={() => {console.log("Pressed")}}
		/>
	);
};

export default Table;